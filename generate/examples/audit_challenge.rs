//! Re-run recorded rules (without AI) and audit repeated positions and termination reasons.
use std::{
    collections::{BTreeMap, HashSet},
    fs,
    path::{Path, PathBuf},
};

use generate::analysis::{GameResultRecord, Outcome};
use serde::Deserialize;
use serde_json::json;
use shared::*;

#[derive(Deserialize)]
struct Metadata {
    catalogue: Vec<CampaignEntry>,
    engine: String,
}
#[derive(Deserialize)]
struct Matchup {
    level: usize,
    games: Vec<GameResultRecord>,
}
fn files(root: &Path, result: &mut Vec<PathBuf>) {
    for entry in fs::read_dir(root).unwrap() {
        let path = entry.unwrap().path();
        if path.is_dir() {
            files(&path, result);
        } else if path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("matchup-")
            && path.extension().is_some_and(|x| x == "json")
        {
            result.push(path);
        }
    }
}
fn key(game: &Game) -> Vec<u8> {
    let mut mages: Vec<_> = game.iter_mages().collect();
    mages.sort_by_key(|m| m.index);
    serde_json::to_vec(&(
        game.turn_for(),
        mages,
        game.powerups().iter().collect::<Vec<_>>(),
        game.overcharge_at().is_some(),
    ))
    .unwrap()
}
fn main() {
    let root = PathBuf::from(
        std::env::args()
            .nth(1)
            .unwrap_or("assessments/campaign-challenge".into()),
    );
    let mut paths = Vec::new();
    files(&root, &mut paths);
    paths.sort();
    let mut reports = Vec::new();
    let mut total_games = 0;
    let mut total_plies = 0;
    for path in paths {
        let metadata: Metadata = serde_json::from_slice(
            &fs::read(path.parent().unwrap().join("metadata.json")).unwrap(),
        )
        .unwrap();
        let matchup: Matchup = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let entry = &metadata.catalogue[matchup.level];
        let mut repeated_games = 0;
        let mut repeated_positions = 0;
        let mut max_repeats = 0;
        let mut max_quiet = 0;
        let mut terminations = BTreeMap::<String, usize>::new();
        for record in &matchup.games {
            let mut game = Game::new(&entry.level(), !entry.tutorial).unwrap();
            let mut seen = HashSet::from([key(&game)]);
            let mut repeated = 0;
            let mut quiet = 0;
            assert_eq!(
                record.plies,
                record.replay.len(),
                "{} missing replay",
                path.display()
            );
            for turn in &record.replay {
                assert_eq!(game.turn_for(), turn.team);
                assert_eq!(game.powerups().get(&turn.turn.1).copied(), turn.pickup);
                let mut actual = game
                    .take_move(turn.turn.0, turn.turn.1)
                    .expect("recorded move must be legal");
                let mut expected = turn.damage.clone();
                actual.sort();
                expected.sort();
                assert_eq!(actual, expected);
                let mut mana = [0u32; 2];
                for mage in game.iter_mages() {
                    mana[usize::from(mage.team == Team::Blue)] += mage.mana.0 as u32;
                }
                assert_eq!(mana, turn.mana);
                repeated += usize::from(!seen.insert(key(&game)));
                quiet = if turn.pickup.is_none() && turn.damage.is_empty() {
                    quiet + 1
                } else {
                    0
                };
                max_quiet = max_quiet.max(quiet);
            }
            let outcome = match game.result() {
                Some(GameResult::Win(Team::Red)) => Outcome::Win,
                Some(GameResult::Win(Team::Blue)) => Outcome::Loss,
                Some(GameResult::Stalemate) => Outcome::Draw,
                None => Outcome::SafetyLimit,
            };
            assert_eq!(outcome, record.outcome);
            assert_eq!(game.overcharge_at(), record.overcharge_at);
            let termination = if game.legal_turns().is_empty() {
                "NoLegalMoves"
            } else if game.stalemate().0 {
                "Inactivity"
            } else {
                "SafetyLimit"
            };
            assert_eq!(termination, record.termination);
            *terminations.entry(termination.into()).or_default() += 1;
            repeated_games += usize::from(repeated > 0);
            repeated_positions += repeated;
            max_repeats = max_repeats.max(repeated);
            total_games += 1;
            total_plies += record.plies;
        }
        reports.push(json!({"path":path,"engine":metadata.engine,"games":matchup.games.len(),"repeated_games":repeated_games,"repeated_positions":repeated_positions,"max_repeated_positions":max_repeats,"max_quiet_plies":max_quiet,"terminations":terminations}));
    }
    println!(
        "{}",
        serde_json::to_string_pretty(
            &json!({"verified_games":total_games,"verified_plies":total_plies,"matchups":reports})
        )
        .unwrap()
    );
}
