"""Report paired Hard/Normal evidence, supporting profiles, and decision diagnostics."""
import argparse
from collections import Counter
import json
import math
from pathlib import Path

ROOT = Path('assessments/campaign-challenge')
ARCHIVE = Path('assessments/campaign-pedagogy/campaign')

def read(path):
    return json.loads(path.read_text()) if path.exists() and path.stat().st_size else None

def stats(games):
    c = Counter(g['outcome'] for g in games)
    n = len(games) - c['SafetyLimit']
    p = c['Win'] / n if n else None
    interval = None
    if n:
        z2 = 1.959963984540054 ** 2
        center = (p + z2 / (2*n)) / (1 + z2/n)
        half = math.sqrt(z2 * (p*(1-p)/n + z2/(4*n*n))) / (1+z2/n)
        interval = [max(0, center-half), min(1, center+half)]
    return dict(n=len(games), counts={o:c[o] for o in ['Win','Loss','Draw','SafetyLimit']}, win_rate=p, wilson95=interval,
                termination=dict(Counter(g['termination'] for g in games)), mean_plies=sum(g['plies'] for g in games)/len(games),
                overall_win_range=[c['Win']/len(games),(c['Win']+c['SafetyLimit'])/len(games)],
                overcharged=sum(g['overcharge_at'] is not None for g in games),
                depths={team:dict(sum((Counter(g[team]['depths']) for g in games), Counter())) for team in ['red','blue']},
                stops={team:dict(sum((Counter(g[team]['stops']) for g in games), Counter())) for team in ['red','blue']})

def pair(before, after):
    assert len(before) == len(after)
    assert all(a['trial']==b['trial'] and a['seed']==b['seed'] for a,b in zip(before,after)), 'unpaired trials'
    differences = [int(b['outcome']=='Win')-int(a['outcome']=='Win') for a,b in zip(before,after)]
    n=len(differences);mean=sum(differences)/n
    error=1.96*math.sqrt(sum((d-mean)**2 for d in differences)/(n-1)/n) if n>1 else 0
    return dict(win_delta=mean, paired_normal95=[max(-1,mean-error),min(1,mean+error)],
                gained_wins=differences.count(1), lost_wins=differences.count(-1), n=n)

def diagnostics(path):
    data=read(path)
    if not data:return None
    assert len(data)==1
    root=data[0];points=[root]+root['decisions'];interesting=[]
    for point in points:
        scores=[m['score'] for m in point['moves']]
        if scores and max(scores)>min(scores):
            interesting.append(dict(ply=point['ply'], trial=point.get('trial'), outcome=point.get('outcome'),
                                    depth=point['completed_depth'], stop=point['stop'], best=point['moves'][0],
                                    worst_score=min(scores), spread=max(scores)-min(scores),
                                    tied_best=sum(s==scores[0] for s in scores), continuation_result=point['continuation_result']))
    return dict(config=root['config'], opening_best=root['moves'][0] if root['moves'] else None,
                points=len(points), meaningful=len(interesting), completed_depths=sorted(set(p['completed_depth'] for p in points)),
                exhausted=sum(p['stop']=='Nodes' for p in points),
                depth_changes=sum(len({json.dumps(d['search']['moves'][0]['turn']) for d in p['depths'] if d['search']['moves']})>1 for p in points),
                red_winning_rollouts=sum(p['continuation_result']=='RedWin' for p in points),
                capped_rollouts=sum(p['continuation_capped'] for p in points),
                example=next((p for p in interesting if p['trial'] is not None and p['best']['score']==99999 and p['worst_score']==-99999),next(iter(interesting),None)),
                representatives=sorted({(p['outcome'],p['trial']) for p in root['decisions']}))

