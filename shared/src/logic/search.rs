use std::collections::HashMap;

use rand_chacha::{
    rand_core::{RngCore, SeedableRng},
    ChaCha8Rng,
};
use serde::{Deserialize, Serialize};

use crate::{Game, Team, Turn};

/// Saved opponent strength. Probabilities are tuning defaults, not strength guarantees.
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Difficulty {
    /// Two plies, 50 ms, 65/25/10 ranked sampling.
    Easy,
    /// Four plies, 150 ms, 80/15/5 ranked sampling.
    #[default]
    Normal,
    /// Eight plies, 500 ms, 90/8/2 ranked sampling.
    Hard,
}
impl Difficulty {
    /// Parse storage, defaulting missing or invalid values to Normal.
    pub fn from_preference(value: &str) -> Self {
        match value {
            "Easy" => Self::Easy,
            "Hard" => Self::Hard,
            _ => Self::Normal,
        }
    }
    /// Stable storage and display name.
    pub fn label(self) -> &'static str {
        match self {
            Self::Easy => "Easy",
            Self::Normal => "Normal",
            Self::Hard => "Hard",
        }
    }
    /// Cycle the settings selector.
    pub fn next(self) -> Self {
        match self {
            Self::Easy => Self::Normal,
            Self::Normal => Self::Hard,
            Self::Hard => Self::Easy,
        }
    }
    /// Maximum search depth in plies.
    pub fn depth(self) -> usize {
        match self {
            Self::Easy => 2,
            Self::Normal => 4,
            Self::Hard => 8,
        }
    }
    /// Worker search budget, excluding Wasm startup.
    pub fn milliseconds(self) -> u32 {
        match self {
            Self::Easy => 50,
            Self::Normal => 150,
            Self::Hard => 500,
        }
    }
    /// Relative probabilities for the three best moves.
    pub fn weights(self) -> [u64; 3] {
        match self {
            Self::Easy => [65, 25, 10],
            Self::Normal => [80, 15, 5],
            Self::Hard => [90, 8, 2],
        }
    }
}
/// Per-search resource limits; a zero depth requests only a legal fallback.
#[derive(Clone, Copy)]
pub struct SearchLimits {
    /// Maximum completed depth.
    pub max_depth: usize,
    /// Deterministic cap on visited successor nodes.
    pub max_nodes: Option<u64>,
    /// Maximum transposition entries; zero disables caching.
    pub table_capacity: usize,
}
impl SearchLimits {
    /// Depth-limited search with a bounded table and no node cap.
    pub fn depth(max_depth: usize) -> Self {
        Self {
            max_depth,
            max_nodes: None,
            table_capacity: 32768,
        }
    }
}
/// Why search returned.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum StopReason {
    /// The root already has a result.
    Terminal,
    /// All requested iterations completed.
    Depth,
    /// Node budget exhausted.
    Nodes,
    /// Caller deadline expired.
    Deadline,
}
/// Exact root score at the published depth, positive for Red.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ScoredMove {
    /// Legal root turn.
    pub turn: Turn,
    /// Evaluation from Red's perspective.
    pub score: isize,
}
/// Only fully completed iterations are published; partial work counts toward nodes.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SearchResult {
    /// Root moves ranked best first for the active team, with seeded ties.
    pub moves: Vec<ScoredMove>,
    /// Seeded legal move when no iteration completes; absent at terminal roots.
    pub fallback: Option<Turn>,
    /// Depth shared by every published score.
    pub completed_depth: usize,
    /// Visited successor nodes, including interrupted work.
    pub visited_nodes: u64,
    /// Completion or interruption cause.
    pub stop_reason: StopReason,
}
impl SearchResult {
    /// Actual best move, independently of difficulty sampling.
    pub fn best_move(&self) -> Option<Turn> {
        self.moves.first().map(|m| m.turn).or(self.fallback)
    }
    /// Sample ranked moves, renormalizing for fewer than three, including forced outcomes.
    pub fn select(&self, difficulty: Difficulty, seed: u64) -> Option<Turn> {
        if self.moves.is_empty() {
            return self.fallback;
        }
        let count = self.moves.len().min(3);
        let weights = difficulty.weights();
        let mut draw =
            ChaCha8Rng::seed_from_u64(seed).next_u64() % weights[..count].iter().sum::<u64>();
        for (m, weight) in self.moves.iter().zip(weights) {
            if draw < weight {
                return Some(m.turn);
            }
            draw -= weight;
        }
        unreachable!()
    }
}
#[derive(Clone, Copy)]
enum Bound {
    Exact,
    Lower,
    Upper,
}
#[derive(Clone, Copy)]
struct Entry {
    depth: usize,
    score: isize,
    bound: Bound,
    best: Turn,
}
struct Search<F> {
    limits: SearchLimits,
    deadline: F,
    nodes: u64,
    table: HashMap<Vec<u8>, Entry>,
}
const INF: isize = isize::MAX / 2;
impl<F: FnMut() -> bool> Search<F> {
    fn check(&mut self) -> Result<(), StopReason> {
        if self.limits.max_nodes.is_some_and(|n| self.nodes >= n) {
            return Err(StopReason::Nodes);
        }
        if (self.deadline)() {
            return Err(StopReason::Deadline);
        }
        Ok(())
    }
    fn value(
        &mut self,
        game: &Game,
        depth: usize,
        mut alpha: isize,
        mut beta: isize,
    ) -> Result<isize, StopReason> {
        self.check()?;
        self.nodes += 1;
        if game.result().is_some() || depth == 0 {
            return Ok(game.evaluate());
        }
        let key = game.search_key();
        let cached = self.table.get(&key).copied();
        let (original_alpha, original_beta) = (alpha, beta);
        if let Some(e) = cached.filter(|e| e.depth == depth) {
            match e.bound {
                Bound::Exact => return Ok(e.score),
                Bound::Lower => alpha = alpha.max(e.score),
                Bound::Upper => beta = beta.min(e.score),
            }
            if alpha >= beta {
                return Ok(e.score);
            }
        }
        let mut turns = game.legal_turns().to_vec();
        if let Some(e) = cached {
            turns.sort_by_key(|t| *t != e.best);
        }
        let red = game.turn_for() == Team::Red;
        let mut value = if red { -INF } else { INF };
        let mut best = turns[0];
        for turn in turns {
            let mut child = game.clone();
            child.take_move(turn.0, turn.1);
            let score = self.value(&child, depth - 1, alpha, beta)?;
            if (red && score > value) || (!red && score < value) {
                value = score;
                best = turn;
            }
            if red {
                alpha = alpha.max(value);
            } else {
                beta = beta.min(value);
            }
            if alpha >= beta {
                break;
            }
        }
        let bound = if value <= original_alpha {
            Bound::Upper
        } else if value >= original_beta {
            Bound::Lower
        } else {
            Bound::Exact
        };
        if self.table.len() < self.limits.table_capacity || self.table.contains_key(&key) {
            self.table.insert(
                key,
                Entry {
                    depth,
                    score: value,
                    bound,
                    best,
                },
            );
        }
        Ok(value)
    }
}
impl Game {
    /// All root scores are exact at the same completed depth, positive for Red.
    pub fn search(
        &self,
        limits: SearchLimits,
        seed: u64,
        deadline: impl FnMut() -> bool,
    ) -> SearchResult {
        let mut result = SearchResult {
            moves: vec![],
            fallback: None,
            completed_depth: 0,
            visited_nodes: 0,
            stop_reason: StopReason::Depth,
        };
        if self.result().is_some() {
            result.stop_reason = StopReason::Terminal;
            return result;
        }
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        let mut turns = self.legal_turns().to_vec();
        for i in (1..turns.len()).rev() {
            turns.swap(i, rng.next_u64() as usize % (i + 1));
        }
        let tie_order = turns.clone();
        result.fallback = turns.first().copied();
        let mut search = Search {
            limits,
            deadline,
            nodes: 0,
            table: HashMap::new(),
        };
        'iterations: for depth in 1..=limits.max_depth {
            let mut moves = Vec::new();
            for &turn in &turns {
                if let Err(reason) = search.check() {
                    result.stop_reason = reason;
                    break 'iterations;
                }
                let mut child = self.clone();
                child.take_move(turn.0, turn.1);
                match search.value(&child, depth - 1, -INF, INF) {
                    Ok(score) => moves.push(ScoredMove { turn, score }),
                    Err(reason) => {
                        result.stop_reason = reason;
                        break 'iterations;
                    }
                }
            }
            moves.sort_by_key(|m| {
                (
                    if self.turn_for() == Team::Red {
                        -m.score
                    } else {
                        m.score
                    },
                    tie_order.iter().position(|t| *t == m.turn).unwrap(),
                )
            });
            turns = moves.iter().map(|m| m.turn).collect();
            result.moves = moves;
            result.completed_depth = depth;
        }
        result.visited_nodes = search.nodes;
        result
    }
}
