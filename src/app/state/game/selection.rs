use shared::{Game, Mage, Team};

/// Remember each local player's mage without restoring a destination or gesture.
#[derive(Default)]
pub(super) struct SelectionMemory {
    mages: [Option<usize>; 2],
    restored_turn: Option<usize>,
}

impl SelectionMemory {
    pub fn remember(&mut self, mage: &Mage) {
        self.mages[team_slot(mage.team)] = Some(mage.index);
    }

    pub fn suppress_turn(&mut self, turn: usize) {
        self.restored_turn = Some(turn);
    }

    /// Called only once gameplay is available, after playback and pending updates.
    pub fn restore(&mut self, game: &Game) -> Option<usize> {
        if self.restored_turn == Some(game.turns()) {
            return None;
        }
        self.restored_turn = Some(game.turns());
        let team = game.turn_for();
        let index = self.mages[team_slot(team)]?;
        game.get_mage(index)
            .filter(|mage| mage.is_alive() && mage.team == team)
            .map(|mage| mage.index)
    }
}

fn team_slot(team: Team) -> usize {
    match team {
        Team::Red => 0,
        Team::Blue => 1,
    }
}

#[cfg(test)]
mod tests {
    use shared::{Level, MageSort, Position};

    use super::*;

    #[test]
    fn restores_each_players_mage_once_per_turn_and_skips_sleepers() {
        let mut level = Level::default();
        level.board = shared::Board::new(8, 8).unwrap();
        level.mages = vec![
            Mage::new(0, Team::Red, MageSort::Plus, Position(1, 1)),
            Mage::new(1, Team::Blue, MageSort::Plus, Position(6, 6)),
        ];
        let mut game = Game::new(&level, false).unwrap();
        let mut memory = SelectionMemory::default();
        assert_eq!(memory.restore(&game), None);
        memory.remember(game.get_mage(0).unwrap());
        assert!(game.take_move(Position(1, 1), Position(2, 1)).is_some());
        assert_eq!(memory.restore(&game), None);
        memory.remember(game.get_mage(1).unwrap());
        assert!(game.take_move(Position(6, 6), Position(5, 6)).is_some());
        assert_eq!(memory.restore(&game), Some(0));
        assert_eq!(memory.restore(&game), None); // A deliberate deselection stays deselected.
        assert!(game.take_move(Position(2, 1), Position(1, 1)).is_some());
        assert_eq!(memory.restore(&game), Some(1));
        assert!(game.take_move(Position(5, 6), Position(6, 6)).is_some());
        game.get_mage_mut(0).unwrap().mana.0 = 0;
        assert_eq!(memory.restore(&game), None);
        let rewind = game.rewind(2);
        memory.suppress_turn(rewind.turns());
        assert_eq!(memory.restore(&rewind), None);
    }
}
