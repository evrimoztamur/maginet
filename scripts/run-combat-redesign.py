"""Reproduce candidate screens/followups with the preceding assessment's trial namespaces."""
import argparse
import base64
import json
import subprocess
from pathlib import Path

ROOT = Path('assessments/campaign-combat-redesign')

def encode(width, height, mages, props):
    data = [((width-1)<<5)|((height-1)<<2), len(mages)]
    for x, y, team, kind, mana in mages:
        data += [(x<<5)|(y<<2)|team, kind, (mana<<4)|max(4,mana)]
    data += [len(props)]
    for x,y,kind in props:
        data += [(x<<5)|(y<<2),kind]
    return base64.b32encode(bytes(data)).decode().rstrip('=').translate(str.maketrans('ABCDEFGHIJKLMNOPQRSTUVWXYZ234567','0123456789abcdefghjkmnpqrstvwxyz'))

if __name__ == '__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('stage',choices=['screen','final'])
    parser.add_argument('--candidate')
    parser.add_argument('--output-root',type=Path,default=ROOT)
    args=parser.parse_args()
    for entry in json.loads((ROOT/'candidates.json').read_text()):
        if args.candidate and entry['id']!=args.candidate: continue
        if args.stage=='final' and not entry.get('selected'): continue
        meta=json.loads((Path('assessments/campaign-revision/final')/entry['battle']/'metadata.json').read_text())
        cmd=['target/release/generate','analyse','--code',entry['code'],'--seed-namespace',meta['config']['seed_namespace'],'--replays','--output',str(args.output_root/args.stage/entry['id'])]
        if args.stage=='final': cmd+=['--games','300','--red-profile','normal','--blue-profile','normal']
        subprocess.run(cmd,check=True)
