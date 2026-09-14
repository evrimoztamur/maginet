"""Combine focused followups with explicitly attributed unchanged baseline matchups."""
import collections,json,math
from pathlib import Path
root=Path('assessments/campaign-revision');baseline=Path('assessments/campaign-seed-1')
read=lambda p:json.loads(p.read_text())
junctions={e['name']:e for e in read(root/'junctions.json')}
graph=read(root/'graph.json');changes={e['name']:e for e in read(root/'scenarios.json')};old=read(baseline/'metadata.json');base_stats=read(baseline/'aggregate.json')
def games(stage,name,r=1,b=1):return read(root/stage/name.lower().replace(' ','-')/f'matchup-00-{r}-{b}.json')['games']
def stats(stage,name,r=1,b=1):return next(s['stats'] for s in read(root/stage/name.lower().replace(' ','-')/'aggregate.json') if s['red']==r and s['blue']==b)
def n(s):return sum(s[k] for k in ['wins','losses','draws','unresolved'])
def cell(s):
 p=s['resolved_win_rate'];ci=s['wilson95'];return f"{p*100:.1f}% [{ci[0]*100:.1f}, {ci[1]*100:.1f}]; n={n(s)}; U={s['unresolved']}; range {s['overall_win_range'][0]*100:.1f}–{s['overall_win_range'][1]*100:.1f}%" if p is not None else f"n={n(s)}; unresolved"
def counts(gs):return '/'.join(str(sum(g['outcome']==o for g in gs)) for o in ['Win','Loss','Draw','SafetyLimit'])
combined=[];md=['# Revised campaign assessment','', 'Simulated-agent evidence; human difficulty remains unvalidated. Red is the player. All runs retain the original profiles, seed 1 and 200-ply cap. Intervals are nominal 95% Wilson intervals; draws are resolved non-wins. Ranges bound unresolved outcomes.','', 'Eight revised scenarios and eight new 1v1 junction battles have 30 trials in all nine matchups, then 300 Normal/Normal trials. The first 30 final trials overlap the screen and are not independent replications. The new junctions have no original counterparts. Original versions of the eight revised scenarios have matching 300-trial Normal/Normal followups. Every untouched matchup reuses the immutable initial survey (30 trials); no untouched scenario is rerun.','', 'The [map](map.svg) shows the full layout. The [graph](graph.json) is exported from the shared UI/analyser catalogue. The [scenario manifest](scenarios.json) records original and revised codes. Each revised run explicitly uses the original canonical code as its seed namespace. Individual dataset metadata records graph, scenario, configuration, namespace, replay policy, and engine fingerprint. The eight earlier scenario datasets retain their run-time layout metadata; their simulations are reused after the cardinal-layout change because positions and connections do not affect battle simulation. The exported graph is the current progression authority. Every one of its 40 links joins cardinal-neighbour battle cells, including eight new 1v1 junctions, for 36 portals in total.','', '## Paired Normal/Normal comparisons','', '| Battle | Original W/L/D/U | Revised W/L/D/U | Original win rate [95% CI] | Revised win rate [95% CI] | Win change | Paired gained/lost wins |','|---|---|---|---|---|---:|---|']
for name in changes:
 a,b=games('original',name),games('final',name);assert len(a)==len(b)==300
 assert [g['seed'] for g in a]==[g['seed'] for g in b]
 # Existing baseline trials reproduce despite added replay/termination instrumentation.
 idx=next(i for i,e in enumerate(old['catalogue']) if e['name']==name)
 before=read(baseline/f'matchup-{idx:02}-1-1.json')['games']
 assert [{k:g[k] for k in before[0]} for g in a[:30]]==before
 assert b[:30]==games('screen',name)
 sa,sb=stats('original',name),stats('final',name)
 gained=sum(x['outcome']!='Win' and y['outcome']=='Win' for x,y in zip(a,b));lost=sum(x['outcome']=='Win' and y['outcome']!='Win' for x,y in zip(a,b))
 md.append(f"| {name} | {counts(a)} | {counts(b)} | {cell(sa)} | {cell(sb)} | {(sb['resolved_win_rate']-sa['resolved_win_rate'])*100:+.1f} pp | {gained}/{lost} |")
