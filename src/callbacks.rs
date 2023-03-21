use std::{cell::RefCell, rc::Rc};

use shared::{Message, SessionRequest};
use wasm_bindgen::JsValue;
use web_sys::{console, DomRect, HtmlCanvasElement, KeyboardEvent, MouseEvent, TouchEvent};

use crate::{
    app::{App, BOARD_OFFSET, BOARD_SCALE},
    net::{send_ready, MessagePool},
    window,
};

pub fn on_resize(canvas: &Rc<HtmlCanvasElement>, bound: &Rc<RefCell<Option<DomRect>>>) {
    bound.replace(Some(canvas.get_bounding_client_rect()));
}

pub fn on_mouse_down(app: &Rc<RefCell<App>>, event: MouseEvent) {
    let mut app = app.borrow_mut();

    match event.button() {
        0 => app.pointer.button = true,
        2 => app.pointer.alt_button = true,
        _ => (),
    }
}

pub fn on_mouse_up(app: &Rc<RefCell<App>>, event: MouseEvent) {
    let mut app = app.borrow_mut();

    match event.button() {
        0 => app.pointer.button = false,
        2 => app.pointer.alt_button = false,
        _ => (),
    }
}

pub fn on_mouse_move(
    app: &Rc<RefCell<App>>,
    bound: &Rc<RefCell<Option<DomRect>>>,
    event: MouseEvent,
) {
    let bound = bound.borrow();

    if let Some(bound) = bound.as_ref() {
        let mut app = app.borrow_mut();

        let x = event.client_x() - bound.left() as i32;
        let y = event.client_y() - bound.top() as i32;
        let x = (x as f64 * (512.0 / bound.width())) as i32;
        let y = (y as f64 * (512.0 / bound.width())) as i32;

        app.pointer.location = (x / 2, y / 2);
    }

    event.prevent_default();
}

pub fn on_touch_start(
    app: &Rc<RefCell<App>>,
    bound: &Rc<RefCell<Option<DomRect>>>,
    event: TouchEvent,
) {
    let bound = bound.borrow();

    if let Some(bound) = bound.as_ref() {
        if let Some(touch) = event.target_touches().item(0) {
            let mut app = app.borrow_mut();

            let x = touch.client_x() - bound.left() as i32;
            let y = touch.client_y() - bound.top() as i32;

            let x = (x as f64 * (512.0 / bound.width())) as i32;
            let y = (y as f64 * (512.0 / bound.width())) as i32;

            let pointer_location = (x as i32 / 2, y as i32 / 2);

            if (pointer_location.0 - app.pointer.location.0).abs() < 16
                && (pointer_location.1 - app.pointer.location.1).abs() < 16
            {
                app.pointer.button = true;
            } else if let Some(selected_tile) =
                app.lobby
                    .game
                    .location_as_position(pointer_location, BOARD_OFFSET, BOARD_SCALE)
            {
                if let Some(_) = app.lobby.game.live_occupant(&selected_tile) {
                    app.pointer.button = true;
                }
            }

            app.pointer.location = (x as i32 / 2, y as i32 / 2);
        }
    }
    event.prevent_default();
}

pub fn on_touch_end(app: &Rc<RefCell<App>>, _: TouchEvent) {
    let mut app = app.borrow_mut();

    app.pointer.button = false;
    // event.prevent_default();
}

pub fn on_touch_move(
    app: &Rc<RefCell<App>>,
    bound: &Rc<RefCell<Option<DomRect>>>,
    event: TouchEvent,
) {
    let mut app = app.borrow_mut();
    let bound = bound.borrow();

    if let Some(bound) = bound.as_ref() {
        if let Some(touch) = event.target_touches().item(0) {
            let x = touch.client_x() - bound.left() as i32;
            let y = touch.client_y() - bound.top() as i32;

            let x = (x as f64 * (512.0 / bound.width())) as i32;
            let y = (y as f64 * (512.0 / bound.width())) as i32;

            app.pointer.location = (x as i32 / 2, y as i32 / 2);
        }
    }
    event.prevent_default();
}

pub fn on_state_response(app: &Rc<RefCell<App>>, value: JsValue) {
    let mut app = app.borrow_mut();
    let message: Message = serde_wasm_bindgen::from_value(value).unwrap();

    match message {
        Message::Lobby(lobby) => {
            app.lobby = lobby;
        }
        _ => (),
    }

    if !app.in_lobby() && app.session_id.is_some() {
        send_ready(app.session_id.clone().unwrap());
    }
}

pub fn on_message_response(message_pool: &Rc<RefCell<MessagePool>>, value: JsValue) {
    let mut message_pool = message_pool.borrow_mut();

    message_pool.push(serde_wasm_bindgen::from_value(value).unwrap());
}

pub fn on_session_response(app: &Rc<RefCell<App>>, value: JsValue) {
    let mut app = app.borrow_mut();
    let session_request: SessionRequest = serde_wasm_bindgen::from_value(value).unwrap();
    let session_id = session_request.session_id;

    app.session_id = Some(session_id.clone());

    window()
        .local_storage()
        .unwrap_or_default()
        .map(|storage| storage.set_item("session_id", session_id.as_str()));
}

pub fn on_key_down(
    app: &Rc<RefCell<App>>,
    message_pool: &Rc<RefCell<MessagePool>>,
    event: KeyboardEvent,
) {
    let app = app.borrow();
    let mut message_pool = message_pool.borrow_mut();

    match event.code().as_str() {
        "KeyB" => {
            // let turn = app.lobby.game.best_turn();
            // console::log_1(&format!("{:?}", turn).into());

            // message_pool.push(Message::Move(turn.0));
        }
        "KeyN" => {
            // console::log_1(&format!("{:?}", app.lobby.game.all_available_turns(app.lobby.game.turn_for())).into());
        }
        _ => (),
    };
}
