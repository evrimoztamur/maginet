use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use shared::{Level, LobbyError, SessionRequest};
use wasm_bindgen::JsValue;
use web_sys::{
    console, CanvasRenderingContext2d, DomRectReadOnly, FocusEvent, HtmlCanvasElement,
    HtmlInputElement, KeyboardEvent, MouseEvent, TouchEvent,
};

use super::{
    BaseState, EditorState, LobbyState, MenuState, MenuTeleport, Pointer, PreviewState, BOARD_SCALE,
};
use crate::{
    app::State,
    draw::{draw_board, draw_sprite},
    net::get_session_id,
    storage, window,
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
    Base(BaseState),
    MenuMain(MenuState),
    Lobby(LobbyState),
    Editor(EditorState),
    MenuTeleport(MenuTeleport),
    Preview(PreviewState),
}

pub struct AppContext {
    pub session_id: Option<String>,
    pub pointer: Pointer,
    pub frame: u64,
    pub canvas_settings: CanvasSettings,
    pub text_input: Option<(String, String)>,
}

pub struct App {
    app_context: AppContext,
    state_sort: StateSort,
    atlas_complete: bool,
}

impl App {
    pub fn new(canvas_settings: &CanvasSettings) -> App {
        App {
            app_context: AppContext {
                session_id: get_session_id(),
                pointer: Pointer::new(&canvas_settings),
                frame: 0,
                canvas_settings: canvas_settings.clone(),
                text_input: None,
            },
            state_sort: StateSort::Base(BaseState::default()),
            atlas_complete: false,
        }
    }

