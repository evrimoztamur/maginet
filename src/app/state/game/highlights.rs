use shared::{Game, Mage, Position, PowerUp};

/// Pattern study includes tiles outside the board. Only a legal destination
/// preview can claim damage; actual hits still come from the shared rules.
pub(super) fn attack_highlights(
    game: &Game,
    mage: &Mage,
    destination: Option<Position>,
) -> Vec<(bool, Position)> {
    let destination = destination.filter(|at| {
        game.legal_turns()
            .contains(&shared::Turn(mage.position, *at))
    });
    let at = destination.unwrap_or(mage.position);
    let targets = game.targets(mage, at);
    let beam = mage.powerup == Some(PowerUp::Beam)
        || (destination.is_some() && game.powerups().get(&at) == Some(&PowerUp::Beam));
    let mut tiles: Vec<_> = if beam {
        // Continue the row and column beyond the board edge for the preview.
        let (width, height) = game.board_size();
        (-2..width as i8 + 2)
            .map(|x| Position(x, at.1))
            .chain((-2..height as i8 + 2).map(|y| Position(at.0, y)))
            .filter(|tile| *tile != at)
            .collect()
    } else {
        mage.spell.pattern.iter().map(|dir| &at + dir).collect()
    };
    if destination.is_some() {
        for (_, tile) in &targets {
            if !tiles.contains(tile) {
                tiles.push(*tile);
            }
        }
    }
    tiles
        .into_iter()
        .map(|tile| {
            (
                destination.is_some() && targets.contains(&(true, tile)),
                tile,
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use shared::{Board, BoulderStyle, Level, MageSort, Team};

    use super::*;

    #[test]
    fn idle_pattern_includes_sleepers_stones_and_off_board_without_damage() {
        let mut level = Level::default();
        level.board = Board::new(4, 4).unwrap();
        level.mages = vec![
            Mage::new(0, Team::Red, MageSort::Cross, Position(1, 1)),
            Mage::new(1, Team::Blue, MageSort::Plus, Position(2, 2)),
            Mage::new(2, Team::Blue, MageSort::Plus, Position(3, 3)),
        ];
        level.mages[2].mana.0 = 0;
        level
            .powerups
            .insert(Position(0, 2), PowerUp::Boulder(BoulderStyle::Rock));
        let game = Game::new(&level, false).unwrap();
        let mage = game.get_mage(0).unwrap();
        let idle = attack_highlights(&game, mage, None);
        assert_eq!(idle.len(), 8);
        for tile in [
            Position(-1, -1),
            Position(2, 2),
            Position(3, 3),
            Position(0, 2),
        ] {
            assert!(idle.contains(&(false, tile)));
        }
        assert!(idle.iter().all(|(damage, _)| !damage));
        let preview = attack_highlights(&game, mage, Some(Position(0, 1)));
        assert!(preview.contains(&(false, Position(2, -1))));
        let plus = game.get_mage(1).unwrap();
        assert!(attack_highlights(&game, plus, None).contains(&(false, Position(2, 0))));
        // An opponent can be inspected, but cannot obtain a move/damage preview.
        assert!(attack_highlights(&game, plus, Some(Position(1, 0)))
            .iter()
            .all(|(hit, _)| !hit));
    }

    #[test]
    fn destination_damage_uses_rules_including_beams_and_shield_retaliation() {
        let mut level: Level = shared::TUTORIAL_CODE.into();
        level.mages = vec![
            Mage::new(0, Team::Red, MageSort::Plus, Position(1, 1)),
            Mage::new(1, Team::Blue, MageSort::Cross, Position(3, 2)),
        ];
        level.mages[1].powerup = Some(PowerUp::Shield);
        level.powerups.clear();
        level.powerups.insert(Position(2, 1), PowerUp::Beam);
        let game = Game::new(&level, false).unwrap();
        let mage = game.get_mage(0).unwrap();
        let at = Position(2, 1);
        let highlights = attack_highlights(&game, mage, Some(at));
        let targets = game.targets(mage, at);
        for (_, tile) in &targets {
            assert!(highlights.contains(&(targets.contains(&(true, *tile)), *tile)));
        }
        assert!(highlights.contains(&(true, at)));
        assert!(highlights.contains(&(false, Position(-2, 1))));
    }
}
