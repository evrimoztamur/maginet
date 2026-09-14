"""Compare combat redesigns to the preceding 300-trial assessment without rerunning it."""
import collections
import json
from pathlib import Path

ROOT=Path('assessments/campaign-combat-redesign')
PREVIOUS=Path('assessments/campaign-revision')
read=lambda p:json.loads(p.read_text())
entries=read(ROOT/'candidates.json')
selected={e['battle']:e for e in entries if e.get('selected')}
names={e['id']:e['name'] for e in read(ROOT/'graph.json')['catalogue']}
def records(stage,ident):return read(ROOT/stage/ident/'matchup-00-1-1.json')['games']
def counts(gs):return '/'.join(str(sum(g['outcome']==out for g in gs)) for out in ['Win','Loss','Draw','SafetyLimit'])
def inactivity(gs):return sum(g['termination']=='Inactivity' for g in gs)
def elimination(gs):return sum(g['termination']=='NoLegalMoves' and 0 in g['replay'][-1]['mana'] for g in gs)
def stats(stage,ident,r=1,b=1):return next(e['stats'] for e in read(ROOT/stage/ident/'aggregate.json') if e['red']==r and e['blue']==b)
def n(s):return s['wins']+s['losses']+s['draws']+s['unresolved']
def cell(s):
 p=s['resolved_win_rate'];ci=s['wilson95'];bounds=s['overall_win_range']
 return f"{100*p:.1f}% [{100*ci[0]:.1f}, {100*ci[1]:.1f}]; n={n(s)}; U={s['unresolved']}; range {100*bounds[0]:.1f}–{100*bounds[1]:.1f}%" if p is not None else f"unresolved; n={n(s)}"
md=['# Combat-focused campaign redesign','','Patterns I, Rite II, and Rite IV were redesigned to finish through combat more often. The cardinal map, 36 portals, tutorial, other battles, AI profiles, shared rules and 200-ply cap are unchanged. Human difficulty remains unvalidated.','','The [candidate manifest](candidates.json) preserves all screened designs. Each screen has 30 trials in all nine matchups. Selected candidates receive 300 Normal/Normal trials using exactly the preceding assessment’s explicit seed namespace. The preceding 300-trial results are reused rather than rerun. Final first-30 trials overlap the screen and are not independent replications.','','## Candidate screening','','| Battle | Candidate | W/L/D/U (Normal/Normal) | Inactivity endings | Elimination endings | Selected |','|---|---|---|---:|---:|---|']
for e in entries:
 gs=records('screen',e['id'])
 md.append(f"| {names[e['battle']]} | [{e['id']}](screen/{e['id']}/report.md) | {counts(gs)} | {inactivity(gs)} | {elimination(gs)} | {'yes' if e.get('selected') else 'no'} |")
md+=['','Elimination endings have no legal moves and at least one team at zero mana. Other no-legal-move endings can be immobilization, so they are not silently counted as elimination.','','## Paired 300-trial followups','','| Battle | Before W/L/D/U | After W/L/D/U | Inactivity before → after | Elimination before → after | Win rate before [95% CI] | Win rate after [95% CI] |','|---|---|---|---|---|---|---|']
for battle,e in selected.items():
 before=read(PREVIOUS/'final'/battle/'matchup-00-1-1.json')['games'];after=records('final',e['id'])
 bm=read(PREVIOUS/'final'/battle/'metadata.json');am=read(ROOT/'final'/e['id']/'metadata.json')
 assert bm['engine']==am['engine'] and bm['config']==am['config']
 assert [g['seed'] for g in before]==[g['seed'] for g in after]
 assert after[:30]==records('screen',e['id'])
 bs=read(PREVIOUS/'final'/battle/'aggregate.json')[0]['stats'];new=stats('final',e['id'])
 md.append(f"| {names[battle]} | {counts(before)} | {counts(after)} | {inactivity(before)} → {inactivity(after)} | {elimination(before)} → {elimination(after)} | {cell(bs)} | {cell(new)} |")
md+=['','Wilson intervals use resolved wins/(wins+losses+draws); unresolved bounds remain explicit. These are marginal intervals, not intervals for the paired difference. Candidate selection is exploratory and can overfit this seed namespace. No fixed win-rate threshold determines acceptance.','','## Current full campaign matrices','','Untouched cells are explicitly reused from the preceding combined assessment. Redesigned cells use 300 Normal/Normal trials and 30 for the other eight matchups. Each cell reports its actual sample count. Search telemetry remains in the linked dataset reports and combined JSON.']
combined=[]
for old in read(PREVIOUS/'combined.json')['matchups']:
 item=dict(old);battle=item['id'];r=item['red'];b=item['blue']
 if battle in selected:
  ident=selected[battle]['id'];stage='final' if r==b==1 else 'screen'
  item.update(source=f'{stage}/{ident}/matchup-00-{r}-{b}.json',stats=stats(stage,ident,r,b),reused_previous=False,reused_baseline=False)
 else:item.update(source='../campaign-revision/'+item['source'],reused_previous=True)
 combined.append(item)
lookup={(e['id'],e['red'],e['blue']):e for e in combined}
graph=read(ROOT/'graph.json')
for e in graph['catalogue']:
 md+=['',f"### {e['name']}",'','| Player / opponent | Easy | Normal | Hard |','|---|---|---|---|']
 for r in range(3):
  cells=[]
  for b in range(3):
   item=lookup.get((e['id'],r,b))
   cells.append(f"[{cell(item['stats'])}]({item['source']})" if item else '—')
  md.append('| '+['Easy','Normal','Hard'][r]+' | '+' | '.join(cells)+' |')
adj=collections.defaultdict(list)
for edge in graph['connections']:
 adj[edge['from']].append(edge['to'])
 if not edge['one_way']:adj[edge['to']].append(edge['from'])
dist={'tutorial':0};queue=collections.deque(['tutorial'])
while queue:
 node=queue.popleft()
 for to in adj[node]:
  if to not in dist:dist[to]=dist[node]+1;queue.append(to)
for heading,routes in [('Main route',[graph['main_route']]),('Optional routes',graph['optional_routes'])]:
 md+=['',f'## {heading}','','Normal/Normal comparisons; ≥20-point drops are exploratory flags, supported only by disjoint intervals and ≤10% unresolved at both endpoints. No multiple-comparison correction. Directed shortest distances are context.','','| Edge | Distances | Red win drop | Flag |','|---|---|---:|---|']
 for route in routes:
  for a,b in zip(route,route[1:]):
   if (a,1,1) not in lookup:continue
   sa,sb=lookup[a,1,1]['stats'],lookup[b,1,1]['stats'];drop=100*(sa['resolved_win_rate']-sb['resolved_win_rate'])
   supported=sa['wilson95'][0]>sb['wilson95'][1] and sa['unresolved']*10<=n(sa) and sb['unresolved']*10<=n(sb)
   flag=('supported candidate' if supported else 'followup candidate') if drop>=20-1e-8 else 'below threshold'
   md.append(f'| {names[a]} → {names[b]} | {dist[a]} → {dist[b]} | {drop:+.1f} pp | {flag} |')
md+=['','See [interpretation and replay findings](interpretation.md) for design decisions and remaining limitations.']
(ROOT/'report.md').write_text('\n'.join(md)+'\n')
(ROOT/'combined.json').write_text(json.dumps(dict(graph='graph.json',matchups=combined),indent=2)+'\n')
print(f'Validated {len(selected)} paired followups; combined {len(combined)} matchup cells with explicit provenance.')
