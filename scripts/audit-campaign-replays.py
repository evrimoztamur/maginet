"""Independently audit recorded move order, hit accounting, and team mana (stdlib)."""
import json
from pathlib import Path

root = Path('assessments/campaign-revision')
checked = 0
for path in root.glob('*/*/matchup-*.json'):
    metadata = json.loads((path.parent / 'metadata.json').read_text())
    code = metadata['catalogue'][0]['code']
    alphabet = '0123456789abcdefghjkmnpqrstvwxyz'
    bits = ''.join(f'{alphabet.index(c):05b}' for c in code)
    data = bytes(int(bits[i:i+8], 2) for i in range(0, len(bits)-7, 8))
    initial = {
        i: {'position': [data[2+3*i] >> 5, (data[2+3*i] >> 2) & 7],
            'team': data[2+3*i] & 3, 'mana': data[4+3*i] >> 4}
        for i in range(data[1])
    }
    for game in json.loads(path.read_text())['games']:
        mages = {i: {**m, 'position': m['position'][:]} for i, m in initial.items()}
        assert len(game['replay']) == game['plies']
        for ply, move in enumerate(game['replay']):
            team = (data[0] & 3) ^ (ply % 2)
            assert move['team'] == ['Red', 'Blue'][team]
            mage = next(m for m in mages.values() if m['mana'] and m['position'] == move['turn'][0])
            assert mage['team'] == team
            mage['position'] = move['turn'][1]
            for hit in move['damage']:
                victim = next(m for m in mages.values() if m['mana'] and m['position'] == hit)
                victim['mana'] -= 1
            assert move['mana'] == [sum(m['mana'] for m in mages.values() if m['team'] == t) for t in range(2)]
        checked += 1
print(f'Trace mana/order audit passed for {checked} games, including archived candidates.')
