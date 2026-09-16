use shared::*;

fn level(id: &str) -> Level {
    campaign_catalogue(false)
        .into_iter()
        .find(|entry| entry.id == id)
        .unwrap()
        .level()
}

fn opening(level: &Level) -> SearchResult {
    let result = Game::new(level, true)
        .unwrap()
        .search(SearchLimits::depth(10), 1, || false);
    assert_eq!(result.completed_depth, 10);
    result
}

#[test]
fn basics_reward_forks_coordination_then_reject_a_losing_greedy_fork() {
    for (id, correct, mistake) in [
        (
            "basics-i",
            Turn(Position(1, 2), Position(2, 2)),
            Turn(Position(1, 2), Position(1, 1)),
        ),
        (
            "basics-ii",
            Turn(Position(1, 3), Position(1, 2)),
            Turn(Position(1, 1), Position(1, 2)),
        ),
        (
            "basics-iii",
            Turn(Position(0, 3), Position(0, 2)),
            Turn(Position(1, 3), Position(1, 2)),
        ),
    ] {
        let result = opening(&level(id));
        assert_eq!(result.best_move(), Some(correct), "{id}");
        assert_eq!(
            result.moves[0].score, 99999,
            "{id} must have a forced winning line"
        );
        assert_eq!(
            result
                .moves
                .iter()
                .find(|m| m.turn == mistake)
                .unwrap()
                .score,
            -99999,
            "{id} mistake must be punishable"
        );
    }
    let mut finale = Game::new(&level("basics-iii"), true).unwrap();
    assert_eq!(
        finale
            .take_move(Position(1, 3), Position(1, 2))
            .unwrap()
            .len(),
        3
    );
    assert!(
        finale.legal_turns().iter().any(|turn| {
            let mut reply = finale.clone();
            reply.take_move(turn.0, turn.1).unwrap();
            !reply.get_mage(1).unwrap().is_alive()
        }),
        "the triple attack exposes the fragile Knight to immediate retaliation"
    );
}

#[test]
fn rune_introductions_need_their_new_mechanic_for_a_winning_line() {
    for id in ["diagonals-i", "shields-i"] {
        let mut puzzle = level(id);
        assert_eq!(opening(&puzzle).moves[0].score, 99999, "{id} with its rune");
        puzzle.powerups.clear();
        assert_eq!(
            opening(&puzzle).moves[0].score,
            -99999,
            "{id} cannot bypass its rune"
        );
    }
}

#[test]
fn revised_puzzles_preserve_completion_without_crediting_removed_basics_i() {
    for (id, old) in [
        ("basics-ii", "j0228014cm0j8v804gp04900"),
        ("basics-iii", "j022801mcm0j8v804gp04d06201g00s80dm07403g01g"),
        ("patterns-i", "dg30r0j4500m8v048g0g4h250526212400"),
        ("patterns-iii", "pg3220j4g41m8h818gr06h4g052780j400"),
        ("diagonals-i", "dg1080a4d40j409408"),
        ("shields-i", "d012g0an840k80h401200"),
        ("rite-iv", "dg30r0a45g1m8v048g0g2h210d2621240gm04h024g0mg00"),
    ] {
        let old_canonical = Level::parse_code(old).unwrap().as_code();
        assert!(
            campaign_progress_aliases(&level(id).as_code()).contains(&old_canonical.as_str()),
            "{id}"
        );
    }
    assert_eq!(level("basics-i").as_code(), "e01jg1148m0j8k834g00");
    assert!(!campaign_progress_aliases(&level("basics-i").as_code()).contains(&"hg12g014cm0j800"));
}
