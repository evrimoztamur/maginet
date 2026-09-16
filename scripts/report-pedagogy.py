"""Compare teaching candidates with matched seeds; keep catalogue results descriptive."""
import argparse
import json
from pathlib import Path
import runpy

h = runpy.run_path(str(Path(__file__).with_name('report-overcharge.py')))


def outcome(stats):
    return '/'.join(str(stats['outcomes'][key]) for key in h['OUTCOMES'])


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', type=Path, default=Path('assessments/campaign-pedagogy'))
    parser.add_argument('--candidates-only', action='store_true')
    args = parser.parse_args()
    root = args.root
    entries = json.loads((root/'candidates.json').read_text())
    screens, followups = {}, {}
    for entry in entries:
        meta, cells = h['load'](root/'screen'/entry['id'])
        assert len(cells) == 1
        screens[entry['id']] = h['describe'](cells[(0, 1, 1)])
        if (root/'followup'/entry['id']).exists():
            fm, fc = h['load'](root/'followup'/entry['id'])
            assert fm['catalogue'] == meta['catalogue']
            for key in meta['config']:
                if key != 'games':
                    assert fm['config'][key] == meta['config'][key]
            assert fc[(0, 1, 1)][:30] == cells[(0, 1, 1)]
            followups[entry['id']] = (fm, fc[(0, 1, 1)])
    selected = []
    for entry in entries:
        if not entry.get('selected'):
            continue
        baseline = next(e for e in entries if e['baseline'] and e['battle'] == entry['battle'])
        am, a = followups[baseline['id']]
        bm, b = followups[entry['id']]
        assert am['config'] == bm['config']
        selected.append({'id': entry['adopt_as'], 'candidate': entry['id'],
                         'baseline': baseline['id'], 'lesson': entry['lesson'],
                         'all_300': h['compare'](a, b),
                         'holdout_270': h['compare'](a[30:], b[30:])})
    tests = [(entry, metric) for entry in selected if entry['candidate'] != entry['baseline']
             for metric in ('mcnemar_exact_p', 'draw_mcnemar_exact_p')]
    ranked = sorted(tests, key=lambda pair: pair[0]['holdout_270'][pair[1]])
    running = 0.
    for rank, (entry, metric) in enumerate(ranked):
        record = entry['holdout_270']
        running = max(running, min(1., (len(ranked)-rank)*record[metric]))
        record['draw_holm_p' if metric.startswith('draw') else 'win_holm_p'] = running
    data = {'candidate_screen': screens, 'selected': selected, 'holdout_holm_tests': len(tests)}
    if not args.candidates_only:
        meta, cells = h['load'](root/'campaign')
        old_meta, old_cells = h['load'](root.parent/'campaign-draws'/'all')
        assert meta['config'] == old_meta['config']
        unchanged = []
        for i, entry in enumerate(meta['catalogue']):
            old = next(((j, e) for j, e in enumerate(old_meta['catalogue'])
                        if e['code'] == entry['code'] and e['style'] == entry['style']), None)
            if old is None or entry['chaos']:
                continue
            for key, games in cells.items():
                if key[0] == i:
                    assert games == old_cells[(old[0], *key[1:])], entry['id']
            unchanged.append(entry['id'])
        data['campaign'] = {
            'overall': h['describe']([g for games in cells.values() for g in games]),
            'unchanged_scenarios_exact_replay_match': unchanged,
            'cells': [{'id': meta['catalogue'][i]['id'], 'red': r, 'blue': b,
                       **h['describe'](games)} for (i, r, b), games in sorted(cells.items())]}
    (root/'comparison.json').write_text(json.dumps(data, indent=2)+'\n')
    lines = ['# Campaign teaching and mana assessment', '',
             'Three Basics battles follow the tutorial: recognize a two-target attack, coordinate different patterns, then reject a greedy attack that loses a mage. The original Basics I is removed because it removes the need to consider an effective enemy response. The [design](design.md) records the actual decisions and rejected alternatives.', '',
             '## Matched-seed Normal / Normal followups', '',
             'Each side has 300 trials with the combined draw rules and AI held constant. Red is the player. W/L/D/U means win/loss/draw/unresolved at the 200-ply cap. The new finale is compared with old Basics IV; new I with old II, and new II with old III. Other rows compare the same named battle.', '',
             '| New battle | Previous W/L/D/U | Revised W/L/D/U | Red win rate | Δ win pp |',
             '|---|---|---|---|---:|']
    for entry in selected:
        c = entry['all_300']
        lines.append(f"| {entry['id']} | {outcome(c['before'])} | {outcome(c['after'])} | {c['before']['red_win_rate']:.1%} → {c['after']['red_win_rate']:.1%} | {c['red_win_delta_pp']:+.1f} |")
    lines += ['', '## Separate 270-trial followups', '',
              'Candidates were explored using 30 trials and exact opening analysis. Every followup reproduces those first 30 trials exactly. Results below exclude them. Holm adjustment covers win and draw metrics for seven changed battles (14 tests). Repositioned Basics I is unchanged and excluded from these tests. This is evidence about these simulated agents, not a human learning curve.', '',
              '| Battle | Δ win pp | Win Holm p | Δ draw pp | Draw Holm p |',
              '|---|---:|---:|---:|---:|']
    for entry in selected:
        if entry['candidate'] == entry['baseline']:
            continue
        c = entry['holdout_270']
        lines.append(f"| {entry['id']} | {c['red_win_delta_pp']:+.1f} | {c['win_holm_p']:.5f} | {c['draw_delta_pp']:+.1f} | {c['draw_holm_p']:.5f} |")
    if 'campaign' in data:
        c = data['campaign']['overall']
        lines += ['', '## Final catalogue screen', '',
                  f"The full revised catalogue contains 29 battles plus the tutorial. Excluding randomized Ascension III, the screen covers {len(cells)} profile/scenario cells and {c['n']:,} games: {outcome(c)} W/L/D/U. All nine Easy/Normal/Hard pairings receive 30 trials; the tutorial retains its Easy opponent. Mean length is {c['mean_plies']:.2f} plies.", '',
                  f"All {len(unchanged)} unchanged fixed scenarios reproduce the previous combined-rules screen exactly, including the moved Basics I. Changed level codes intentionally get their normal production seed namespaces here; this full-catalogue screen is not a paired causal comparison. Use the matched-seed followups above to compare revisions.", '',
                  '| Current battle | Normal / Normal W/L/D/U |', '|---|---|']
        for entry in data['campaign']['cells']:
            if entry['red'] == entry['blue'] == 1:
                lines.append(f"| {entry['id']} | {outcome(entry)} |")
    lines += ['', '## Reproduce', '',
              'Use the frozen combined-rules generator at the revision in [experiment.json](experiment.json) for candidates, then the campaign revision for the catalogue screen.', '',
              '```sh', 'python3 scripts/run-pedagogy.py screen', 'python3 scripts/run-pedagogy.py followup',
              'cargo run --release -p generate -- campaign --games 30 --seed 1 --max-plies 200 --workers 4 --replays --output assessments/campaign-pedagogy/campaign',
              'python3 scripts/report-pedagogy.py', '```', '',
              '[comparison.json](comparison.json) retains every candidate screen, followup, transition, Wilson win interval and final catalogue cell. [validation.json](validation.json) records tests, browser checks and authoritative replay audits. [design.md](design.md) explains the teaching choices and remaining playtest risks.', '']
    (root/'report.md').write_text('\n'.join(lines))
    print(json.dumps({'selected': len(selected), 'candidate_screens': len(screens),
                      'followups': len(followups), 'complete': not args.candidates_only}))


if __name__ == '__main__':
    main()
