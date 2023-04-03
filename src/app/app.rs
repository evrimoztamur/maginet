use serde::{Deserialize, Serialize};
use shared::{LobbyError, LobbyID, Message, SessionRequest};
use wasm_bindgen::JsValue;
use web_sys::{
    CanvasRenderingContext2d, DomRectReadOnly, HtmlImageElement, KeyboardEvent, MouseEvent,
    TouchEvent,
};

use super::{LobbyState, Pointer, BOARD_OFFSET, BOARD_SCALE};
use crate::{
    app::State,
    draw::draw_sprite,
    net::{get_session_id, send_ready},
    window,
};

/// Errors concerning the [`App`].
#[derive(Debug, Serialize, Deserialize)]
pub struct AppError(String);

impl From<LobbyError> for AppError {
    fn from(lobby_error: LobbyError) -> Self {
        AppError(format!("LobbyError: {0}", lobby_error.0))
    }
}

pub enum StateSort {
    MenuMain,
    MenuLobby,
    MenuSettings,
    Lobby(LobbyState),
}

pub struct AppContext {
    pub session_id: Option<String>,
    pub pointer: Pointer,
    pub frame: u64,
}

pub struct App {
    app_context: AppContext,
    state_sort: StateSort,
}

impl App {
    pub fn new() -> App {
        App {
            app_context: AppContext {
                session_id: get_session_id(),
                pointer: Pointer::new(),
                frame: 0,
            },
            state_sort: StateSort::MenuMain,
        }
    }

    pub fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
    ) -> Result<(), JsValue> {
        context.clear_rect(0.0, 0.0, 512.0, 512.0);
        context.save();

        context.scale(2.0, 2.0)?;

        let result = match &mut self.state_sort {
            StateSort::MenuMain => Ok(()),
            StateSort::MenuLobby => Ok(()),
            StateSort::MenuSettings => Ok(()),
            StateSort::Lobby(lobby_state) => lobby_state.draw(context, atlas, &self.app_context),
        };

        // DRAW cursor
        draw_sprite(
            context,
            atlas,
            64.0,
            8.0,
            16.0,
            16.0,
            self.app_context.pointer.location.0 as f64 - 5.0,
            self.app_context.pointer.location.1 as f64 - 1.0,
        )?;

        context.restore();

        self.app_context.frame += 1;
        self.app_context.pointer.swap();

        result
    }

    pub fn tick(&mut self) {
        match &mut self.state_sort {
            StateSort::Lobby(lobby_state) => lobby_state.tick(&self.app_context),
            _ => (),
        }
    }

    pub fn lobby_id(&self) -> Result<LobbyID, AppError> {
        match &self.state_sort {
            StateSort::Lobby(lobby_state) => lobby_state.lobby_id().map_err(|err| err.into()),
            _ => Err(AppError("app is not in the appropriate state".to_string())),
        }
    }

    pub fn session_id(&self) -> Option<&String> {
        self.app_context.session_id.as_ref()
    }

    pub fn set_session_id(&mut self, session_id: String) {
        self.app_context.session_id = Some(session_id);
    }

    pub fn on_mouse_down(&mut self, event: MouseEvent) {
        match event.button() {
            0 => self.app_context.pointer.button = true,
            2 => self.app_context.pointer.alt_button = true,
            _ => (),
        }
    }

    pub fn on_mouse_up(&mut self, event: MouseEvent) {
        match event.button() {
            0 => self.app_context.pointer.button = false,
            2 => self.app_context.pointer.alt_button = false,
            _ => (),
        }
    }

    pub fn on_mouse_move(&mut self, bound: &DomRectReadOnly, event: MouseEvent) {
        let x = event.client_x() - bound.left() as i32;
        let y = event.client_y() - bound.top() as i32;
        let x = (x as f64 * (512.0 / bound.width())) as i32;
        let y = (y as f64 * (512.0 / bound.width())) as i32;

        self.app_context.pointer.location = (x / 2, y / 2);

        event.prevent_default();
    }

    pub fn on_touch_start(&mut self, bound: &DomRectReadOnly, event: TouchEvent) {
        if let Some(touch) = event.target_touches().item(0) {
            let x = touch.client_x() - bound.left() as i32;
            let y = touch.client_y() - bound.top() as i32;

            let x = (x as f64 * (512.0 / bound.width())) as i32;
            let y = (y as f64 * (512.0 / bound.width())) as i32;

            let pointer_location = (x as i32 / 2, y as i32 / 2);

            if (pointer_location.0 - self.app_context.pointer.location.0).abs() < 16
                && (pointer_location.1 - self.app_context.pointer.location.1).abs() < 16
            {
                self.app_context.pointer.button = true;
            } else {
                match &mut self.state_sort {
                    StateSort::Lobby(lobby_state) => {
                        if let Some(selected_tile) = lobby_state.location_as_position(
                            pointer_location,
                            BOARD_OFFSET,
                            BOARD_SCALE,
                        ) {
                            if lobby_state.live_occupied(selected_tile) {
                                self.app_context.pointer.button = true;
                            }
                        }
                    }
                    _ => (),
                };
            }

            self.app_context.pointer.location = (x as i32 / 2, y as i32 / 2);
        }

        event.prevent_default();
    }

    pub fn on_touch_end(&mut self, _: TouchEvent) {
        self.app_context.pointer.button = false;
    }

    pub fn on_touch_move(&mut self, bound: &DomRectReadOnly, event: TouchEvent) {
        if let Some(touch) = event.target_touches().item(0) {
            let x = touch.client_x() - bound.left() as i32;
            let y = touch.client_y() - bound.top() as i32;

            let x = (x as f64 * (512.0 / bound.width())) as i32;
            let y = (y as f64 * (512.0 / bound.width())) as i32;

            self.app_context.pointer.location = (x as i32 / 2, y as i32 / 2);
        }

        event.prevent_default();
    }

    pub fn on_key_down(&mut self, event: KeyboardEvent) {
        match event.code().as_str() {
            "KeyB" => {
                // let turn = app
                //     .lobby
                //     .game
                //     .best_turn(window().performance().unwrap().now().to_bits());
                // console::log_1(&format!("{:?}", turn).into());

                // message_pool.push(Message::Move(turn.0));
            }
            "KeyN" => {
                // console::log_1(&format!("{:?}", app.lobby.game.all_available_turns(app.lobby.game.turn_for())).into());
            }
            _ => (),
        };
    }

    pub fn on_session_response(&mut self, value: JsValue) {
        let session_request: SessionRequest = serde_wasm_bindgen::from_value(value).unwrap();
        let session_id = session_request.session_id;

        self.set_session_id(session_id.clone());

        window()
            .local_storage()
            .unwrap_or_default()
            .map(|storage| storage.set_item("session_id", session_id.as_str()));
    }

    pub fn on_state_response(&mut self, value: JsValue) {
        let message: Message = serde_wasm_bindgen::from_value(value).unwrap();

        match &mut self.state_sort {
            StateSort::Lobby(lobby_state) => {
                match message {
                    Message::Lobby(lobby) => {
                        lobby_state.set_lobby(lobby);
                    }
                    _ => (),
                }

                if let Ok(lobby_id) = self.lobby_id() {
                    send_ready(lobby_id, self.session_id().unwrap().clone());
                }
            }
            _ => (),
        };
    }

    pub fn on_message_response(&mut self, value: JsValue) {
        let message: Message = serde_wasm_bindgen::from_value(value).unwrap();

        match &mut self.state_sort {
            StateSort::Lobby(lobby_state) => {
                lobby_state.push_message(message);
            }
            _ => (),
        };
    }
}
