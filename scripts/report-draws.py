"""Paired three-arm report; reuse the original screen without changing its files."""
import argparse
from collections import Counter
import json
import math
from pathlib import Path
import runpy

helpers = runpy.run_path(str(Path(__file__).with_name('report-overcharge.py')))
load, compare = helpers['load'], helpers['compare']
ARMS = ('control', 'rules', 'all')
PAIRS = {'rules_vs_control': ('control', 'rules'), 'all_vs_control': ('control', 'all'),
         'ai_vs_rules': ('rules', 'all')}


def describe(games):
    result = helpers['describe'](games)
    lengths = sorted(g['plies'] for g in games)
    result['median_plies'] = (lengths[(len(lengths)-1)//2] + lengths[len(lengths)//2])/2
    result['p95_plies'] = lengths[math.ceil(len(lengths)*.95)-1]
    result['p99_plies'] = lengths[math.ceil(len(lengths)*.99)-1]
    result['max_plies'] = max(lengths)
    return result


def outcome_text(stats):
    return '/'.join(str(stats['outcomes'][k]) for k in helpers['OUTCOMES'])


def datasets(paths):
    loaded = {arm: load(path) for arm, path in paths.items()}
    reference = loaded['control'][0]
    cells = {arm: data[1] for arm, data in loaded.items()}
    for arm, (meta, records) in loaded.items():
        for key in ('config', 'catalogue', 'graph', 'main_route'):
            assert meta[key] == reference[key], (arm, key)
        assert records.keys() == cells['control'].keys()
    return reference, cells


def comparisons(games):
    return {label: compare(games[a], games[b]) for label, (a, b) in PAIRS.items()}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', type=Path, default=Path('assessments/campaign-draws'))
    parser.add_argument('--screen-only', action='store_true')
    args = parser.parse_args()
    root = args.root
    experiment = json.loads((root/'experiment.json').read_text())
    paths = {arm: root/(experiment['control_dataset'] if arm == 'control' else arm) for arm in ARMS}
    meta, cells = datasets(paths)
    games = {arm: [g for key in sorted(cells[arm]) for g in cells[arm][key]] for arm in ARMS}
    overall = {arm: describe(g) for arm, g in games.items()}
    screen_comparisons = comparisons(games)
    cell_reports = []
    for key in sorted(cells['control']):
        level, red, blue = key
        records = {arm: cells[arm][key] for arm in ARMS}
        entry = meta['catalogue'][level]
        cell_reports.append({'id': entry['id'], 'name': entry['name'], 'red': red, 'blue': blue,
                             'arms': {arm: describe(g) for arm, g in records.items()},
                             'comparisons': comparisons(records)})
    profile_reports = []
    for red in range(3):
        for blue in range(3):
            records = {arm: [g for key in sorted(cells[arm]) if key[1:] == (red, blue)
                             for g in cells[arm][key]] for arm in ARMS}
            profile_reports.append({'red': red, 'blue': blue,
                                    'arms': {arm: describe(g) for arm, g in records.items()},
                                    'comparisons': comparisons(records)})
    followups = []
    if not args.screen_only:
        for scenario in experiment['followup_ids']:
            fm, fc = datasets({arm: root/'followup'/scenario/arm for arm in ARMS})
            assert len(fc['control']) == 1
            screen_index = next(i for i, entry in enumerate(meta['catalogue']) if entry['id'] == scenario)
            assert fm['catalogue'][0] == meta['catalogue'][screen_index]
            records = {arm: fc[arm][(0, 1, 1)] for arm in ARMS}
            for arm in ARMS:
                assert records[arm][:30] == cells[arm][(screen_index, 1, 1)], (scenario, arm)
            followups.append({'id': scenario, 'name': fm['catalogue'][0]['name'],
                              'arms': {arm: describe(g) for arm, g in records.items()},
                              'comparisons': comparisons(records),
                              'holdout_270': comparisons({arm: g[30:] for arm, g in records.items()})})
    # Two metrics for two primary comparisons across the six preselected levels.
    tests = [(f, comparison, metric) for f in followups
             for comparison in ('all_vs_control', 'ai_vs_rules')
             for metric in ('mcnemar_exact_p', 'draw_mcnemar_exact_p')]
    ranked = sorted(tests, key=lambda item: item[0]['holdout_270'][item[1]][item[2]])
    running = 0.
    for rank, (f, comparison, metric) in enumerate(ranked):
        record = f['holdout_270'][comparison]
        running = max(running, min(1., (len(ranked)-rank)*record[metric]))
        record['draw_holm_p' if metric.startswith('draw') else 'win_holm_p'] = running
    data = {'arms': overall, 'comparisons': screen_comparisons, 'cells': cell_reports,
            'profiles': profile_reports, 'followups': followups,
            'holdout_holm_tests': len(tests), 'complete': not args.screen_only}
    (root/'comparison.json').write_text(json.dumps(data, indent=2)+'\n')
    lines = ['# Inactivity and repetition: paired campaign assessment', '',
             'Control retains deadlock overcharge. Rules adds pickup resets and sixteen quiet plies; All adds an AI preference for fewer past visits among equal root scores. Nothing is merged. This measures simulated agents, not human play.', '',
             'The screen uses 30 identical seeded trials in each of 264 profile/scenario cells: 7,920 games per arm. Profiles retain depths 2/4/8, node caps 1,000/5,000/20,000, and existing ranked sampling. The safety cap is 200 plies. Tutorial inactivity remains disabled; the randomized scenario is excluded. Control reuses the completed overcharge screen unchanged.', '',
             '## Full screen', '', '| Metric | Control | Rules | All |', '|---|---:|---:|---:|']
    for label, key in [('Red wins','Win'),('Red losses','Loss'),('Draws','Draw'),('Safety-limit games','SafetyLimit')]:
        lines.append('| '+label+' | '+' | '.join(str(overall[a]['outcomes'][key]) for a in ARMS)+' |')
    lines.append('| Draw rate | '+' | '.join(f"{overall[a]['outcomes']['Draw']/overall[a]['n']:.2%}" for a in ARMS)+' |')
    for key in ('Elimination','Immobilization','Inactivity'):
        lines.append('| '+key+' endings | '+' | '.join(str(overall[a]['terminations'].get(key,0)) for a in ARMS)+' |')
    for label, key in [('Mean plies','mean_plies'),('Median plies','median_plies'),('95th percentile plies','p95_plies'),('99th percentile plies','p99_plies'),('Longest game, plies','max_plies'),('Overcharges','overcharged')]:
        lines.append('| '+label+' | '+' | '.join(f'{overall[a][key]:.2f}' if key=='mean_plies' else str(overall[a][key]) for a in ARMS)+' |')
    lines += ['', '| Comparison | Δ Red win pp | Δ draw pp | Draw→decisive | Decisive→draw | Changed outcomes |', '|---|---:|---:|---:|---:|---:|']
    for label, c in screen_comparisons.items():
        lines.append(f"| {label} | {c['red_win_delta_pp']:+.2f} | {c['draw_delta_pp']:+.2f} | {c['paired_draw_reductions']} | {c['paired_draw_increases']} | {c['outcome_changes']} |")
    lines += ['', 'Draw transitions include any non-draw outcome; safety-limit counts are shown explicitly above. Aggregate profile mixtures are descriptive, not a human difficulty rating. Shared trial seeds across profiles also limit interpreting pooled paired p-values as independent evidence.', '',
              '## Normal / Normal screen', '', 'W/L/D/U = Red wins / losses / draws / unresolved at the safety cap. Each row has 30 trials per arm; estimates are coarse.', '',
              '| Battle | Control W/L/D/U | Rules W/L/D/U | All W/L/D/U | All Δ Red win pp |', '|---|---|---|---|---:|']
    for c in cell_reports:
        if c['red'] == c['blue'] == 1:
            lines.append('| '+c['name']+' | '+' | '.join(outcome_text(c['arms'][a]) for a in ARMS)+f" | {c['comparisons']['all_vs_control']['red_win_delta_pp']:+.1f} |")
    lines += ['', '## Preselected followups', '',
              'Six battles were selected from the old screen for their inactivity-draw counts before looking at new results. Each receives 300 Normal/Normal trials per arm. First 30 trials must reproduce the screen exactly; only the remaining 270 enter confirmation tests. Holm adjustment covers win and draw tests for All versus Control and All versus Rules across all six battles (24 tests).', '',
              '| Battle | Control W/L/D/U | Rules W/L/D/U | All W/L/D/U |', '|---|---|---|---|']
    for f in followups:
        lines.append('| '+f['name']+' | '+' | '.join(outcome_text(f['arms'][a]) for a in ARMS)+' |')
    lines += ['', '| Battle | Comparison, new 270 trials | Δ Red win pp | Win Holm p | Δ draw pp | Draw Holm p |', '|---|---|---:|---:|---:|---:|']
    for f in followups:
        for comparison in ('all_vs_control','ai_vs_rules'):
            c=f['holdout_270'][comparison]
            lines.append(f"| {f['name']} | {comparison} | {c['red_win_delta_pp']:+.2f} | {c['win_holm_p']:.5f} | {c['draw_delta_pp']:+.2f} | {c['draw_holm_p']:.5f} |")
    lines += ['', '## Reproduce', '',
              'Freeze the generator and audit executables at the revisions in [experiment.json](experiment.json) into `target/draw-experiment/{control,rules,all}/`. Then run:', '',
              '```sh', 'python3 scripts/run-draws.py screen', 'python3 scripts/run-draws.py followup --arms control rules all', 'python3 scripts/report-draws.py', '```', '',
              '[comparison.json](comparison.json) includes every profile/scenario cell, paired transitions, Wilson win intervals and telemetry. [interpretation.md](interpretation.md) explains practical effects and limitations. [validation.json](validation.json) records tests and replay audits. Raw checkpoints include full move, pickup, damage and overcharge traces. Frozen binaries are ignored build artifacts; exact source commits are retained.', '']
    (root/'report.md').write_text('\n'.join(lines))
    print(json.dumps({'arms':overall,'followups':len(followups),'complete':not args.screen_only},indent=2))


if __name__ == '__main__':
    main()
