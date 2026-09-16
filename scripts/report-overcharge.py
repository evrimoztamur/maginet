"""Paired campaign rule comparison. Standard library only; retains raw provenance."""
import argparse
from collections import Counter
import json
import math
from pathlib import Path

OUTCOMES = ('Win', 'Loss', 'Draw', 'SafetyLimit')


def load(root):
    meta = json.loads((root / 'metadata.json').read_text())
    expected = {(i, r, b) for i, e in enumerate(meta['catalogue']) if not e.get('chaos')
                for r in range(3) for b in range(1 if e['tutorial'] else 3)
                if meta['config'].get('red_profile') in (None, r)
                and meta['config'].get('blue_profile') in (None, b)}
    cells = {}
    for path in root.glob('matchup-*.json'):
        cell = json.loads(path.read_text())
        key = (cell['level'], cell['red'], cell['blue'])
        assert key not in cells
        assert len(cell['games']) == meta['config']['games']
        cells[key] = cell['games']
    assert cells.keys() == expected, f'Incomplete data: {root}: {len(cells)}/{len(expected)}'
    return meta, cells


def wilson(wins, n):
    z2 = 1.959963984540054 ** 2
    p = wins / n
    center = (p + z2 / (2 * n)) / (1 + z2 / n)
    half = (z2 * (p * (1 - p) / n + z2 / (4 * n * n))) ** .5 / (1 + z2 / n)
    return [center - half, center + half]


def describe(games):
    counts = Counter(g['outcome'] for g in games)
    terminations = Counter()
    for g in games:
        cause = g['termination']
        if cause == 'NoLegalMoves' and g.get('replay'):
            cause = 'Elimination' if 0 in g['replay'][-1]['mana'] else 'Immobilization'
        terminations[cause] += 1
    return {
        'n': len(games), 'outcomes': {k: counts[k] for k in OUTCOMES},
        'red_win_rate': counts['Win'] / len(games),
        'red_win_wilson95': wilson(counts['Win'], len(games)),
        'terminations': dict(terminations),
        'mean_plies': sum(g['plies'] for g in games) / len(games),
        'overcharged': sum(g.get('overcharge_at') is not None for g in games),
        'overcharge_survivors': dict(Counter(g['overcharge_mages'] for g in games if g.get('overcharge_mages') is not None)),
        'nodes': sum(g[t]['nodes'] for g in games for t in ('red', 'blue')),
        'searches': sum(g[t]['searches'] for g in games for t in ('red', 'blue')),
        'fallbacks': sum(g[t]['fallbacks'] for g in games for t in ('red', 'blue')),
    }


def compare(before, after):
    assert len(before) == len(after)
    for a, b in zip(before, after):
        assert (a['trial'], a['seed']) == (b['trial'], b['seed'])
    gain = sum(a['outcome'] != 'Win' and b['outcome'] == 'Win' for a, b in zip(before, after))
    loss = sum(a['outcome'] == 'Win' and b['outcome'] != 'Win' for a, b in zip(before, after))
    def paired_p(up, down):
        n = up + down
        return min(1., 2 * sum(math.comb(n, k) for k in range(min(up, down) + 1)) / 2 ** n) if n else 1.
    draw_reductions = sum(a['outcome'] == 'Draw' and b['outcome'] != 'Draw' for a, b in zip(before, after))
    draw_increases = sum(a['outcome'] != 'Draw' and b['outcome'] == 'Draw' for a, b in zip(before, after))
    return {
        'before': describe(before), 'after': describe(after),
        'red_win_delta_pp': 100 * (gain - loss) / len(before),
        'paired_win_gains': gain, 'paired_win_losses': loss,
        'mcnemar_exact_p': paired_p(gain, loss),
        'draw_delta_pp': 100 * (draw_increases - draw_reductions) / len(before),
        'paired_draw_reductions': draw_reductions, 'paired_draw_increases': draw_increases,
        'draw_mcnemar_exact_p': paired_p(draw_reductions, draw_increases),
        'outcome_changes': sum(a['outcome'] != b['outcome'] for a, b in zip(before, after)),
        'transitions': dict(Counter(f"{a['outcome']} -> {b['outcome']}" for a, b in zip(before, after))),
    }