md += ['', 'Paired counts describe shared trial namespaces, not identical move trajectories after the scenario changes. The marginal Wilson intervals are not confidence intervals for the paired difference. No fixed win-rate target determines acceptance.','', '## Full revised matrices','', 'Each cell reports its own actual sample count. Normal/Normal uses 300 trials on revised battles; their other cells use 30. Untouched battles and tutorial reuse 30-trial baseline cells. Tutorial opponents remain Easy with rule stalemates disabled.']
for name in junctions:
 assert games('junction-final',name)[:30]==games('junction-screen',name)
lookup={}
for i,e in enumerate(graph['catalogue']):
 name=e['name'];md += ['',f"### {name}",'', '| Player / opponent | Easy | Normal | Hard |','|---|---|---|---|']
 for r in range(3):
  row=[]
  for b in range(3):
   if e['tutorial'] and b:row.append('—');continue
   if name in changes or name in junctions:
    stage=('junction-' if name in junctions else '')+('final' if r==b==1 else 'screen');s=stats(stage,name,r,b);source=f"{stage}/{e['id']}/matchup-00-{r}-{b}.json"
   else:
    idx=next(j for j,a in enumerate(old['catalogue']) if a['name']==name);s=next(a['stats'] for a in base_stats if a['level']==idx and a['red']==r and a['blue']==b);source=f'../campaign-seed-1/matchup-{idx:02}-{r}-{b}.json'
   lookup[e['id'],r,b]=s;combined.append(dict(id=e['id'],red=r,blue=b,source=source,reused_baseline=name not in changes and name not in junctions,stats=s));row.append(f'[{cell(s)}]({source})')
  md.append('| '+['Easy','Normal','Hard'][r]+' | '+' | '.join(row)+' |')
# Directed distances, independently traversing the exported shared graph.
adj=collections.defaultdict(list)
for edge in graph['connections']:
 adj[edge['from']].append(edge['to'])
 if not edge['one_way']:adj[edge['to']].append(edge['from'])
dist={'tutorial':0};q=collections.deque(['tutorial'])
while q:
 for dest in adj[q.popleft()]:
  if dest not in dist:dist[dest]=min(dist[src]+1 for src in dist if dest in adj[src]);q.append(dest)
assert len(dist)==len(graph['catalogue'])
names={e['id']:e['name'] for e in graph['catalogue']}
for title,routes in [('Main-route progression',[graph['main_route']]),('Optional routes and shortcuts',graph['optional_routes'])]:
 md+=['',f'## {title}','','Normal/Normal ordered-edge comparisons. A ≥20-point drop is a supported exploratory candidate only with disjoint intervals and ≤10% unresolved at both ends. No multiple-comparison correction; optional difficulty is not a core-route failure. Distances use directed shortest paths.','','| Edge | Distances | Red win drop | Flag |','|---|---|---:|---|']
 for route in routes:
  for a,b in zip(route,route[1:]):
   if (a,1,1) not in lookup:continue
   sa,sb=lookup[a,1,1],lookup[b,1,1];drop=(sa['resolved_win_rate']-sb['resolved_win_rate'])*100
   supported=sa['wilson95'][0]>sb['wilson95'][1] and sa['unresolved']*10<=n(sa) and sb['unresolved']*10<=n(sb)
   flag=('supported candidate' if supported else 'followup candidate') if drop>=20-1e-8 else 'below spike threshold'
   md.append(f'| {names[a]} → {names[b]} | {dist[a]} → {dist[b]} | {drop:+.1f} pp | {flag} |')
