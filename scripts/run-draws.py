"""Reproduce the three-arm inactivity experiment using frozen per-arm binaries."""
import argparse
import json
from pathlib import Path
import subprocess

parser = argparse.ArgumentParser()
parser.add_argument('stage', choices=['screen', 'followup'])
parser.add_argument('--arms', nargs='+', choices=['control', 'rules', 'all'], default=['rules', 'all'])
parser.add_argument('--root', type=Path, default=Path('assessments/campaign-draws'))
parser.add_argument('--bin-root', type=Path, default=Path('target/draw-experiment'))
parser.add_argument('--workers', type=int, default=4)
parser.add_argument('--id', action='append')
args = parser.parse_args()
experiment = json.loads((args.root/'experiment.json').read_text())
control = args.root/experiment['control_dataset']
meta = json.loads((control/'metadata.json').read_text())
selected = args.id or experiment['followup_ids']
for arm in args.arms:
    binary = (args.bin_root/arm/'generate').resolve()
    if args.stage == 'screen':
        if arm == 'control':
            print('Control screen is reused unchanged from', control)
            continue
        jobs = [('campaign', [], args.root/arm, 30)]
    else:
        jobs = [('analyse', ['--code', entry['code'], '--red-profile', 'normal', '--blue-profile', 'normal'],
                 args.root/'followup'/entry['id']/arm, 300)
                for entry in meta['catalogue'] if entry['id'] in selected]
    for mode, options, output, games in jobs:
        subprocess.run([str(binary), mode, *options, '--games', str(games), '--seed', '1',
                        '--max-plies', '200', '--workers', str(args.workers), '--replays',
                        '--output', str(output)], check=True)
