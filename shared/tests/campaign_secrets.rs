use std::{collections::HashSet, time::Duration};

use shared::*;
fn entry(id: &str) -> CampaignEntry {
    campaign_catalogue(false)
        .into_iter()
        .find(|e| e.id == id)
        .unwrap()
}
fn forced_red_win(game: &Game, plies: usize) -> bool {
    if let Some(result) = game.result() {
        return result == GameResult::Win(Team::Red);
    }
    if plies == 0 {
        return false;
    }
    let outcomes = game.legal_turns().iter().map(|t| {
        let mut next = game.clone();
        next.take_move(t.0, t.1);
        forced_red_win(&next, plies - 1)
    });
    if game.turn_for() == Team::Red {
        outcomes.into_iter().any(|w| w)
    } else {
        outcomes.into_iter().all(|w| w)
    }
}
#[test]
fn crossfire_has_one_immediate_double_elimination() {
    let level = entry("crossfire").level();
    let game = Game::new(&level, true).unwrap();
    let wins: Vec<_> = game
        .legal_turns()
        .iter()
        .copied()
        .filter(|t| {
            let mut next = game.clone();
            next.take_move(t.0, t.1);
            next.result() == Some(GameResult::Win(Team::Red))
        })
        .collect();
    assert_eq!(wins, vec![Turn(Position(0, 1), Position(1, 1))]);
    let mut without_beam = level.clone();
    without_beam.powerups.clear();
    let mut game = Game::new(&without_beam, true).unwrap();
    game.take_move(wins[0].0, wins[0].1);
    assert!(game.result() != Some(GameResult::Win(Team::Red)));
}
#[test]
fn side_step_requires_the_diagonal_pickup_for_its_five_ply_solution() {
    let level = entry("side-step").level();
    let game = Game::new(&level, true).unwrap();
    let wins: Vec<_> = game
        .legal_turns()
        .iter()
        .copied()
        .filter(|t| {
            let mut next = game.clone();
            next.take_move(t.0, t.1);
            forced_red_win(&next, 4)
        })
        .collect();
    assert_eq!(wins, vec![Turn(Position(0, 0), Position(1, 0))]);
    let mut plain = level.clone();
    plain.powerups.clear();
    assert!(!forced_red_win(&Game::new(&plain, true).unwrap(), 5));
}
#[test]
fn chaos_is_seeded_four_by_four_with_a_stable_completion_key() {
    let e = entry("ascension-iii");
    let key = e.level().as_code();
    let mut codes = HashSet::new();
    for seed in 0..12 {
        let settings = LobbySettings {
            seed,
            loadout_method: LoadoutMethod::ArenaChaos(e.level(), e.position),
            ..Default::default()
        };
        assert_eq!(
            settings.loadout_method.campaign_progress_code(),
            Some(key.clone())
        );
        let lobby = Lobby::new(settings.clone(), Duration::ZERO);
        assert_eq!(lobby.game.board_size(), (4, 4));
        assert!(lobby.game.powerups().is_empty());
        assert!(matches!(lobby.game.board().style, BoardStyle::Grass));
        for team in [Team::Red, Team::Blue] {
            assert_eq!(
                lobby.game.iter_mages().filter(|m| m.team == team).count(),
                4
            );
        }
        assert_eq!(
            lobby.game.prototype_code(),
            Lobby::new(settings, Duration::ZERO).game.prototype_code()
        );
        codes.insert(lobby.game.prototype_code());
    }
    assert!(codes.len() > 1);
    assert!(campaign_catalogue(false)
        .iter()
        .filter(|e| e.id.starts_with("ascension-"))
        .all(|e| matches!(e.style, BoardStyle::Grass)));
}

#[test]
fn diagonals_intro_breaks_an_x_mage_impasse_and_preserves_earned_stars() {
    let entry = entry("diagonals-i");
    let level = entry.level();
    assert_eq!(level.mages.len(), 2);
    assert!(level
        .mages
        .iter()
        .all(|m| matches!(m.sort, MageSort::Cross)));
    let game = Game::new(&level, true).unwrap();
    assert!(forced_red_win(&game, 5));
    // Every first move is harmless: the player has to use the new movement.
    for t in game.legal_turns() {
        let mut next = game.clone();
        assert!(next.take_move(t.0, t.1).unwrap().is_empty());
    }
    let mut plain = level.clone();
    plain.powerups.clear();
    assert!(!forced_red_win(&Game::new(&plain, true).unwrap(), 7));
    // With only cardinal moves, both X mages share a square colour before
    // Red's turn. Red changes colour and cannot land a diagonal attack.
    fn no_red_hits(game: &Game, depth: usize) {
        if depth == 0 || game.result().is_some() {
            return;
        }
        for t in game.legal_turns() {
            let mut next = game.clone();
            let hits = next.take_move(t.0, t.1).unwrap();
            if game.turn_for() == Team::Red {
                assert!(hits.is_empty());
            }
            no_red_hits(&next, depth - 1);
        }
    }
    no_red_hits(&Game::new(&plain, true).unwrap(), 7);
    assert!(campaign_progress_aliases(&level.as_code()).contains(&"d010812ncm23809408"));
}
