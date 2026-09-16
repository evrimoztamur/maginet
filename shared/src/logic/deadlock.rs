//! Conservative reachability before the first damage. Living pieces may pass through
//! each other, so a possible contact is inconclusive; absence of contact is a proof.
use crate::{Level, Position, PowerUp, Team};

/// Result of the optimistic, powerup-free contact analysis.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ContactReachability {
    /// A contact exists in the abstraction, but may not be reachable in real play.
    Possible,
    /// Even the relaxed movement rules cannot produce damage.
    Impossible,
    /// Pickups, equipped abilities, or a finished battle are outside this analysis.
    Unsupported,
}

const FILE_A: u64 = 0x0101_0101_0101_0101;
const FILE_H: u64 = FILE_A << 7;

fn square(p: Position) -> u64 {
    1 << (p.1 as u32 * 8 + p.0 as u32)
}

fn neighbours(bits: u64, diagonal: bool) -> u64 {
    let horizontal = ((bits & !FILE_A) >> 1) | ((bits & !FILE_H) << 1);
    horizontal
        | (bits << 8)
        | (bits >> 8)
        | if diagonal {
            (horizontal << 8) | (horizontal >> 8)
        } else {
            0
        }
}

fn shifted(bits: u64, delta: Position) -> u64 {
    let (dx, dy) = (delta.0 as i32, delta.1 as i32);
    if dx.abs() >= 8 || dy.abs() >= 8 {
        return 0;
    }
    let bits = if dx >= 0 {
        (bits & (FILE_A * (0xff >> dx))) << dx
    } else {
        (bits & (FILE_A * ((0xff << -dx) & 0xff))) >> -dx
    };
    if dy >= 0 {
        bits << (8 * dy)
    } else {
        bits >> (-8 * dy)
    }
}

/// Proves a subset of structural deadlocks, independently of the inactivity clock.
///
/// Uses four turn phases and one bitboard per mage per phase. On a team's turn,
/// each mage may move, or wait if a teammate could consume that turn. Occupancy
/// by living mages is deliberately ignored. Boulders and sleepers remain blocked.
/// No damage is possible before the fixed point iff this optimistic model says
/// `Impossible`; `Possible` is not proof of an actual legal attack sequence.
/// `diagonal` models granting diagonal movement to every survivor.
pub fn contact_reachability(level: &Level, to_move: Team, diagonal: bool) -> ContactReachability {
    if level.board.width > 8
        || level.board.height > 8
        || level
            .powerups
            .values()
            .any(|p| !matches!(p, PowerUp::Boulder(_)))
        || level
            .mages
            .iter()
            .any(|m| m.is_alive() && m.powerup.is_some())
    {
        return ContactReachability::Unsupported;
    }
    let live: Vec<_> = level.mages.iter().filter(|m| m.is_alive()).collect();
    let mut counts = [0usize; 2];
    for mage in &live {
        counts[usize::from(mage.team == Team::Blue)] += 1;
    }
    if counts.contains(&0) {
        return ContactReachability::Unsupported;
    }
    let mut passable = 0;
    for y in 0..level.board.height {
        passable |= ((1u64 << level.board.width) - 1) << (y * 8);
    }
    for p in level.powerups.keys() {
        passable &= !square(*p);
    }
    for mage in level.mages.iter().filter(|m| !m.is_alive()) {
        passable &= !square(mage.position);
    }
    let mut possible: Vec<_> = live.iter().map(|m| [square(m.position), 0, 0, 0]).collect();
    loop {
        let mut changed = false;
        for phase in 0..4 {
            let team = if phase % 2 == 0 {
                to_move
            } else {
                to_move.enemy()
            };
            let enemies = live
                .iter()
                .zip(&possible)
                .filter(|(m, _)| m.team != team)
                .fold(0, |mask, (_, p)| mask | p[phase]);
            for (mage, positions) in live.iter().zip(&mut possible) {
                let mut next = positions[phase];
                if mage.team == team {
                    let destinations = neighbours(next, diagonal) & passable;
                    if mage
                        .spell
                        .pattern
                        .iter()
                        .any(|offset| shifted(destinations, *offset) & enemies != 0)
                    {
                        return ContactReachability::Possible;
                    }
                    next = destinations
                        | if counts[usize::from(team == Team::Blue)] > 1 {
                            next
                        } else {
                            0
                        };
                }
                let next_phase = (phase + 1) % 4;
                changed |= next & !positions[next_phase] != 0;
                positions[next_phase] |= next;
            }
        }
        if !changed {
            return ContactReachability::Impossible;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bit_shifts_match_coordinates_at_every_board_edge() {
        for y in 0..8 {
            for x in 0..8 {
                for dy in -8..=8 {
                    for dx in -8..=8 {
                        let (tx, ty) = (x + dx, y + dy);
                        let expected = if (0..8).contains(&tx) && (0..8).contains(&ty) {
                            square(Position(tx, ty))
                        } else {
                            0
                        };
                        assert_eq!(shifted(square(Position(x, y)), Position(dx, dy)), expected);
                    }
                }
            }
        }
    }
}