    pub fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
    ) -> Result<(), JsValue> {
        context.clear_rect(
            0.0,
            0.0,
            self.app_context.canvas_settings.element_width() as f64,
            self.app_context.canvas_settings.element_height() as f64,
        );
        interface_context.clear_rect(
            0.0,
            0.0,
            self.app_context.canvas_settings.element_width() as f64,
            self.app_context.canvas_settings.element_height() as f64,
        );

        context.save();
        interface_context.save();

        if self.app_context.canvas_settings.orientation {
            context.translate(self.app_context.canvas_settings.element_width() as f64, 0.0)?;
            context.rotate(std::f64::consts::PI / 2.0)?;
            interface_context
                .translate(self.app_context.canvas_settings.element_width() as f64, 0.0)?;
            interface_context.rotate(std::f64::consts::PI / 2.0)?;
        }

        context.scale(2.0, 2.0)?;
        interface_context.scale(2.0, 2.0)?;

        context.translate(
            self.app_context.canvas_settings.padding_x() as f64,
            self.app_context.canvas_settings.padding_y() as f64,
        )?;

        interface_context.translate(
            self.app_context.canvas_settings.padding_x() as f64,
            self.app_context.canvas_settings.padding_y() as f64,
        )?;

        let mut result = Ok(());

        if !self.atlas_complete {
            self.atlas_complete = true;
            draw_board(&atlas, 256.0, 256.0, 2, 2, 2, 2)?;
            draw_board(atlas, 256.0, 320.0, 4, 2, 4, 2)?;
        } else {
            result = match &mut self.state_sort {
                StateSort::MenuMain(state) => {
                    state.draw(context, interface_context, atlas, &self.app_context)
                }
                StateSort::MenuTeleport(state) => {
                    state.draw(context, interface_context, atlas, &self.app_context)
                }
                StateSort::Lobby(state) => {
                    state.draw(context, interface_context, atlas, &self.app_context)
                }
                StateSort::Base(state) => {
                    state.draw(context, interface_context, atlas, &self.app_context)
                }
                StateSort::Editor(state) => {
                    state.draw(context, interface_context, atlas, &self.app_context)
                }
                StateSort::Preview(state) => {
                    state.draw(context, interface_context, atlas, &self.app_context)
                }
            };
        }

        // DRAW cursor
        draw_sprite(
            interface_context,
            atlas,
            64.0,
            8.0,
            16.0,
            16.0,
            self.app_context.pointer.location.0 as f64 - 5.0,
            self.app_context.pointer.location.1 as f64 - 2.0,
        )?;

        context.restore();
        interface_context.restore();

        self.app_context.frame = (window().performance().unwrap().now() * 0.06) as u64;
        self.app_context.pointer.swap();
        self.app_context.text_input = None;

        result
    }

    pub fn tick(&mut self, text_input: &HtmlInputElement) {
        let next_state = match &mut self.state_sort {
            StateSort::MenuMain(state) => state.tick(text_input, &self.app_context),
            StateSort::MenuTeleport(state) => state.tick(text_input, &self.app_context),
            StateSort::Lobby(state) => state.tick(text_input, &self.app_context),
            StateSort::Base(state) => state.tick(text_input, &self.app_context),
            StateSort::Editor(state) => state.tick(text_input, &self.app_context),
            StateSort::Preview(state) => state.tick(text_input, &self.app_context),
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

    pub fn on_blur(&mut self, _event: FocusEvent, text_input: &HtmlInputElement) {
        if let Some(field) = text_input.dataset().get("field") {
            self.app_context.text_input = Some((field, text_input.value()));
            text_input.dataset().delete("field");
        }
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
        let x = event.page_x() - bound.left() as i32;
        let y = event.page_y() - bound.top() as i32;
        let pointer_location =
            App::transform_pointer(&self.app_context.canvas_settings, bound, x, y);

        self.app_context.pointer.location = pointer_location;

        event.prevent_default();
    }

    pub fn on_touch_start(&mut self, bound: &DomRectReadOnly, event: TouchEvent) {
        if let Some(touch) = event.target_touches().item(0) {
            let x = touch.page_x() - bound.left() as i32;
            let y = touch.page_y() - bound.top() as i32;
            let pointer_location =
                App::transform_pointer(&self.app_context.canvas_settings, bound, x, y);

            {
                match &mut self.state_sort {
                    StateSort::Lobby(lobby_state) => {
                        if pointer_location.0 < 0 {
                            self.app_context.pointer.button = true;
                        } else if lobby_state.is_interface_active() {
                            self.app_context.pointer.button = true;
                        } else {
                            let board_offset = lobby_state.board_offset();

                            match (
                                lobby_state.location_as_position(
                                    pointer_location,
                                    board_offset,
                                    BOARD_SCALE,
                                ),
                                lobby_state.location_as_position(
                                    self.app_context.pointer.location,
                                    board_offset,
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
                    }
                    _ => self.app_context.pointer.button = true,
                };
            }

            self.app_context.pointer.location = pointer_location;
        }
    }

    pub fn on_touch_end(&mut self, bound: &DomRectReadOnly, event: TouchEvent) {
        if let Some(touch) = event.target_touches().item(0) {
            let x = touch.page_x() - bound.left() as i32;
            let y = touch.page_y() - bound.top() as i32;

            let pointer_location =
                App::transform_pointer(&self.app_context.canvas_settings, bound, x, y);
            self.app_context.pointer.location = pointer_location;
        }

        self.app_context.pointer.button = false;
    }

    pub fn on_touch_move(&mut self, bound: &DomRectReadOnly, event: TouchEvent) {
        if let Some(touch) = event.target_touches().item(0) {
            let x = touch.page_x() - bound.left() as i32;
            let y = touch.page_y() - bound.top() as i32;

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
        match &mut self.state_sort {
            StateSort::Lobby(state) => {
                match event.code().as_str() {
                    "KeyB" => {
                        state.take_best_turn();
                    }
                    "KeyN" => {
                        console::log_1(
                            &format!(
                                "{:?}",
                                state
                                    .lobby()
                                    .game
                                    .all_available_turns(state.lobby().game.turn_for())
                            )
                            .into(),
                        );
                    }
                    "KeyM" => {
                        console::log_1(&format!("{:?}", state.lobby()).into());
                    }
                    _ => (),
                };
            }
            StateSort::Editor(_state) => match event.code().as_str() {
                _ => (),
            },
            _ => (),
        }
    }

    pub fn on_session_response(&mut self, value: JsValue) {
        let session_request: SessionRequest = serde_wasm_bindgen::from_value(value).unwrap();
        let session_id = session_request.session_id;

        self.set_session_id(session_id.clone());

        storage().map(|storage| storage.set_item("session_id", session_id.as_str()));
    }

    fn load_levels() -> HashMap<usize, Level> {
        serde_json::from_str(
            storage()
                .and_then(|storage| storage.get_item("levels").unwrap_or_default())
                .unwrap_or_default()
                .as_str(),
        )
        .unwrap_or_default()
    }

    fn save_levels(levels: HashMap<usize, Level>) {
        let value = serde_json::to_string(&levels).unwrap();
        storage().and_then(|storage| storage.set_item("levels", value.as_str()).ok());
    }

    pub fn load_level(level_id: usize) -> Option<Level> {
        let levels = Self::load_levels();

        console::log_1(&format!("{:?}", levels).into());

        levels.get(&level_id).cloned()
    }

    pub fn save_level(level_id: usize, level: Level) {
        let mut levels = Self::load_levels();

        levels.insert(level_id, level);

        Self::save_levels(levels);
    }
}

#[derive(Clone, Default)]
pub struct CanvasSettings {
    pub interface_width: u32,
    pub interface_height: u32,
    pub canvas_width: u32,
    pub canvas_height: u32,
    pub canvas_scale: u32,
    pub orientation: bool,
}

impl CanvasSettings {
    pub fn inverse_interface_center(&self) -> (i32, i32) {
        (
            -((self.interface_width / 2) as i32),
            -((self.interface_height / 2) as i32),
        )
    }

    pub fn element_width(&self) -> u32 {
        if self.orientation {
            self.canvas_height * self.canvas_scale
        } else {
            self.canvas_width * self.canvas_scale
        }
    }

    pub fn element_height(&self) -> u32 {
        if self.orientation {
            self.canvas_width * self.canvas_scale
        } else {
            self.canvas_height * self.canvas_scale
        }
    }

    pub fn padding_x(&self) -> u32 {
        (self.canvas_width - self.interface_width) / 2
    }

    pub fn padding_y(&self) -> u32 {
        (self.canvas_height - self.interface_height) / 2
    }

    pub fn padding(&self) -> (i32, i32) {
        (self.padding_x() as i32, self.padding_y() as i32)
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
