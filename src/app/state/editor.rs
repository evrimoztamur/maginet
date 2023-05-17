use shared::{Board, LobbySettings, LobbySort, Position, DEFAULT_BOARD_SIZE, Mage};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::{LobbyState, MenuState, State};
use crate::{
    app::{
        Alignment, AppContext, ButtonClass, ButtonElement, ButtonTrim, Interface, Particle,
        ParticleSort, StateSort, ToggleButtonElement, UIElement, UIEvent, BOARD_OFFSET,
        BOARD_OFFSET_F64, BOARD_SCALE, BOARD_SCALE_F64,
    },
    draw::{draw_crosshair, draw_particle, draw_sprite},
};

pub enum EditorSelection {
    Mage(Mage),
    Tile(Position),
    None,
}

pub struct EditorState {
    interface: Interface,
    board: Board,
    particles: Vec<Particle>,
    selection: EditorSelection,
}

const BUTTON_MENU: usize = 0;
const BUTTON_MODE_TOGGLE: usize = 10;

impl EditorState {
    pub fn new() -> EditorState {
        let button_menu = ToggleButtonElement::new(
            (-60, 118),
            (20, 20),
            BUTTON_MENU,
            ButtonTrim::Round,
            ButtonClass::Bright,
            crate::app::ContentElement::Sprite((112, 32), (16, 16)),
        );

        let button_mode_toggle = ButtonElement::new(
            (236, 228),
            (80, 24),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Glorious,
            ButtonClass::Action,
            crate::app::ContentElement::Text("Battle".to_string(), Alignment::Center),
        );

        let button_save = ButtonElement::new(
            (244, 204),
            (64, 16),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Text("Save".to_string(), Alignment::Center),
        );

        let button_width_minus = ButtonElement::new(
            (82, 248),
            (12, 12),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Sprite((88, 24), (8, 8)),
        );

        let button_width_plus = ButtonElement::new(
            (98, 248),
            (12, 12),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Sprite((80, 24), (8, 8)),
        );

        let button_height_minus = ButtonElement::new(
            (216, 114),
            (12, 12),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Sprite((88, 24), (8, 8)),
        );

        let button_height_plus = ButtonElement::new(
            (216, 130),
            (12, 12),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Sprite((80, 24), (8, 8)),
        );

        let button_team_left = ButtonElement::new(
            (236, 114 - 92),
            (12, 20),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Sprite((64, 24), (8, 8)),
        );

        let button_team_right = ButtonElement::new(
            (304, 114 - 92),
            (12, 20),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Sprite((72, 24), (8, 8)),
        );

        let button_spell_left = ButtonElement::new(
            (244, 114 - 44),
            (12, 20),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Sprite((64, 24), (8, 8)),
        );

        let button_spell_right = ButtonElement::new(
            (296, 114 - 44),
            (12, 20),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Sprite((72, 24), (8, 8)),
        );

        let button_mana_left = ButtonElement::new(
            (244, 114 - 8),
            (12, 12),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Sprite((64, 24), (8, 8)),
        );

        let button_mana_right = ButtonElement::new(
            (296, 114 - 8),
            (12, 12),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Sprite((72, 24), (8, 8)),
        );

        let button_delete = ButtonElement::new(
            (260, 138),
            (32, 20),
            BUTTON_MODE_TOGGLE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Sprite((128, 32), (16, 16)),
        );

        let mage_interface = Interface::new(vec![
            Box::new(button_team_left),
            Box::new(button_team_right),
            Box::new(button_spell_left),
            Box::new(button_spell_right),
            Box::new(button_mana_left),
            Box::new(button_mana_right),
            Box::new(button_delete),
        ]);

        let root_element = Interface::new(vec![
            Box::new(button_menu),
            Box::new(button_mode_toggle),
            Box::new(button_save),
            Box::new(button_width_minus),
            Box::new(button_width_plus),
            Box::new(button_height_minus),
            Box::new(button_height_plus),
            Box::new(mage_interface),
        ]);

        let board = Board::new(DEFAULT_BOARD_SIZE.0, DEFAULT_BOARD_SIZE.1).unwrap();

        EditorState {
            interface: root_element,
            board,
            particles: Vec::new(),
            selection: EditorSelection::None
        }
    }
}

impl State for EditorState {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        let frame = app_context.frame;
        let pointer = &app_context.pointer;

        context.save();

        context.translate(BOARD_OFFSET_F64.0 - 32.0, BOARD_OFFSET_F64.1)?;

        draw_sprite(context, atlas, 256.0, 0.0, 256.0, 256.0, 0.0, 0.0)?;

        for particle in self.particles.iter_mut() {
            particle.tick();
            draw_particle(context, atlas, &particle, frame)?;
        }

        self.particles.drain_filter(|particle| !particle.is_alive());

        if let Some(selected_tile) = self.board.location_as_position(
            pointer.location,
            (BOARD_OFFSET.0 - 32, BOARD_OFFSET.1),
            BOARD_SCALE,
        ) {
            draw_crosshair(context, atlas, &selected_tile, (32.0, 32.0), frame)?;
        }

        context.restore();

        self.interface
            .draw(interface_context, atlas, pointer, frame)?;

        Ok(())
    }

    fn tick(&mut self, app_context: &AppContext) -> Option<StateSort> {
        let pointer = &app_context.pointer;

        if let Some(selected_tile) = self.board.location_as_position(
            pointer.location,
            (BOARD_OFFSET.0 - 32, BOARD_OFFSET.1),
            BOARD_SCALE,
        ) {
            if pointer.clicked() {
                for _ in 0..40 {
                    let d = js_sys::Math::random() * std::f64::consts::TAU;
                    let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;
                    self.particles.push(Particle::new(
                        (selected_tile.0 as f64, selected_tile.1 as f64),
                        (d.cos() * v, d.sin() * v),
                        (js_sys::Math::random() * 20.0) as u64,
                        ParticleSort::Missile,
                    ));
                }
            }
        }

        if let Some(UIEvent::ButtonClick(value)) = self.interface.tick(pointer) {
            match value {
                BUTTON_MODE_TOGGLE => {}
                _ => (),
            }
        }

        None
    }
}
