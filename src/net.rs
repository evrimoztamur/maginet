use futures::TryFutureExt;
use js_sys::Promise;
use shared::{Message, SessionMessage, SessionRequest};
use wasm_bindgen::{JsCast, JsValue};
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{Request, RequestInit, Response};

use crate::window;

pub struct MessagePool {
    pub messages: Vec<Message>,
    block_frame: u64,
}

impl MessagePool {
    const BLOCK_FRAMES: u64 = 60;

    pub fn new() -> MessagePool {
        MessagePool {
            messages: Vec::new(),
            block_frame: 0,
        }
    }

    pub fn available(&self, frame: u64) -> bool {
        frame >= self.block_frame
    }

    pub fn block(&mut self, frame: u64) {
        self.block_frame = frame + Self::BLOCK_FRAMES;
    }

    pub fn push(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn clear(&mut self) {
        self.messages.clear();
    }
}

fn wrap_response_into_json(value: JsValue) -> JsFuture {
    assert!(value.is_instance_of::<Response>());
    let resp: Response = value.dyn_into().unwrap();
    JsFuture::from(resp.json().unwrap())
}

pub fn fetch(request: &Request) -> Promise {
    let resp_value = JsFuture::from(web_sys::window().unwrap().fetch_with_request(request))
        .and_then(wrap_response_into_json);

    future_to_promise(resp_value)
}

fn request_url(method: &str, url: &str) -> Request {
    let mut opts = RequestInit::new();
    opts.method(method);

    Request::new_with_str_and_init(&url, &opts).unwrap()
}

pub fn request_turns_since(since: usize) -> Request {
    let pathname = pathname();
    request_url("GET", &format!("{pathname}/turns/{since}"))
}

pub fn request_state() -> Request {
    let pathname = pathname();
    request_url("GET", &format!("{pathname}/state"))
}

pub fn request_session() -> Request {
    request_url("GET", &format!("/session"))
}

pub fn send_ready(session_id: String) -> Option<Promise> {
    let session_request = SessionRequest { session_id };

    if let Ok(json) = serde_json::to_string(&session_request) {
        let pathname = pathname();
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.body(Some(&json.into()));

        let url = format!("{pathname}/ready");

        let request = &Request::new_with_str_and_init(&url, &opts).unwrap();

        request
            .headers()
            .set("Content-Type", "application/json")
            .unwrap();

        Some(fetch(request))
    } else {
        None
    }
}

pub fn send_message(session_id: String, message: Message) -> Option<Promise> {
    let session_message = SessionMessage {
        session_id,
        message,
    };

    if let Ok(json) = serde_json::to_string(&session_message) {
        let pathname = pathname();
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.body(Some(&json.into()));

        let url = format!("{pathname}/act");

        let request = &Request::new_with_str_and_init(&url, &opts).unwrap();

        request
            .headers()
            .set("Content-Type", "application/json")
            .unwrap();

        Some(fetch(request))
    } else {
        None
    }
}

pub fn pathname() -> String {
    web_sys::window().unwrap().location().pathname().unwrap()
}

pub fn get_session_id() -> Option<String> {
    window()
        .local_storage()
        .unwrap_or_default()
        .and_then(|storage| storage.get_item("session_id").unwrap_or_default())
}
