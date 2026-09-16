//! Read-only draw diagnosis from recorded games. Does not change production rules.
use std::{
    collections::{BTreeMap, HashMap, HashSet, VecDeque},
    fs,
    path::PathBuf,
};

use generate::analysis::{GameResultRecord, Outcome};
use rayon::prelude::*;
use serde::Deserialize;
use serde_json::{json, Value};
use shared::*;

#[derive(Deserialize)]
struct Metadata {
    catalogue: Vec<CampaignEntry>,
}
#[derive(Deserialize)]
struct Matchup {
    level: usize,
    red: usize,
    blue: usize,
    games: Vec<GameResultRecord>,
}

// Exclude only the inactivity clock/history. Include side, powers, sleepers and
// whether the one-time overcharge has been consumed. Preserve the authoritative
// shielding cache too; do not assume it can safely be reconstructed. Geometry is
// fixed per game.
fn position_key(game: &Game) -> Vec<u8> {
    let mut shields: Vec<(Position, Team)> =
        serde_json::from_value(serde_json::to_value(game).unwrap()["shielded_positions"].clone())
            .unwrap();
    shields.sort_by_key(|(position, team)| (*position, usize::from(*team == Team::Blue)));
    serde_json::to_vec(&(
        game.iter_mages().collect::<Vec<_>>(),
        game.powerups().iter().collect::<Vec<_>>(),
        game.turn_for(),
        game.overcharge_at().is_some(),
        shields,
    ))
    .unwrap()
}

fn resume_clock(game: &Game) -> Game {
    let mut value = serde_json::to_value(game).unwrap();
    value["last_nominal"] = json!(game.turns() + 1000);
    serde_json::from_value(value).unwrap()
}

fn opportunities(game: &Game) -> [usize; 3] {
    let mut counts = [0; 3];
    for turn in game.legal_turns() {
        let mut next = game.clone();
        let hits = next.take_move(turn.0, turn.1).unwrap();
        let enemy_hit = hits
            .iter()
            .any(|p| next.occupant(p).unwrap().team != game.turn_for());
        let own_hit = hits
            .iter()
            .any(|p| next.occupant(p).unwrap().team == game.turn_for());
        counts[0] += usize::from(!hits.is_empty());
        counts[1] += usize::from(enemy_hit);
        counts[2] += usize::from(enemy_hit && !own_hit);
    }
    counts
}

fn verify_witness(game: &Game, path: &[Turn]) {
    let mut game = resume_clock(game);
    for (i, turn) in path.iter().enumerate() {
        let activation = game.overcharge_at();
        let hits = game.take_move(turn.0, turn.1).expect("legal witness move");
        assert_eq!(!hits.is_empty(), i + 1 == path.len());
        if game.overcharge_at() != activation {
            game = resume_clock(&game);
        }
    }
}

// Exact cooperative reachability using real moves/pickups/shields/overcharge.
// A found path proves combat is possible; it does NOT show it can be forced.
fn next_damage(game: &Game) -> Value {
    const CAP: usize = 50_000;
    const DEPTH: usize = 64;
    let root = resume_clock(game);
    let mut seen = HashSet::from([position_key(&root)]);
    let mut queue = VecDeque::from([(root, Vec::<Turn>::new())]);
    let mut edges = 0;
    let mut cutoff = false;
    while let Some((node, path)) = queue.pop_front() {
        if node.result().is_some() {
            continue;
        }
        for turn in node.legal_turns() {
            if edges >= CAP {
                return json!({"status":"Budget", "edges":edges});
            }
            edges += 1;
            let mut child = node.clone();
            let hits = child.take_move(turn.0, turn.1).unwrap();
            let mut child_path = path.clone();
            child_path.push(*turn);
            if !hits.is_empty() {
                verify_witness(game, &child_path);
                let enemy_hit = hits
                    .iter()
                    .any(|p| child.occupant(p).unwrap().team != node.turn_for());
                return json!({"status":"Reachable", "plies":child_path.len(), "path":child_path, "edges":edges, "damages_enemy":enemy_hit});
            }
            // A later overcharge also resets the real clock. Continue suppressing
            // that clock without suppressing the overcharge rules themselves.
            if child.overcharge_at() != node.overcharge_at() {
                child = resume_clock(&child);
            }
            if seen.insert(position_key(&child)) {
                if child_path.len() < DEPTH {
                    queue.push_back((child, child_path));
                } else {
                    cutoff = true;
                }
            }
        }
    }
    json!({"status":if cutoff {"BeyondDepthOrImpossible"} else {"Impossible"}, "edges":edges})
}