def main():
    parser=argparse.ArgumentParser();parser.add_argument('--strict',action='store_true');args=parser.parse_args()
    entries=read(ROOT/'candidates.json');archive=read(ARCHIVE/'metadata.json');rows=[];missing=[]
    for e in entries:
        if not e.get('selected'):continue
        baseline=next(x for x in entries if x['battle']==e['battle'] and x['baseline'])
        before_path=ROOT/'followup'/baseline['id']/'matchup-00-2-1.json'
        after_path=ROOT/'followup'/e['id']/'matchup-00-2-1.json'
        before,after=read(before_path),read(after_path)
        row=dict(id=e['battle'],candidate=e['id'],code=e['code'],idea=e['idea'],revised=not e['baseline'])
        index=next(i for i,x in enumerate(archive['catalogue']) if x['id']==e['battle'])
        original_meta=read(ROOT/'screen'/baseline['id']/'metadata.json')
        assert original_meta['engine']==archive['engine'], 'cannot reuse changed engine support'
        old_screen=read(ARCHIVE/f'matchup-{index:02}-2-1.json')['games']
        fresh_screen=read(ROOT/'screen'/baseline['id']/'matchup-00-2-1.json')['games']
        assert old_screen==fresh_screen, f"{e['battle']} archive no longer reproduces"
        row['support']={}
        for label,r,b in [('NN',1,1),('HH',2,2)]:
            previous=read(ARCHIVE/f'matchup-{index:02}-{r}-{b}.json')['games']
            path=ROOT/'support'/(e['id']+('-normal-normal' if label=='NN' else '-hard-hard'))/f'matchup-00-{r}-{b}.json'
            current=previous if e['baseline'] else (read(path) or {}).get('games')
            row['support'][label]=dict(before=stats(previous),after=stats(current) if current else None)
            if not current:missing.append(str(path))
        if before and after:
            before,after=before['games'],after['games'];assert len(before)==len(after)==300
            screen=read(ROOT/'screen'/e['id']/'matchup-00-2-1.json')['games']
            assert before[:30]==fresh_screen and after[:30]==screen, 'screen changed in followup'
            row.update(before=stats(before),after=stats(after),paired=pair(before,after),holdout270=pair(before[30:],after[30:]))
        else:missing.extend(str(p) for p in [before_path,after_path] if not p.exists())
        row['decisions']=diagnostics(ROOT/'decisions'/(e['id']+'.json'))
        if not row['decisions']:missing.append('decisions/'+e['id'])
        else:
            review_input=read(ROOT/'decisions'/(e['id']+'.input.json'))
            if row['decisions']['config']['nodes'] < 500000 or not review_input or review_input[0]['replay'] != str(after_path):
                missing.append('finalist review/'+e['id'])
        rows.append(row)
    rows.sort(key=lambda e:next(i for i,x in enumerate(archive["catalogue"]) if x["id"]==e["id"]))
    screens=[]
    for e in entries:
        data=read(ROOT/'screen'/e['id']/'matchup-00-2-1.json')
        review=diagnostics(ROOT/'decisions'/(e['id']+'.json'))
        if not data or len(data['games']) != 30:missing.append('screen/'+e['id'])
        if not review:missing.append('candidate review/'+e['id'])
        screens.append({**e,'stats':stats(data['games']) if data else None,'diagnostics':review})
    audit=read(ROOT/'replay-audit.json')
    if audit:
        indexed={m['path']:m for m in audit['matchups']}
        for row in rows:
            baseline=next(e for e in entries if e['battle']==row['id'] and e['baseline'])
            row['replay_audit']={label:indexed[str(ROOT/'followup'/candidate/'matchup-00-2-1.json')] for label,candidate in [('before',baseline['id']),('after',row['candidate'])]}
    result=dict(engine=archive['engine'],primary='Hard/Normal',support='30 paired Normal/Normal and Hard/Hard; unchanged original engine screens reused after exact HN replay verification',
                rows=rows,candidates=screens,chaos=[dict(red=m['red'],blue=m['blue'],stats=stats([t[2] for t in m['trials']])) for m in read(ROOT/'chaos.json')['matchups']],missing=missing)
    (ROOT/'comparison.json').write_text(json.dumps(result,indent=2)+'\n')
    def counts(s):return '/'.join(str(s['counts'][o]) for o in ['Win','Loss','Draw','SafetyLimit'])
    def rate(s):return f"{100*s['win_rate']:.1f}% [{100*s['wilson95'][0]:.1f}, {100*s['wilson95'][1]:.1f}]"
    lines=['# Later campaign assessment','', 'Hard player / Normal opponent is primary. Each completed row has 300 matched seeds. W/L/D/U separates wins, losses, draws, and unresolved games; intervals are Wilson 95% intervals for wins among resolved games (draws remain in the denominator). These are simulated outcomes, not human success rates.', '',
           '| Battle | Before W/L/D/U | After W/L/D/U | After win rate [95% CI] | Paired Δ percentage points | Mean plies before → after |', '|---|---|---|---|---|---|']
    for row in rows:
        if 'after' not in row:lines.append(f"| {row['id']} | pending | pending | | | |");continue
        a,b=row['before'],row['after'];delta=row['paired'];lines.append(f"| {row['id']} | {counts(a)} | {counts(b)} | {rate(b)} | {delta['win_delta']*100:+.1f} [{delta['paired_normal95'][0]*100:+.1f}, {delta['paired_normal95'][1]*100:+.1f}] | {a['mean_plies']:.1f} → {b['mean_plies']:.1f} |")
    lines+=['','## Supporting comparisons','', 'Original NN/HH screens use the identical engine and reproduce every original HN screen replay exactly. Only these two supporting profiles are used; no Easy-player result informs selection. Supporting samples have 30 games and consequently wide uncertainty. Full Wilson intervals, search-depth distributions, node-budget stops, termination counts, unresolved ranges and the 270-trial holdout deltas are in `comparison.json`.', '',
            '| Battle | Normal/Normal before → after W/L/D/U | Hard/Hard before → after W/L/D/U |','|---|---|---|']
    for row in rows:
        values=[]
        for label in ['NN','HH']:
            support=row['support'][label];values.append(counts(support['before'])+' → '+(counts(support['after']) if support['after'] else 'pending'))
        lines.append('| '+row['id']+' | '+' | '.join(values)+' |')
    lines+=['','## Decision review','', 'Decision files retain every legal scored alternative, scores across requested depths 2/4/8/10, best-versus-next gaps, all immediate replies, actual opponent-response diagnostics, and bounded best-play continuations. Scores are from Red’s perspective and only completed search iterations are published. A rollout win is evidence of a line, not a proof against every response. Budget-limited and capped results remain explicit.', '', '| Battle | Inspected points / score spreads | Completed depths | Budget stops | Depth-dependent best moves | Winning rollouts |','|---|---|---|---|---|---|']
    for row in rows:
        d=row['decisions']
        if d:lines.append(f"| {row['id']} | {d['points']} / {d['meaningful']} | {d['completed_depths']} | {d['exhausted']} | {d['depth_changes']} | {d['red_winning_rollouts']} |")
    if audit:
        lines+=['','## Replay audit','',f"The rules replay audit validates {audit['verified_games']:,} recorded games and {audit['verified_plies']:,} plies, including discarded candidates and supporting profiles. Counts below cover 300 primary games per version. Inactivity can award a win by remaining mana; it is not synonymous with a draw. Repetition ignores history and quiet-clock values, but includes pieces, runes, side to move and overcharge.",'', '| Battle | Games with repeated positions before → after | Inactivity endings before → after | Longest quiet stretch after (plies) |','|---|---|---|---|']
        for row in rows:
            a,b=row['replay_audit']['before'],row['replay_audit']['after']
            lines.append(f"| {row['id']} | {a['repeated_games']} → {b['repeated_games']} | {a['terminations'].get('Inactivity',0)} → {b['terminations'].get('Inactivity',0)} | {b['max_quiet_plies']} |")
    lines+=['','## Randomized Chaos finale','', 'Separate 30-loadout samples; the same loadout seeds are paired across profiles. The template and random generation are unchanged. These results are not used to tune fixed encounters.', '', '| Profile | W/L/D/U | Win rate [95% CI] | Mean plies | Termination |','|---|---|---|---|---|']
    for matchup in result['chaos']:
        label={1:'Normal',2:'Hard'}
        s=matchup['stats']
        lines.append(f"| {label[matchup['red']]}/{label[matchup['blue']]} | {counts(s)} | {rate(s)} | {s['mean_plies']:.1f} | {s['termination']} |")
    lines+=['','See `design.md` for strategic ideas, rejected candidates, target-band exceptions and progression. Screens are exploratory: the first 30 trials are included in the 300-trial totals. `holdout270` excludes those trials; no result is presented as an independent pre-registered experiment.','',f"Pending outputs: {len(missing)}."]
    (ROOT/'report.md').write_text('\n'.join(lines)+'\n')
    print(f'{len(rows)} encounters, {len(screens)} candidates, {len(missing)} pending outputs')
    if args.strict:assert not missing, missing

if __name__=='__main__':main()
