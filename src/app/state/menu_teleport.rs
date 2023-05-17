use std::ops::BitXorAssign;

use shared::{Board, LobbySettings, LobbySort};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::{LobbyState, MenuState, State};
use crate::{
    app::{
        Alignment, AppContext, ButtonClass, ButtonElement, ButtonTrim, Interface, Particle,
        ParticleSort, StateSort, UIElement, UIEvent, BOARD_SCALE,
    },
    draw::{draw_crosshair, draw_particle, draw_sprite},
    tuple_as,
};

pub struct MenuTeleport {
    interface: Interface,
    lobby_id: u16,
    particles: Vec<Particle>,
}

const BOARD_OFFSET: (i32, i32) = ((4 * BOARD_SCALE.0) / 2, (4 * BOARD_SCALE.1) / 2);

const BUTTON_TELEPORT: usize = 20;
const BUTTON_BACK: usize = 21;

impl MenuTeleport {
    pub fn new() -> MenuTeleport {
        let button_teleport = ButtonElement::new(
            (16, 188),
            (80, 24),
            BUTTON_TELEPORT,
            ButtonTrim::Glorious,
            ButtonClass::Action,
            crate::app::ContentElement::Text("Teleport".to_string(), Alignment::Center),
        );

        let button_back = ButtonElement::new(
            (156, 192),
            (88, 16),
            BUTTON_BACK,
            ButtonTrim::Glorious,
            ButtonClass::Default,
            crate::app::ContentElement::Text("Back".to_string(), Alignment::Center),
        );

        let root_element = Interface::new(vec![Box::new(button_teleport), Box::new(button_back)]);

        MenuTeleport {
            interface: root_element,
            lobby_id: 0,
            particles: Vec::new(),
        }
    }
}

impl State for MenuTeleport {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        let board_scale = tuple_as!(BOARD_SCALE, f64);

        let frame = app_context.frame;
        let pointer = &app_context.pointer;

        context.save();

        context.translate(board_scale.0 + 64.0, board_scale.1 + 48.0)?;

        draw_sprite(context, atlas, 256.0, 0.0, 64.0, 64.0, 0.0, 0.0)?;
        draw_sprite(context, atlas, 448.0, 0.0, 64.0, 64.0, 64.0, 0.0)?;
        draw_sprite(context, atlas, 256.0, 192.0, 64.0, 64.0, 0.0, 64.0)?;
        draw_sprite(context, atlas, 448.0, 192.0, 64.0, 64.0, 64.0, 64.0)?;

        for particle in self.particles.iter_mut() {
            particle.tick();
            draw_particle(context, atlas, &particle, frame)?;
        }

        self.particles.drain_filter(|particle| !particle.is_alive());

        let board = Board {
            width: 4,
            height: 4,
        };

        if let Some(selected_tile) = board.location_as_position(
            pointer.location,
            (BOARD_OFFSET.0 + 64, BOARD_OFFSET.1 + 48),
            BOARD_SCALE,
        ) {
            if self.lobby_id & (1 << ((selected_tile.1 << 2) | selected_tile.0)) as u16 == 0 {
                draw_crosshair(context, atlas, &selected_tile, (32.0, 32.0), frame)?;
            } else {
                draw_crosshair(context, atlas, &selected_tile, (64.0, 32.0), frame)?;
            }
        }

        let mut lid = self.lobby_id;

        while lid != 0 {
            let tz = lid.trailing_zeros();
            let x = tz % 4;
            let y = tz / 4;

            lid ^= 1 << tz;

            draw_sprite(
                context,
                atlas,
                96.0,
                32.0,
                16.0,
                16.0,
                x as f64 * board_scale.0 + 8.0,
                y as f64 * board_scale.1 + 8.0,
            )?;
        }

        context.restore();

        self.interface
            .draw(interface_context, atlas, pointer, frame)?;

        Ok(())
    }

    fn tick(&mut self, app_context: &AppContext) -> Option<StateSort> {
        let pointer = &app_context.pointer;

        let board = Board {
            width: 4,
            height: 4,
        };

        if let Some(selected_tile) = board.location_as_position(
            pointer.location,
            (BOARD_OFFSET.0 + 64, BOARD_OFFSET.1 + 48),
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

                self.lobby_id
                    .bitxor_assign(1 << ((selected_tile.1 << 2) | selected_tile.0));
            }
        }

        if let Some(UIEvent::ButtonClick(value)) = self.interface.tick(pointer) {
            match value {
                BUTTON_BACK => {
                    return Some(StateSort::MenuMain(MenuState::new()));
                }
                BUTTON_TELEPORT => {
                    return Some(StateSort::Lobby(LobbyState::new(LobbySettings {
                        lobby_sort: LobbySort::Online(self.lobby_id),
                        ..Default::default()
                    })));
                }
                _ => (),
            }
        }

        None
    }
}
