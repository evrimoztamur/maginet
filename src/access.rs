//! Mobile access is supplied by the native bridge, never persisted in web storage.
#[cfg(feature = "mobile")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "mobile")]
#[wasm_bindgen(module = "/static/js/mobile-access.js")]
extern "C" {
    fn owned() -> bool;
    #[wasm_bindgen(js_name = reviewing)]
    fn native_reviewing() -> bool;
    fn request(action: &str);
    fn inactive() -> bool;
    #[wasm_bindgen(js_name = takeFailure)]
    fn take_failure() -> bool;
}
pub fn demo() -> bool {
    #[cfg(feature = "mobile")]
    {
        !owned()
    }
    #[cfg(not(feature = "mobile"))]
    {
        cfg!(feature = "demo")
    }
}
pub fn purchase(action: &str) {
    #[cfg(feature = "mobile")]
    request(action);
    #[cfg(not(feature = "mobile"))]
    let _ = action;
}
pub fn online() -> bool {
    if demo() {
        purchase("purchase");
        false
    } else {
        true
    }
}
pub fn backgrounded() -> bool {
    #[cfg(feature = "mobile")]
    {
        inactive()
    }
    #[cfg(not(feature = "mobile"))]
    {
        false
    }
}

pub fn network_failed() -> bool {
    #[cfg(feature = "mobile")]
    {
        take_failure()
    }
    #[cfg(not(feature = "mobile"))]
    {
        false
    }
}

// Reviewer shortcuts never write campaign stars or purchase state.
thread_local! {
    static REVIEW_LEVELS: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

pub fn reviewing() -> bool {
    #[cfg(feature = "mobile")]
    {
        native_reviewing()
    }
    #[cfg(not(feature = "mobile"))]
    {
        false
    }
}

pub fn review_levels_unlocked() -> bool {
    if !reviewing() {
        REVIEW_LEVELS.with(|unlocked| unlocked.set(false));
        return false;
    }
    REVIEW_LEVELS.with(|unlocked| unlocked.get())
}

pub fn unlock_review_levels() {
    if reviewing() {
        REVIEW_LEVELS.with(|unlocked| unlocked.set(true));
    }
}
