use serde::Deserialize;
use shared::{Difficulty, Game, SearchLimits, SearchResult, Turn};
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/static/js/ai-client.js")]
extern "C" {
    #[wasm_bindgen(js_name = startSearch)]
    fn start_search(
        snapshot: &str,
        difficulty: &str,
        seed: &str,
        id: u32,
        revision: u32,
    ) -> JsValue;
    #[wasm_bindgen(js_name = pollSearch)]
    fn poll_search(job: &JsValue) -> Option<String>;
    #[wasm_bindgen(js_name = cancelSearch)]
    fn cancel_search(job: &JsValue);
}
#[derive(Deserialize)]
struct Reply {
    id: u32,
    revision: u32,
    #[serde(default)]
    failed: bool,
    #[serde(default)]
    selected: Option<Turn>,
    #[serde(default)]
    stats: Option<SearchResult>,
}
pub struct Pending {
    job: JsValue,
    id: u32,
    revision: u32,
    seed: u64,
}
impl Pending {
    pub fn new(game: &Game, difficulty: Difficulty, seed: u64, id: u32, revision: u32) -> Self {
        Self {
            job: start_search(
                &serde_json::to_string(game).unwrap(),
                difficulty.label(),
                &seed.to_string(),
                id,
                revision,
            ),
            id,
            revision,
            seed,
        }
    }
    pub fn poll(&self, game: &Game, revision: u32) -> Option<Option<Turn>> {
        let raw = poll_search(&self.job)?;
        let reply = serde_json::from_str::<Reply>(&raw).ok();
        if revision != self.revision {
            return Some(None);
        }
        if let Some(reply) = reply {
            if reply.id != self.id || reply.revision != revision {
                return Some(None);
            }
            let _statistics = reply.stats;
            if !reply.failed
                && reply
                    .selected
                    .map_or(game.result().is_some(), |t| game.legal_turns().contains(&t))
            {
                return Some(reply.selected);
            }
        }
        Some(
            game.search(SearchLimits::depth(0), self.seed, || false)
                .best_move(),
        )
    }
}
impl Drop for Pending {
    fn drop(&mut self) {
        cancel_search(&self.job);
    }
}

#[wasm_bindgen]
pub fn search_ai(
    snapshot: &str,
    difficulty: &str,
    seed: &str,
    now: &js_sys::Function,
) -> Result<String, JsValue> {
    let game: Game =
        serde_json::from_str(snapshot).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let difficulty = Difficulty::from_preference(difficulty);
    let seed = seed
        .parse::<u64>()
        .map_err(|_| JsValue::from_str("invalid seed"))?;
    let clock = || {
        now.call0(&JsValue::NULL)
            .ok()
            .and_then(|v| v.as_f64())
            .unwrap_or(f64::INFINITY)
    };
    let end = clock() + difficulty.milliseconds() as f64;
    let stats = game.search(SearchLimits::depth(difficulty.depth()), seed, || {
        clock() >= end
    });
    Ok(serde_json::json!({"selected": stats.select(difficulty, seed), "stats": stats}).to_string())
}
