//! iOS access is supplied by the native bridge, never persisted in web storage.
#[cfg(feature = "ios")]
use wasm_bindgen::prelude::*;
#[cfg(feature = "ios")]
#[wasm_bindgen(module = "/static/js/ios-access.js")]
extern "C" {
    fn owned() -> bool;
    fn request(action: &str);
    fn inactive() -> bool;
    #[wasm_bindgen(js_name = takeFailure)]
    fn take_failure() -> bool;
}
pub fn demo() -> bool {
    #[cfg(feature = "ios")]
    {
        return !owned();
    }
    #[cfg(not(feature = "ios"))]
    {
        cfg!(feature = "demo")
    }
}
pub fn purchase(action: &str) {
    #[cfg(feature = "ios")]
    request(action);
    #[cfg(not(feature = "ios"))]
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
    #[cfg(feature = "ios")]
    {
        return inactive();
    }
    #[cfg(not(feature = "ios"))]
    {
        false
    }
}

pub fn network_failed() -> bool {
    #[cfg(feature = "ios")]
    {
        return take_failure();
    }
    #[cfg(not(feature = "ios"))]
    {
        false
    }
}
