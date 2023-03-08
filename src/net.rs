use futures::TryFutureExt;
use js_sys::Promise;
use shared::OutMessage;
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{Request, RequestInit, Response};

fn wrap_response_into_json(value: JsValue) -> JsFuture {
    assert!(value.is_instance_of::<Response>());
    let resp: Response = value.dyn_into().unwrap();
    JsFuture::from(resp.json().unwrap())
}

pub fn fetch(request: &Request) -> Option<Promise> {
    let resp_value = JsFuture::from(web_sys::window().unwrap().fetch_with_request(request))
        .and_then(wrap_response_into_json);

    Some(future_to_promise(resp_value))
    // None
}

pub fn send_message(message: OutMessage) -> Option<Promise> {
    if let Some(json) = serde_json::to_string(&message).ok() {
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.body(Some(&json.into()));

        let url = format!("test");

        let request = &Request::new_with_str_and_init(&url, &opts).unwrap();

        request
            .headers()
            .set("Content-Type", "application/json")
            .unwrap();

        fetch(request)
    } else {
        None
    }
}
