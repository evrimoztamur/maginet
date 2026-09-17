"""Reproduce frozen candidate screens and paired followups; Easy players are never run.
Candidate codes and original seed namespaces are recorded in candidates.json.
Use a fresh --output-root after rebuilding against a changed campaign catalogue.
"""
import argparse
import json
from pathlib import Path
import subprocess

ROOT = Path('assessments/campaign-challenge')

def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('stage', choices=['screen', 'followup', 'support'])
    parser.add_argument('--id')
    parser.add_argument('--workers', type=int, default=4)
    parser.add_argument('--output-root', type=Path, default=ROOT)
    args = parser.parse_args()
    entries = json.loads((ROOT / 'candidates.json').read_text())
    selected = {e['battle'] for e in entries if e.get('selected')}
    for entry in entries:
        if args.id and not entry['id'].startswith(args.id):
            continue
        if args.stage != 'screen' and not (entry.get('selected') or entry['baseline'] and entry['battle'] in selected):
            continue
        # The report verifies and reuses the unchanged engine's archived original NN/HH screens.
        if args.stage == 'support' and entry['baseline']:
            continue
        pairs = [('normal', 'normal'), ('hard', 'hard')] if args.stage == 'support' else [('hard', 'normal')]
        for red, blue in pairs:
            name = entry['id'] + f'-{red}-{blue}' if args.stage == 'support' else entry['id']
            subprocess.run([
                'target/release/generate', 'analyse', '--code', entry['code'],
                '--seed-namespace', entry['seed_namespace'], '--games', '300' if args.stage == 'followup' else '30',
                '--seed', '1', '--max-plies', '200', '--red-profile', red, '--blue-profile', blue,
                '--workers', str(args.workers), '--replays', '--output', str(args.output_root / args.stage / name),
            ], check=True)

if __name__ == '__main__':
    main()