md+=['','## Replay accounting','','Inactivity is the existing no-combat timer, which may award either side a win by remaining mana. “No damage” means no recorded hit at any point, a stricter passive-ending indicator. All trace files include moves, pickups, hit tiles, mana totals and termination cause.','','| Scenario | Version | Inactivity endings / wins | No-damage games / wins | First Win / Loss / Draw trials |','|---|---|---|---|---|']
for name in list(changes)+list(junctions):
 for stage in (['junction-final'] if name in junctions else ['original','final']):
  gs=games(stage,name);inactive=[g for g in gs if g['termination']=='Inactivity'];passive=[g for g in gs if not any(m['damage'] for m in g['replay'])]
  reps=[next((str(g['trial']) for g in gs if g['outcome']==o),'—') for o in ['Win','Loss','Draw']]
  md.append(f"| {name} | [{stage}]({stage}/{name.lower().replace(' ','-')}/matchup-00-1-1.json) | {len(inactive)} / {sum(g['outcome']=='Win' for g in inactive)} | {len(passive)} / {sum(g['outcome']=='Win' for g in passive)} | {' / '.join(reps)} |")
md+=['','See [interpretation](interpretation.md) for inspected teaching sequences, remaining draw problems, and limitations. Individual screen reports retain all search telemetry, sensitivity/reversal flags and node saturation findings. The combined JSON preserves all aggregate telemetry with source attribution.']
(root/'report.md').write_text('\n'.join(md)+'\n');(root/'combined.json').write_text(json.dumps(dict(graph='graph.json',matchups=combined),indent=2)+'\n')
print(f'Combined {len(combined)} matchups; {sum(n(x["stats"]) for x in combined)} represented trials; all paired seeds and original first-30 results verified.')

# Standalone map for reviewing the same cardinal graph used by the UI.
from html import escape
positions={e['id']:e['position'] for e in graph['catalogue']}
scale=120;offset=(70,420)
xy=lambda pos:(offset[0]+pos[0]*scale,offset[1]+pos[1]*scale)
svg=['<svg xmlns="http://www.w3.org/2000/svg" width="1820" height="650" viewBox="0 0 1820 650">', '<rect width="100%" height="100%" fill="#072d2d"/>','<defs><marker id="arrow" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M0,0 L8,4 L0,8" fill="#ccc5de"/></marker></defs>', '<text x="24" y="28" fill="white" font-family="sans-serif" font-size="18">Campaign: 36 battle portals; every link joins cardinal neighbours</text>']
for edge in graph['connections']:
 a,b=positions[edge['from']],positions[edge['to']]
 assert abs(a[0]-b[0])+abs(a[1]-b[1])==1
 ax,ay=xy(a);bx,by=xy(b);dx,dy=b[0]-a[0],b[1]-a[1]
 arrow=' marker-end="url(#arrow)"' if edge['one_way'] else ''
 svg.append(f'<path d="M{ax+dx*34},{ay+dy*34} L{bx-dx*34},{by-dy*34}" fill="none" stroke="#ccc5de" stroke-width="2"{arrow}/>')
colors={'Grass':'#2f704c','Desert':'#997443','Flesh':'#a65560','Crust':'#73605c','Eldritch':'#765296'}
for e in graph['catalogue']:
 x,y=xy(e['position']);junction=e['id'].startswith('junction-')
 svg.append(f'<rect x="{x-32}" y="{y-22}" width="64" height="44" rx="8" fill="{colors[e["style"]]}" stroke="#ede5ce"/>')
 if junction:svg.append(f'<text x="{x}" y="{y+4}" fill="white" text-anchor="middle" font-family="sans-serif" font-size="13">1v1</text>')
 svg.append(f'<text x="{x}" y="{y+39}" fill="white" text-anchor="middle" font-family="sans-serif" font-size="12">{escape(e["name"])}</text>')
svg+=['<text x="24" y="620" fill="white" font-family="sans-serif" font-size="14">Arrowed exits unlock forwards only. Nearby portals without a drawn link do not unlock each other.</text>','</svg>']
(root/'map.svg').write_text('\n'.join(svg)+'\n')
