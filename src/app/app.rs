use serde::{Deserialize, Serialize};
use shared::{LobbyError, SessionRequest};
use wasm_bindgen::JsValue;
use web_sys::{
    CanvasRenderingContext2d, DomRectReadOnly, HtmlImageElement, KeyboardEvent, MouseEvent,
    TouchEvent,
};

use super::{LobbyState, MenuState, MenuTeleport, Pointer, BOARD_OFFSET, BOARD_SCALE};
use crate::{app::State, draw::draw_sprite, net::get_session_id, window};

/// Errors concerning the [`App`].
#[derive(Debug, Serialize, Deserialize)]
pub struct AppError(String);

impl From<LobbyError> for AppError {
    fn from(lobby_error: LobbyError) -> Self {
        AppError(format!("LobbyError: {0}", lobby_error.0))
    }
}

pub enum StateSort {
    MenuMain(MenuState),
    MenuTeleport(MenuTeleport),
    Lobby(LobbyState),
}

pub struct AppContext {
    pub session_id: Option<String>,
    pub pointer: Pointer,
    pub frame: u64,
    pub canvas_settings: CanvasSettings,
}

pub struct App {
    app_context: AppContext,
    state_sort: StateSort,
}

impl App {
    pub fn new(canvas_settings: CanvasSettings) -> App {
        App {
            app_context: AppContext {
                session_id: get_session_id(),
                pointer: Pointer::new(&canvas_settings),
                frame: 0,
                canvas_settings,
            },
            state_sort: StateSort::MenuMain(MenuState::new()),
        }
    }

    pub fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
    ) -> Result<(), JsValue> {
        context.clear_rect(
            0.0,
            0.0,
            self.app_context.canvas_settings.element_width() as f64,
            self.app_context.canvas_settings.element_height() as f64,
        );
        context.save();
        if self.app_context.canvas_settings.orientation() {
            context.translate(
                self.app_context.canvas_settings.canvas_width() as f64,
                self.app_context.canvas_settings.canvas_height() as f64,
            )?;
            context.rotate(std::f64::consts::PI / 2.0)?;
            context.translate(
                -(self.app_context.canvas_settings.canvas_width() as f64),
                -(self.app_context.canvas_settings.canvas_height() as f64),
            )?;
        }
        context.scale(2.0, 2.0)?;
        context.translate(
            self.app_context.canvas_settings.padding_x() as f64,
            self.app_context.canvas_settings.padding_y() as f64,
        )?;

        let mut result = Ok(());

        if atlas.complete() {
            result = match &mut self.state_sort {
                StateSort::MenuMain(menu_state) => {
                    menu_state.draw(context, atlas, &self.app_context)
                }
                StateSort::Lobby(lobby_state) => {
                    lobby_state.draw(context, atlas, &self.app_context)
                }
                StateSort::MenuTeleport(menu_state) => {
                    menu_state.draw(context, atlas, &self.app_context)
                }
            };
        }
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
        let next_state = match &mut self.state_sort {
            StateSort::MenuMain(menu_state) => menu_state.tick(&self.app_context),
            StateSort::Lobby(lobby_state) => lobby_state.tick(&self.app_context),
            StateSort::MenuTeleport(menu_state) => menu_state.tick(&self.app_context),
        };

        if let Some(next_state) = next_state {
            self.state_sort = next_state;
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
        let pointer_location =
            App::transform_pointer(&self.app_context.canvas_settings, bound, x, y);

        self.app_context.pointer.location = pointer_location;

        event.prevent_default();
    }

    pub fn on_touch_start(&mut self, bound: &DomRectReadOnly, event: TouchEvent) {
        if let Some(touch) = event.target_touches().item(0) {
            let x = touch.client_x() - bound.left() as i32;
            let y = touch.client_y() - bound.top() as i32;
            let pointer_location =
                App::transform_pointer(&self.app_context.canvas_settings, bound, x, y);

            {
                match &mut self.state_sort {
                    StateSort::Lobby(lobby_state) => {
                        match (
                            lobby_state.location_as_position(
                                pointer_location,
                                BOARD_OFFSET,
                                BOARD_SCALE,
                            ),
                            lobby_state.location_as_position(
                                self.app_context.pointer.location,
                                BOARD_OFFSET,
                                BOARD_SCALE,
                            ),
                        ) {
                            (Some(current_tile), Some(last_tile)) => {
                                if current_tile == last_tile {
                                    self.app_context.pointer.button = true;
                                } else if lobby_state.live_occupied(current_tile) {
                                    self.app_context.pointer.button = true;
                                }
                            }
                            _ => (),
                        }
                    }
                    _ => self.app_context.pointer.button = true,
                };
            }

            self.app_context.pointer.location = pointer_location;
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

            let pointer_location =
                App::transform_pointer(&self.app_context.canvas_settings, bound, x, y);
            self.app_context.pointer.location = pointer_location;
        }

        event.prevent_default();
    }

    fn transform_pointer(
        canvas_settings: &CanvasSettings,
        bound: &DomRectReadOnly,
        x: i32,
        y: i32,
    ) -> (i32, i32) {
        let x = (x as f64 * (canvas_settings.element_width() as f64 / bound.width())) as i32;
        let y = (y as f64 * (canvas_settings.element_height() as f64 / bound.height())) as i32;

        Pointer::location_from_real(canvas_settings, (x as i32 / 2, y as i32 / 2))
    }

    pub fn on_key_down(&mut self, event: KeyboardEvent) {
        match event.code().as_str() {
            "KeyB" => match &mut self.state_sort {
                StateSort::Lobby(lobby_state) => {
                    lobby_state.take_best_turn();
                }
                _ => (),
            },
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
}

#[derive(Clone, Default)]
pub struct CanvasSettings {
    interface_width: u32,
    interface_height: u32,
    canvas_width: u32,
    canvas_height: u32,
    canvas_scale: u32,
    orientation: bool,
}

impl CanvasSettings {
    pub fn interface_width(&self) -> u32 {
        if self.orientation() {
            self.interface_height
        } else {
            self.interface_width
        }
    }
    pub fn interface_height(&self) -> u32 {
        if self.orientation() {
            self.interface_width
        } else {
            self.interface_height
        }
    }
    pub fn canvas_width(&self) -> u32 {
        if self.orientation() {
            self.canvas_height
        } else {
            self.canvas_width
        }
    }
    pub fn canvas_height(&self) -> u32 {
        if self.orientation() {
            self.canvas_width
        } else {
            self.canvas_height
        }
    }
    pub fn element_width(&self) -> u32 {
        self.canvas_width() * self.canvas_scale
    }
    pub fn element_height(&self) -> u32 {
        self.canvas_height() * self.canvas_scale
    }
    pub fn padding_x(&self) -> u32 {
        (self.canvas_width() - self.interface_width) / 2
    }
    pub fn padding_y(&self) -> u32 {
        (self.canvas_height() - self.interface_height) / 2
    }
    pub fn padding(&self) -> (i32, i32) {
        (self.padding_x() as i32, self.padding_y() as i32)
    }
    pub fn orientation(&self) -> bool {
        self.orientation
    }

    pub fn new(
        canvas_width: u32,
        canvas_height: u32,
        interface_width: u32,
        interface_height: u32,
        canvas_scale: u32,
        orientation: bool,
    ) -> CanvasSettings {
        CanvasSettings {
            interface_width,
            interface_height,
            canvas_width,
            canvas_height,
            canvas_scale,
            orientation,
        }
    }
}
