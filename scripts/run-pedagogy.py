"""Screen teaching candidates under the frozen, combined draw rules and AI."""
import argparse
import json
from pathlib import Path
import subprocess

parser=argparse.ArgumentParser()
parser.add_argument('stage', choices=['screen','followup'])
parser.add_argument('--root',type=Path,default=Path('assessments/campaign-pedagogy'))
parser.add_argument('--binary',type=Path,default=Path('target/draw-experiment/all/generate'))
parser.add_argument('--workers',type=int,default=1)
parser.add_argument('--id',action='append')
args=parser.parse_args()
entries=json.loads((args.root/'candidates.json').read_text())
selected_battles={entry['battle'] for entry in entries if entry.get('selected')}
for entry in entries:
    if args.id and entry['id'] not in args.id: continue
    if args.stage=='followup' and not (entry.get('selected') or entry['baseline'] and entry['battle'] in selected_battles): continue
    subprocess.run([str(args.binary.resolve()),'analyse','--code',entry['code'],
                    '--seed-namespace',entry['seed_namespace'],'--games','30' if args.stage=='screen' else '300',
                    '--seed','1','--max-plies','200','--red-profile','normal','--blue-profile','normal',
                    '--workers',str(args.workers),'--replays','--output',str(args.root/args.stage/entry['id'])],check=True)
