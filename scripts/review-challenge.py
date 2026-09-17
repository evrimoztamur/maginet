"""Review first wins/losses for every screened candidate; deepen finalists and use their 300-trial replays."""
import argparse
from concurrent.futures import ThreadPoolExecutor
import hashlib
import json
from pathlib import Path
import subprocess

ROOT = Path('assessments/campaign-challenge')

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--workers',type=int,default=2);parser.add_argument('--refresh',action='store_true');args=parser.parse_args()
    entries=json.loads((ROOT/'candidates.json').read_text());out=ROOT/'decisions';out.mkdir(exist_ok=True)
    def run(entry):
        stage='followup' if entry.get('selected') and (ROOT/'followup'/entry['id']/'matchup-00-2-1.json').exists() else 'screen'
        replay=ROOT/stage/entry['id']/'matchup-00-2-1.json'
        if not replay.exists():raise RuntimeError(f'Missing screen: {entry["id"]}')
        requested=dict(id=entry['id'],code=entry['code'],replay=str(replay),diagnostic_nodes=500000 if entry.get('selected') else 20000)
        result=out/(entry['id']+'.json');inputs=out/(entry['id']+'.input.json')
        if not args.refresh and result.exists() and inputs.exists():
            try:
                previous=json.loads(inputs.read_text())[0];report=json.loads(result.read_text())[0]
                same_scenario = all(previous.get(k) == requested[k] for k in ['id', 'code'])
                same_replay = previous.get('replay') == requested['replay']
                # Retain stronger finalist reviews when a candidate is subsequently rejected.
                if same_scenario and (same_replay or not entry.get('selected')) and report['config']['nodes'] >= requested['diagnostic_nodes']:
                    return
            except (ValueError,KeyError,IndexError):pass
        temporary=result.with_suffix('.tmp');temporary_input=inputs.with_suffix('.tmp')
        temporary_input.write_text(json.dumps([requested],indent=2)+'\n')
        with temporary.open('w') as stream:
            subprocess.run(['target/release/examples/pedagogy',str(temporary_input)],stdout=stream,check=True)
        json.loads(temporary.read_text()) # Never publish incomplete diagnostics.
        temporary.replace(result);temporary_input.replace(inputs)
        print(entry['id'],flush=True)
    with ThreadPoolExecutor(max_workers=args.workers) as pool:list(pool.map(run,entries))
    sources=['generate/examples/pedagogy.rs','shared/src/logic/search.rs','shared/src/logic/game.rs','shared/src/logic/deadlock.rs']
    (ROOT/'decision-sources.json').write_text(json.dumps({p:hashlib.sha256(Path(p).read_bytes()).hexdigest() for p in sources},indent=2)+'\n')

if __name__=='__main__':main()
