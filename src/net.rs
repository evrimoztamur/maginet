use futures::TryFutureExt;
use js_sys::Promise;
use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{Request, Response};

fn wrap_response_into_json(value: JsValue) -> JsFuture {
    assert!(value.is_instance_of::<Response>());
    let resp: Response = value.dyn_into().unwrap();
    JsFuture::from(resp.json().unwrap())
}

pub fn fetch_with_callback(request: &Request, callback: &Closure<dyn FnMut(JsValue)>) -> Promise {
    let resp_value = JsFuture::from(web_sys::window().unwrap().fetch_with_request(request))
        .and_then(wrap_response_into_json);

    let promise = future_to_promise(resp_value);
    promise.then(callback);

    promise
}
