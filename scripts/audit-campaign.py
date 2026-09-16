"""Audit a completed campaign survey without rerunning search (stdlib only)."""
import json
import math
import pathlib
import sys

root = pathlib.Path(sys.argv[1] if len(sys.argv) > 1 else 'assessments/campaign-seed-1')
metadata = json.loads((root / 'metadata.json').read_text())
config, catalogue = metadata['config'], metadata['catalogue']
summaries = json.loads((root / 'aggregate.json').read_text())
expected = {(i, red, blue) for i, entry in enumerate(catalogue)
            if not entry.get('chaos')
            for red in range(3) for blue in range(1 if entry['tutorial'] else 3)
            if config.get('red_profile') in (None, red) and config.get('blue_profile') in (None, blue)}
assert {(s['level'], s['red'], s['blue']) for s in summaries} == expected
assert len(summaries) == len(expected)
assert len(list(root.glob('matchup-*.json'))) == len(expected)
alphabet = '0123456789abcdefghjkmnpqrstvwxyz'

def canonical(code):
    bits = ''.join(f'{alphabet.index(c):05b}' for c in code)
    data = bytes(int(bits[i:i+8], 2) for i in range(0, len(bits)-7, 8))
    start = 3 + 3 * data[1]
    props = sorted(data[i:i+2] for i in range(start, len(data), 2))
    data = data[:start] + b''.join(props)
    bits = ''.join(f'{b:08b}' for b in data)
    bits += '0' * (-len(bits) % 5)
    return ''.join(alphabet[int(bits[i:i+5], 2)] for i in range(0, len(bits), 5))

def seed(code, trial):
    h = 0xcbf29ce484222325
    for b in config['seed'].to_bytes(8, 'little') + (config['seed_namespace'] if config.get('seed_namespace') is not None else canonical(code)).encode() + trial.to_bytes(8, 'little'):
        h = ((h ^ b) * 0x100000001b3) & ((1 << 64) - 1)
    return h

totals = dict(Win=0, Loss=0, Draw=0, SafetyLimit=0)
for summary in summaries:
    i, red, blue = summary['level'], summary['red'], summary['blue']
    record = json.loads((root / f'matchup-{i:02}-{red}-{blue}.json').read_text())
    assert (record['level'], record['red'], record['blue']) == (i, red, blue)
    games, stats = record['games'], summary['stats']
    assert len(games) == config['games']
    for trial, game in enumerate(games):
        assert game['trial'] == trial and game['seed'] == seed(catalogue[i]['code'], trial)
        assert 0 <= game['plies'] <= config['max_plies']
        if game['outcome'] == 'SafetyLimit':
            assert game['plies'] == config['max_plies']
        totals[game['outcome']] += 1
        assert game['red']['searches'] + game['blue']['searches'] == game['plies']
        for side, profile in [('red', red), ('blue', blue)]:
            telemetry = game[side]
            assert sum(telemetry['depths'].values()) == telemetry['searches']
            assert sum(telemetry['stops'].values()) == telemetry['searches']
            assert telemetry['fallbacks'] == telemetry['depths'].get('0', 0)
            assert telemetry['nodes'] <= telemetry['searches'] * config['profiles'][profile]['nodes']
    for outcome, key in [('Win', 'wins'), ('Loss', 'losses'), ('Draw', 'draws'), ('SafetyLimit', 'unresolved')]:
        assert stats[key] == sum(g['outcome'] == outcome for g in games)
    n = len(games) - stats['unresolved']
    assert stats['resolved_win_rate'] == (stats['wins'] / n if n else None)
    assert stats['overall_win_range'] == [stats['wins']/len(games), (stats['wins']+stats['unresolved'])/len(games)]
    assert math.isclose(stats['mean_plies'], sum(g['plies'] for g in games)/len(games))
    for side in ['red', 'blue']:
        for key in ['nodes', 'searches', 'fallbacks']:
            assert stats[side][key] == sum(g[side][key] for g in games)
        for key in ['depths', 'stops']:
            combined = {}
            for g in games:
                for k, v in g[side][key].items():
                    combined[k] = combined.get(k, 0) + v
            assert stats[side][key] == combined
print(f'Verified {len(expected)} matchups / {sum(totals.values())} games: {totals}')