def paired_dataset(before_root, after_root):
    before_meta, before_cells = load(before_root)
    after_meta, after_cells = load(after_root)
    for key in ('config', 'catalogue', 'graph', 'main_route'):
        assert before_meta[key] == after_meta[key], f'Unpaired {key}'
    assert before_cells.keys() == after_cells.keys()
    cells = []
    all_before, all_after = [], []
    for key in sorted(before_cells):
        i, red, blue = key
        entry = before_meta['catalogue'][i]
        a, b = before_cells[key], after_cells[key]
        cells.append({'id': entry['id'], 'name': entry['name'], 'red': red, 'blue': blue, **compare(a, b)})
        all_before.extend(a)
        all_after.extend(b)
    return before_meta, cells, compare(all_before, all_after), before_cells, after_cells


def table_row(c):
    a, b = c['before'], c['after']
    outcomes = lambda x: '/'.join(str(x['outcomes'][k]) for k in OUTCOMES)
    return (f"| {c['name']} | {a['n']} | {outcomes(a)} | {outcomes(b)} | "
            f"{a['red_win_rate']:.1%} → {b['red_win_rate']:.1%} | {c['red_win_delta_pp']:+.1f} | "
            f"{b['overcharged']} | {c['mcnemar_exact_p']:.4f} |")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument('--root', type=Path, default=Path('assessments/campaign-overcharge'))
    args = parser.parse_args()
    root = args.root
    meta, cells, overall, a_cells, b_cells = paired_dataset(root / 'before', root / 'after')
    nn = [c for c in cells if c['red'] == c['blue'] == 1]
    affected_ids = sorted({c['id'] for c in cells if c['outcome_changes'] or c['after']['overcharged']})
    followups = []
    for directory in sorted((root / 'followup').glob('*')):
        if not directory.is_dir(): continue
        fm, fc, fo, fa, fb = paired_dataset(directory / 'before', directory / 'after')
        assert len(fc) == 1 and fc[0]['red'] == fc[0]['blue'] == 1
        scenario_id = fc[0]['id']
        screen_index = next(i for i, e in enumerate(meta['catalogue']) if e['id'] == scenario_id)
        screen_key = (screen_index, 1, 1)
        assert fm['catalogue'][0] == meta['catalogue'][screen_index]
        assert fa[(0, 1, 1)][:30] == a_cells[screen_key]
        assert fb[(0, 1, 1)][:30] == b_cells[screen_key]
        holdout = compare(fa[(0, 1, 1)][30:], fb[(0, 1, 1)][30:])
        followups.append({**fc[0], 'holdout_270': holdout})
    # Holm adjustment across both win and draw tests in all independent holdouts.
    running = 0.
    ranked = sorted([(c, key) for c in followups for key in ('mcnemar_exact_p', 'draw_mcnemar_exact_p')], key=lambda item: item[0]['holdout_270'][item[1]])
    for rank, (c, key) in enumerate(ranked):
        running = max(running, min(1., (len(ranked)-rank)*c['holdout_270'][key]))
        c['holdout_270']['draw_holm_p' if key.startswith('draw') else 'holm_p'] = running
    data = {'overall': overall, 'cells': cells, 'normal_normal': nn, 'affected_ids': affected_ids, 'followups': followups}
    (root / 'comparison.json').write_text(json.dumps(data, indent=2)+'\n')
    text = ['# Deadlock overcharge: paired campaign assessment', '',
            'Simulated agents, not human playtesting. Red is the player. All before/after pairs use identical current scenarios, seeds, profile budgets and trial indices. Historical assessments are unchanged.', '',
            f"The screen covers {len(cells)} matchup cells and {overall['before']['n']:,} games per ruleset. Each cell uses 30 trials, seed 1 and a 200-ply safety cap. Easy/Normal/Hard use depths 2/4/8, node caps 1,000/5,000/20,000 and the existing ranked selection probabilities. Tutorial remains Easy-opponent with inactivity and overcharge disabled. Randomized Ascension III is excluded, as in the earlier fixed-scenario analyses.", '',
            'The rule proves absence of damage using four-phase independent position bitboards, ignoring living-piece collisions. It excludes any remaining collectible or held ability. It grants every survivor a diagonal rune only when cardinal contact is impossible and diagonal contact is possible in the abstraction. It runs at initialization and after moves, before inactivity adjudication; already terminal no-move positions remain terminal. Overcharge resets the inactivity counter once. The ordinary timeout and mana tiebreak remain in force.', '',
            '## Full screen', '', '| Metric | Before | After |', '|---|---:|---:|']
    for label, key in [('Red wins','Win'), ('Red losses','Loss'), ('Draws','Draw'), ('Safety-limit games','SafetyLimit')]:
        text.append(f"| {label} | {overall['before']['outcomes'][key]} | {overall['after']['outcomes'][key]} |")
    for key in ('Elimination','Immobilization','Inactivity','SafetyLimit'):
        text.append(f"| {key} endings | {overall['before']['terminations'].get(key,0)} | {overall['after']['terminations'].get(key,0)} |")
    text += [f"| Mean plies | {overall['before']['mean_plies']:.2f} | {overall['after']['mean_plies']:.2f} |",
             f"| Actual overcharge activations | 0 | {overall['after']['overcharged']} |", '',
             f"Paired outcome changes: {overall['outcome_changes']}. Actual activation survivor counts: `{overall['after']['overcharge_survivors']}`. Aggregate totals mix all difficulty profiles and should not be read as a human difficulty rating.", '',
             '## Normal / Normal screen', '',
             'W/L/D/U means Red wins, losses, draws and safety-limit unresolved games. Win percentages use all trials; unresolved counts are shown explicitly. p is the exact paired McNemar test for a Red win versus any other outcome, unadjusted and exploratory. Thirty trials give coarse estimates.', '',
             '| Battle | N per arm | Before W/L/D/U | After W/L/D/U | Red wins | Δ pp | Overcharges | Paired p |',
             '|---|---:|---|---|---|---:|---:|---:|']
    text += [table_row(c) for c in nn]
    text += ['', '## Followups', '',
             'Affected scenarios receive 300 paired Normal/Normal trials. The first 30 exactly reproduce the screen and are not independent evidence. The remaining 270 provide a separate confirmation sample; Holm adjustment covers both win-rate and draw-rate tests across all selected followups.', '',
             '| Battle | N per arm | Before W/L/D/U | After W/L/D/U | Red wins | Δ pp | Overcharges | Paired p |',
             '|---|---:|---|---|---|---|---:|---:|']
    text += [table_row(c) for c in followups]
    if followups:
        text += ['', '| Battle | New 270-trial Δ win pp | Win Holm p | Δ draw pp | Draw Holm p |', '|---|---:|---:|---:|---:|']
        text += [f"| {c['name']} | {c['holdout_270']['red_win_delta_pp']:+.1f} | {c['holdout_270']['holm_p']:.5f} | {c['holdout_270']['draw_delta_pp']:+.1f} | {c['holdout_270']['draw_holm_p']:.5f} |" for c in followups]
    text += ['', '## Coverage and reproducibility', '',
             f"Screen-selected followup IDs: {', '.join(affected_ids) or 'none'}.", '',
             'The complete nine-profile comparisons, outcome transition counts, Wilson intervals, node/fallback telemetry, and activation counts are in [comparison.json](comparison.json). Each arm retains metadata, engine fingerprints, per-trial checkpoints, complete move/pickup/damage replays, and the standard campaign report. [experiment.json](experiment.json) records source identity and commands. [detector-benchmark.json](detector-benchmark.json) records native timing batches. [interpretation.md](interpretation.md) discusses findings and limitations.', '',
             'Run `python3 scripts/report-overcharge.py` after completing both arms and followups. Reproduction must use the baseline snapshot for the before binary and the experiment rules for the after binary. Do not mix checkpoints across engine fingerprints.', '']
    (root / 'report.md').write_text('\n'.join(text))
    print(json.dumps({'overall':overall,'affected_ids':affected_ids,'followups':len(followups)},indent=2))


if __name__ == '__main__': main()
