"""Run paired campaign revisions into separate, resumable datasets."""
import argparse,json,subprocess
from pathlib import Path
p=argparse.ArgumentParser();p.add_argument('stage',choices=['screen','original','final','junction-screen','junction-final']);p.add_argument('--output-root',type=Path,default=Path('assessments/campaign-revision'));args=p.parse_args()
root=Path('assessments/campaign-revision')
alphabet='0123456789abcdefghjkmnpqrstvwxyz'
def canonical(code):
 bits=''.join(f'{alphabet.index(c):05b}' for c in code)
 data=bytes(int(bits[i:i+8],2) for i in range(0,len(bits)-7,8));start=3+3*data[1]
 data=data[:start]+b''.join(sorted(data[i:i+2] for i in range(start,len(data),2)))
 bits=''.join(f'{b:08b}' for b in data);bits+='0'*(-len(bits)%5)
 return ''.join(alphabet[int(bits[i:i+5],2)] for i in range(0,len(bits),5))
junctions=args.stage.startswith('junction-')
for entry in json.loads((root/('junctions.json' if junctions else 'scenarios.json')).read_text()):
 name=entry['name'].lower().replace(' ','-')
 code=entry['code'] if junctions else entry['original' if args.stage=='original' else 'revised']
 cmd=['target/release/generate','analyse','--code',code,'--seed-namespace',canonical(code if junctions else entry['original']),'--replays','--output',str(args.output_root/args.stage/name)]
 if args.stage not in ['screen','junction-screen']:cmd+=['--games','300','--red-profile','normal','--blue-profile','normal']
 subprocess.run(cmd,check=True)
