//! Replays an assessment through the authoritative rules, including activation timing.
use std::{collections::BTreeMap, fs, path::PathBuf};

use generate::analysis::{GameResultRecord, Outcome, RunConfig};
use serde::Deserialize;
use shared::*;

#[derive(Deserialize)]
struct Metadata {
    config: RunConfig,
    catalogue: Vec<CampaignEntry>,
}
#[derive(Deserialize)]
struct Matchup {
    level: usize,
    red: usize,
    blue: usize,
    games: Vec<GameResultRecord>,
}

fn main() {
    let mut args = std::env::args().skip(1);
    let root = PathBuf::from(args.next().expect("dataset directory"));
    let enabled = match args.next().as_deref() {
        Some("before") => false,
        Some("after") => true,
        _ => panic!("before|after"),
    };
    let meta: Metadata =
        serde_json::from_slice(&fs::read(root.join("metadata.json")).unwrap()).unwrap();
    let expected: usize = meta
        .catalogue
        .iter()
        .filter(|e| !e.chaos)
        .map(|e| {
            (0..3)
                .filter(|r| meta.config.red_profile.is_none_or(|p| p == *r))
                .count()
                * (0..if e.tutorial { 1 } else { 3 })
                    .filter(|b| meta.config.blue_profile.is_none_or(|p| p == *b))
                    .count()
        })
        .sum();
    let mut cells = 0;
    let mut checked = 0;
    let mut activations = 0;
    let mut inactive = BTreeMap::<String, usize>::new();
    for entry in fs::read_dir(&root).unwrap() {
        let path = entry.unwrap().path();
        if !path
            .file_name()
            .unwrap()
            .to_string_lossy()
            .starts_with("matchup-")
        {
            continue;
        }
        let cell: Matchup = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        let scenario = &meta.catalogue[cell.level];
        let level = scenario.level();
        assert_eq!(cell.games.len(), meta.config.games);
        cells += 1;
        for (trial, record) in cell.games.iter().enumerate() {
            assert_eq!(record.trial, trial);
            assert_eq!(record.seed, meta.config.trial_seed(&level, trial));
            assert_eq!(record.replay.len(), record.plies);
            let mut game = Game::new_with_overcharge(&level, !scenario.tutorial, enabled).unwrap();
            let mut live_at_activation = game
                .overcharge_at()
                .map(|_| game.iter_mages().filter(|m| m.is_alive()).count());
            for step in &record.replay {
                assert_eq!(game.turn_for(), step.team);
                assert_eq!(game.powerups().get(&step.turn.1).copied(), step.pickup);
                let before = game.overcharge_at();
                assert_eq!(
                    game.take_move(step.turn.0, step.turn.1)
                        .expect("recorded move must be legal"),
                    step.damage,
                    "{} trial {}",
                    path.display(),
                    trial
                );
                assert_eq!(
                    step.overcharged,
                    before.is_none() && game.overcharge_at().is_some()
                );
                if step.overcharged {
                    live_at_activation = Some(game.iter_mages().filter(|m| m.is_alive()).count());
                }
                let mut mana = [0u32; 2];
                for mage in game.iter_mages() {
                    mana[usize::from(mage.team == Team::Blue)] += mage.mana.0 as u32;
                }
                assert_eq!(step.mana, mana);
            }
            assert_eq!(game.overcharge_at(), record.overcharge_at);
            assert_eq!(live_at_activation, record.overcharge_mages);
            activations += usize::from(record.overcharge_at.is_some());
            let outcome = match game.result() {
                Some(GameResult::Win(Team::Red)) => Outcome::Win,
                Some(GameResult::Win(Team::Blue)) => Outcome::Loss,
                Some(GameResult::Stalemate) => Outcome::Draw,
                None => Outcome::SafetyLimit,
            };
            assert_eq!(outcome, record.outcome);
            assert_eq!(
                record.termination,
                if game.legal_turns().is_empty() {
                    "NoLegalMoves"
                } else if game.stalemate().0 {
                    "Inactivity"
                } else {
                    "SafetyLimit"
                }
            );
            for (team, profile) in [(&record.red, cell.red), (&record.blue, cell.blue)] {
                assert_eq!(team.depths.values().sum::<usize>(), team.searches);
                assert_eq!(team.stops.values().sum::<usize>(), team.searches);
                assert!(team.nodes <= team.searches as u64 * meta.config.profiles[profile].nodes);
            }
            if record.termination == "Inactivity" {
                // Reconstruct the final level from the original and complete replay.
                let mut final_level = level.clone();
                final_level.mages = game.iter_mages().cloned().collect();
                final_level.powerups = game.powerups().clone();
                let status = format!(
                    "{:?}",
                    contact_reachability(&final_level, game.turn_for(), false)
                );
                *inactive.entry(status).or_default() += 1;
            }
            checked += 1;
        }
    }
    assert_eq!(cells, expected);
    println!("{}", serde_json::to_string_pretty(&serde_json::json!({"dataset":root,"overcharge_enabled":enabled,"matchups":cells,"games":checked,"activations":activations,"inactivity_contact_status":inactive})).unwrap());
}
