"""Reproduce the paired screen or selected 300-trial Normal/Normal followups."""
import argparse
import json
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('stage', choices=['screen', 'followup'])
parser.add_argument('--before-bin', type=Path, required=True)
parser.add_argument('--after-bin', type=Path, default=Path('target/release/generate'))
parser.add_argument('--root', type=Path, default=Path('assessments/campaign-overcharge'))
parser.add_argument('--id', action='append')
args = parser.parse_args()
if args.stage == 'screen':
    for arm, binary in [('before', args.before_bin), ('after', args.after_bin)]:
        subprocess.run([str(binary.resolve()), 'campaign', '--games', '30', '--seed', '1', '--max-plies', '200', '--replays', '--output', str(args.root / arm)], check=True)
else:
    meta = json.loads((args.root/'before/metadata.json').read_text())
    selected = args.id or json.loads((args.root/'comparison.json').read_text())['affected_ids']
    for entry in meta['catalogue']:
        if entry['id'] not in selected: continue
        for arm, binary in [('before', args.before_bin), ('after', args.after_bin)]:
            subprocess.run([str(binary.resolve()), 'analyse', '--code', entry['code'], '--games', '300', '--seed', '1', '--max-plies', '200', '--red-profile', 'normal', '--blue-profile', 'normal', '--replays', '--output', str(args.root/'followup'/entry['id']/arm)], check=True)