fn diagnose(level: &Level, record: &GameResultRecord, enabled: bool) -> Value {
    let mut game = Game::new_with_overcharge(level, true, enabled).unwrap();
    let mut counts = HashMap::<Vec<u8>, usize>::new();
    *counts.entry(position_key(&game)).or_default() += 1;
    let mut last_eight = Vec::new();
    for (i, step) in record.replay.iter().enumerate() {
        if i >= record.plies.saturating_sub(8) {
            last_eight.push(opportunities(&game));
        }
        assert_eq!(
            game.take_move(step.turn.0, step.turn.1).unwrap(),
            step.damage
        );
        *counts.entry(position_key(&game)).or_default() += 1;
    }
    assert!(matches!(game.result(), Some(GameResult::Stalemate)));
    assert_eq!(game.turns(), record.plies);
    assert_eq!(game.overcharge_at(), record.overcharge_at);
    let mut final_level = level.clone();
    final_level.mages = game.iter_mages().cloned().collect();
    final_level.powerups = game.powerups().clone();
    let resumed = resume_clock(&game);
    let live = game
        .iter_mages()
        .filter(|m| m.is_alive())
        .collect::<Vec<_>>();
    let last_damage = record
        .replay
        .iter()
        .rposition(|r| !r.damage.is_empty())
        .map(|i| i + 1);
    let last_pickup = record
        .replay
        .iter()
        .rposition(|r| r.pickup.is_some())
        .map(|i| i + 1);
    json!({
        "trial":record.trial, "termination":record.termination, "plies":record.plies,
        "side_to_move":game.turn_for(),
        "live":[live.iter().filter(|m|m.team==Team::Red).count(),live.iter().filter(|m|m.team==Team::Blue).count()],
        "held":live.iter().filter_map(|m|m.powerup).collect::<Vec<_>>(),
        "remaining_props":game.powerups().values().collect::<Vec<_>>(),
        "overcharge_at":game.overcharge_at(),
        "max_position_occurrences":counts.values().max(),
        "final_position_occurrences":counts[&position_key(&game)],
        "last_damage":last_damage, "last_pickup":last_pickup,
        "quiet_plies":record.plies-last_damage.unwrap_or(0),
        "last_eight_opportunities":last_eight,
        "final_opportunities":if resumed.result().is_none(){opportunities(&resumed)}else{[0;3]},
        "contact_status":format!("{:?}",contact_reachability(&final_level,game.turn_for(),false)),
        "next_damage":next_damage(&game),
        "final_level":final_level,
    })
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
    let mut paths = fs::read_dir(&root)
        .unwrap()
        .map(|e| e.unwrap().path())
        .filter(|p| {
            p.file_name()
                .unwrap()
                .to_string_lossy()
                .starts_with("matchup-")
        })
        .collect::<Vec<_>>();
    paths.sort();
    let draws = paths
        .par_iter()
        .flat_map_iter(|path| {
            let cell: Matchup = serde_json::from_slice(&fs::read(path).unwrap()).unwrap();
            let scenario = &meta.catalogue[cell.level];
            let level = scenario.level();
            cell.games
                .iter()
                .filter(|g| g.outcome == Outcome::Draw)
                .map(|record| {
                    let mut result = diagnose(&level, record, enabled);
                    result["scenario"] = json!(scenario.id);
                    result["red"] = json!(cell.red);
                    result["blue"] = json!(cell.blue);
                    result
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    let mut terminations = BTreeMap::<String, usize>::new();
    for draw in &draws {
        *terminations
            .entry(draw["termination"].as_str().unwrap().into())
            .or_default() += 1;
    }
    println!("{}",serde_json::to_string_pretty(&json!({
        "dataset":root,"overcharge_enabled":enabled,"terminations":terminations,"draws":draws,
        "method":"Authoritative replay; repeated position includes side/powers/props/sleepers/overcharge/shield cache. Cooperative shortest-damage BFS suppresses the inactivity clock, at most 64 plies and 50000 edges per draw. Opportunities=[any damage, enemy damage, enemy damage without own damage]. Reachable does not imply forceable or advantageous; Impossible means exhaustive state-graph exhaustion."
    })).unwrap());
}
