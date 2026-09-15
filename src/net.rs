use std::time::Duration;

#[cfg(not(feature = "ios"))]
use futures::TryFutureExt;
use js_sys::Promise;
use shared::{LobbyID, LobbySettings, Message, SessionMessage, SessionNewLobby, SessionRequest};
#[cfg(not(feature = "ios"))]
use wasm_bindgen::{JsCast, JsValue};
#[cfg(not(feature = "ios"))]
use wasm_bindgen_futures::{future_to_promise, JsFuture};
#[cfg(not(feature = "ios"))]
use web_sys::Response;
use web_sys::{Request, RequestInit};

use crate::{storage, window};

#[cfg(all(feature = "deploy", not(feature = "ios")))]
const API_URL: &str = "https://maginet.evrim.zone";
#[cfg(not(feature = "deploy"))]
const API_URL: &str = "https://tunnel.evrim.zone";

#[cfg(feature = "ios")]
const API_URL: &str = "/api";

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

#[cfg(not(feature = "ios"))]
fn wrap_response_into_json(value: JsValue) -> JsFuture {
    assert!(value.is_instance_of::<Response>());
    let resp: Response = value.dyn_into().unwrap();
    JsFuture::from(resp.json().unwrap())
}

#[cfg(feature = "ios")]
#[wasm_bindgen::prelude::wasm_bindgen(module = "/static/js/ios-access.js")]
extern "C" {
    #[wasm_bindgen::prelude::wasm_bindgen(js_name = nativeFetch)]
    fn native_fetch(request: &Request) -> Promise;
}

pub fn fetch(request: &Request) -> Promise {
    #[cfg(feature = "ios")]
    {
        native_fetch(request)
    }
    #[cfg(not(feature = "ios"))]
    {
        let resp_value = JsFuture::from(web_sys::window().unwrap().fetch_with_request(request))
            .and_then(wrap_response_into_json);

        future_to_promise(resp_value)
    }
}

fn request_url(method: &str, url: &str) -> Request {
    let mut opts = RequestInit::new();
    opts.method(method);

    Request::new_with_str_and_init(url, &opts).unwrap()
}

pub fn request_session() -> Request {
    request_url("GET", &format!("{API_URL}/session"))
}

pub fn request_state(lobby_id: LobbyID) -> Request {
    request_url("GET", &format!("{API_URL}/lobby/{lobby_id}/state"))
}

pub fn request_turns_since(lobby_id: LobbyID, session_id: String, since: usize) -> Option<Promise> {
    post_probe(
        format!("{API_URL}/lobby/{lobby_id}/turns/{since}"),
        session_id,
    )
}

pub fn request_lobbies() -> Request {
    request_url("GET", &format!("{API_URL}/lobbies"))
}

pub fn create_new_lobby(lobby_settings: LobbySettings, session_id: String) -> Option<Promise> {
    let session_request = SessionNewLobby {
        lobby_settings,
        session_id: Some(session_id),
    };

    if let Ok(json) = serde_json::to_string(&session_request) {
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.body(Some(&json.into()));

        let url = format!("{API_URL}/lobby/create");

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

pub fn post_probe(url: String, session_id: String) -> Option<Promise> {
    let session_request = SessionRequest { session_id };

    if let Ok(json) = serde_json::to_string(&session_request) {
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.body(Some(&json.into()));

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

pub fn send_ready(lobby_id: LobbyID, session_id: String) -> Option<Promise> {
    post_probe(format!("{API_URL}/lobby/{lobby_id}/ready"), session_id)
}

pub fn send_rematch(lobby_id: LobbyID, session_id: String) -> Option<Promise> {
    post_probe(format!("{API_URL}/lobby/{lobby_id}/rematch"), session_id)
}

pub fn send_message(lobby_id: LobbyID, session_id: String, message: Message) -> Option<Promise> {
    let session_message = SessionMessage {
        session_id,
        message,
    };

    if let Ok(json) = serde_json::to_string(&session_message) {
        let mut opts = RequestInit::new();
        opts.method("POST");
        opts.body(Some(&json.into()));

        let url = format!("{API_URL}/lobby/{lobby_id}/act");

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

pub fn get_session_id() -> Option<String> {
    storage().and_then(|storage| storage.get_item("session_id").unwrap_or_default())
}

pub fn client_timestamp() -> Duration {
    let since_the_epoch = window().performance().unwrap().now() as u64;

    Duration::from_millis(since_the_epoch)
}
