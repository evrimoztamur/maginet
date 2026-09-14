use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};
use shared::{Difficulty, Game, GameResult, Level, SearchLimits, Team};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AgentSettings {
    pub difficulty: Difficulty,
    pub depth: usize,
    pub nodes: u64,
    pub weights: [u64; 3],
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RunConfig {
    pub version: u32,
    pub seed_namespace: Option<String>,
    pub red_profile: Option<usize>,
    pub blue_profile: Option<usize>,
    pub replays: bool,
    pub games: usize,
    pub seed: u64,
    pub max_plies: usize,
    pub profiles: Vec<AgentSettings>,
    pub table_capacity: usize,
}
impl Default for RunConfig {
    fn default() -> Self {
        Self {
            version: 2,
            seed_namespace: None,
            red_profile: None,
            blue_profile: None,
            replays: false,
            games: 30,
            seed: 1,
            max_plies: 200,
            table_capacity: 32768,
            profiles: [Difficulty::Easy, Difficulty::Normal, Difficulty::Hard]
                .into_iter()
                .zip([1000, 5000, 20000])
                .map(|(difficulty, nodes)| AgentSettings {
                    difficulty,
                    depth: difficulty.depth(),
                    nodes,
                    weights: difficulty.weights(),
                })
                .collect(),
        }
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Outcome {
    Win,
    Loss,
    Draw,
    SafetyLimit,
}
#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]
pub struct Telemetry {
    pub nodes: u64,
    pub searches: usize,
    pub fallbacks: usize,
    pub depths: BTreeMap<usize, usize>,
    pub stops: BTreeMap<String, usize>,
}
impl Telemetry {
    fn add(&mut self, other: &Self) {
        self.nodes += other.nodes;
        self.searches += other.searches;
        self.fallbacks += other.fallbacks;
        for (k, v) in &other.depths {
            *self.depths.entry(*k).or_default() += v;
        }
        for (k, v) in &other.stops {
            *self.stops.entry(k.clone()).or_default() += v;
        }
    }
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GameResultRecord {
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub replay: Vec<ReplayMove>,
    pub termination: String,
    pub trial: usize,
    pub seed: u64,
    pub outcome: Outcome,
    pub plies: usize,
    pub red: Telemetry,
    pub blue: Telemetry,
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ReplayMove {
    pub turn: shared::Turn,
    pub team: Team,
    pub pickup: Option<shared::PowerUp>,
    pub damage: Vec<shared::Position>,
    pub mana: [u32; 2],
}
impl RunConfig {
    pub fn trial_seed(&self, level: &Level, trial: usize) -> u64 {
        trial_seed(
            self.seed,
            self.seed_namespace.as_deref().unwrap_or(&level.as_code()),
            trial,
        )
    }
}
// FNV-1a with explicit byte order: independent of platform, scheduling and matchup.
pub fn trial_seed(base: u64, code: &str, trial: usize) -> u64 {
    base.to_le_bytes()
        .into_iter()
        .chain(code.bytes())
        .chain((trial as u64).to_le_bytes())
        .fold(0xcbf29ce484222325, |h, b| {
            (h ^ b as u64).wrapping_mul(0x100000001b3)
        })
}
pub fn simulate(
    level: &Level,
    stalemates: bool,
    config: &RunConfig,
    red: usize,
    blue: usize,
    trial: usize,
) -> GameResultRecord {
    let seed = config.trial_seed(level, trial);
    let mut game = Game::new(level, stalemates).unwrap();
    let mut record = GameResultRecord {
        replay: Vec::new(),
        termination: String::new(),
        trial,
        seed,
        outcome: Outcome::SafetyLimit,
        plies: 0,
        red: Telemetry::default(),
        blue: Telemetry::default(),
    };
    while game.result().is_none() && game.turns() < config.max_plies {
        let (profile, stats) = if game.turn_for() == Team::Red {
            (&config.profiles[red], &mut record.red)
        } else {
            (&config.profiles[blue], &mut record.blue)
        };
        let move_seed = game.history_seed(seed);
        let result = game.search(
            SearchLimits {
                max_depth: profile.depth,
                max_nodes: Some(profile.nodes),
                table_capacity: config.table_capacity,
            },
            move_seed,
            || false,
        );
        stats.nodes += result.visited_nodes;
        stats.searches += 1;
        stats.fallbacks += usize::from(result.moves.is_empty());
        *stats.depths.entry(result.completed_depth).or_default() += 1;
        *stats
            .stops
            .entry(format!("{:?}", result.stop_reason))
            .or_default() += 1;
        let turn = result
            .select(profile.difficulty, move_seed)
            .expect("nonterminal legal move");
        let team = game.turn_for();
        let pickup = game.powerups().get(&turn.1).copied();
        let damage = game.take_move(turn.0, turn.1).expect("legal selected move");
        if config.replays {
            let mut mana = [0, 0];
            for mage in game.iter_mages() {
                mana[usize::from(mage.team == Team::Blue)] += mage.mana.0 as u32;
            }
            record.replay.push(ReplayMove {
                turn,
                team,
                pickup,
                damage,
                mana,
            });
        }
    }
    record.termination = if game.legal_turns().is_empty() {
        "NoLegalMoves"
    } else if game.stalemate().0 {
        "Inactivity"
    } else {
        "SafetyLimit"
    }
    .into();
    record.plies = game.turns();
    record.outcome = match game.result() {
        Some(GameResult::Win(Team::Red)) => Outcome::Win,
        Some(GameResult::Win(Team::Blue)) => Outcome::Loss,
        Some(GameResult::Stalemate) => Outcome::Draw,
        None => Outcome::SafetyLimit,
    };
    record
}
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Aggregate {
    pub wins: usize,
    pub losses: usize,
    pub draws: usize,
    pub unresolved: usize,
    pub resolved_win_rate: Option<f64>,
    pub wilson95: Option<[f64; 2]>,
    pub overall_win_range: [f64; 2],
    pub mean_plies: f64,
    pub min_plies: usize,
    pub max_plies: usize,
    pub red: Telemetry,
    pub blue: Telemetry,
}
pub fn wilson(wins: usize, n: usize) -> Option<[f64; 2]> {
    if n == 0 {
        return None;
    }
    let n = n as f64;
    let p = wins as f64 / n;
    let z2 = 1.959963984540054_f64.powi(2);
    let center = (p + z2 / (2.0 * n)) / (1.0 + z2 / n);
    let half = (z2 * (p * (1.0 - p) / n + z2 / (4.0 * n * n))).sqrt() / (1.0 + z2 / n);
    Some([(center - half).max(0.0), (center + half).min(1.0)])
}
pub fn aggregate(games: &[GameResultRecord]) -> Aggregate {
    assert!(!games.is_empty());
    let count = |o| games.iter().filter(|g| g.outcome == o).count();
    let (wins, losses, draws, unresolved) = (
        count(Outcome::Win),
        count(Outcome::Loss),
        count(Outcome::Draw),
        count(Outcome::SafetyLimit),
    );
    let n = wins + losses + draws;
    let mut red = Telemetry::default();
    let mut blue = Telemetry::default();
    for g in games {
        red.add(&g.red);
        blue.add(&g.blue);
    }
    Aggregate {
        wins,
        losses,
        draws,
        unresolved,
        resolved_win_rate: (n > 0).then(|| wins as f64 / n as f64),
        wilson95: wilson(wins, n),
        overall_win_range: [
            wins as f64 / games.len() as f64,
            (wins + unresolved) as f64 / games.len() as f64,
        ],
        mean_plies: games.iter().map(|g| g.plies).sum::<usize>() as f64 / games.len() as f64,
        min_plies: games.iter().map(|g| g.plies).min().unwrap(),
        max_plies: games.iter().map(|g| g.plies).max().unwrap(),
        red,
        blue,
    }
}

#[cfg(test)]
mod tests {
    use shared::{Board, Mage, Position, PowerUp};

    use super::*;
    #[test]
    fn terminal_roots_and_safety_limit() {
        let config = RunConfig {
            max_plies: 0,
            ..Default::default()
        };
        for (team, outcome) in [(Team::Red, Outcome::Win), (Team::Blue, Outcome::Loss)] {
            let level =
                Level::default_with_mages(vec![Mage::new(0, team, 0.into(), Position(0, 0))]);
            // The opposing side has no move, making these actual terminal roots.
            let mut level = level;
            level.starting_team = team.enemy();
            let result = simulate(&level, true, &config, 0, 0, 0);
            assert_eq!(result.outcome, outcome);
            assert_eq!(result.plies, 0);
        }
        assert_eq!(
            simulate(&Level::default_with_mages(vec![]), true, &config, 0, 0, 0).outcome,
            Outcome::Draw
        );
        let level = shared::campaign_catalogue(false)[0].level();
        assert_eq!(
            simulate(&level, true, &config, 0, 0, 0).outcome,
            Outcome::SafetyLimit
        );
    }
    #[test]
    fn fallback_both_teams_powerups_and_repeatability() {
        let mut config = RunConfig {
            games: 4,
            max_plies: 4,
            ..Default::default()
        };
        for p in &mut config.profiles {
            p.nodes = 0;
        }
        for team in [Team::Red, Team::Blue] {
            let level = Level::new(
                Board::new(4, 4).unwrap(),
                vec![
                    Mage::new(0, Team::Red, 0.into(), Position(0, 0)),
                    Mage::new(1, Team::Blue, 0.into(), Position(3, 3)),
                ],
                [
                    (Position(1, 0), PowerUp::Diagonal),
                    (Position(2, 3), PowerUp::Shield),
                ]
                .into(),
                team,
            );
            let result = simulate(&level, false, &config, 0, 0, 1);
            assert_eq!(result, simulate(&level, false, &config, 0, 0, 1));
            assert_eq!(result.red.fallbacks + result.blue.fallbacks, result.plies);
            assert_eq!(result.red.nodes + result.blue.nodes, 0);
            assert_eq!(result.red.searches, 2);
            assert_eq!(result.blue.searches, 2);
        }
    }
    #[test]
    fn resolved_denominator_and_intervals() {
        assert_eq!(wilson(0, 0), None);
        let ci = wilson(15, 30).unwrap();
        assert!((ci[0] - 0.33154).abs() < 0.0001);
        assert!((ci[1] - 0.66846).abs() < 0.0001);
        let mut games = vec![];
        for outcome in [
            Outcome::Win,
            Outcome::Loss,
            Outcome::Draw,
            Outcome::SafetyLimit,
        ] {
            games.push(GameResultRecord {
                replay: Vec::new(),
                termination: String::new(),
                trial: 0,
                seed: 1,
                outcome,
                plies: 4,
                red: Telemetry::default(),
                blue: Telemetry::default(),
            });
        }
        let a = aggregate(&games);
        assert_eq!(a.resolved_win_rate, Some(1.0 / 3.0));
        assert_eq!(a.overall_win_range, [0.25, 0.5]);
        let a = aggregate(&games[3..]);
        assert_eq!(a.resolved_win_rate, None);
        assert_eq!(a.wilson95, None);
        assert_eq!(a.overall_win_range, [0.0, 1.0]);
    }
}
