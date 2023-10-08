use std::collections::HashMap;

use shared::{Level, Mage, Position, PowerUp};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{MainMenu, State};
use crate::{
    app::{
        Alignment, AppContext, ButtonElement, Interface, LabelTheme, LabelTrim, Particle,
        ParticleSort, ParticleSystem, Pointer, StateSort, UIElement, UIEvent,
    },
    draw::{draw_board, draw_mage, draw_powerup, draw_sprite},
    tuple_as,
};

enum PreviewEntity {
    Mage(Mage),
    PowerUp(PowerUp),
}

struct LevelPortal {
    level: Level,
    preview: [Option<PreviewEntity>; 4],
}

impl LevelPortal {
    fn from_level(level: Level) -> LevelPortal {
        let mut preview = [None, None, None, None];

        for mage in &level.mages {
            let dx = (mage.position.0 < level.board.width as i8 / 2) as usize;
            let dy = (mage.position.1 < level.board.height as i8 / 2) as usize;

            if preview[dx + dy * 2].is_none() {
                preview[dx + dy * 2] = Some(PreviewEntity::Mage(mage.clone()));
            }
        }

        for (position, powerup) in &level.powerups {
            let dx = (position.0 < level.board.width as i8 / 2) as usize;
            let dy = (position.1 < level.board.height as i8 / 2) as usize;

            if preview[dx + dy * 2].is_none() {
                preview[dx + dy * 2] = Some(PreviewEntity::PowerUp(*powerup));
            }

            if let Some(PreviewEntity::Mage(mage)) = &preview[dx + dy * 2] {
                if (mage.position.0 + position.0 + mage.position.1 + position.1) % 2 == 0 {
                    preview[dx + dy * 2] = Some(PreviewEntity::PowerUp(*powerup));
                }
            }
        }

        LevelPortal { level, preview }
    }

    fn draw_background(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        particle_system: &mut ParticleSystem,
        (x, y): (isize, isize),
        frame: u64,
    ) -> Result<(), JsValue> {
        context.translate(x as f64 * 96.0, y as f64 * 96.0)?;

        draw_sprite(context, atlas, 256.0, 0.0, 64.0, 64.0, 0.0, 0.0)?;

        Ok(())
    }

    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        particle_system: &mut ParticleSystem,
        (x, y): (isize, isize),
        frame: u64,
    ) -> Result<(), JsValue> {
        context.translate(x as f64 * 96.0, y as f64 * 96.0)?;

        for (i, preview) in self.preview.iter().enumerate() {
            context.save();
            context.translate((i % 2) as f64 * 32.0 + 16.0, (i / 2) as f64 * 32.0 + 16.0)?;
            match preview {
                Some(PreviewEntity::Mage(mage)) => {
                    draw_mage(context, atlas, mage, frame, shared::Team::Red, true, None)?
                }
                Some(PreviewEntity::PowerUp(powerup)) => {
                    for _ in 0..1 {
                        let d = js_sys::Math::random() * std::f64::consts::TAU;
                        let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.05;
                        particle_system.add(Particle::new(
                            (
                                (i as isize % 2 + x * 3) as f64,
                                (i as isize / 2 + y * 3) as f64,
                            ),
                            (d.cos() * v, d.sin() * v),
                            (js_sys::Math::random() * 20.0) as u64,
                            ParticleSort::for_powerup(powerup),
                        ));
                    }
                    draw_powerup(
                        context,
                        atlas,
                        &Position(i as i8 % 2, i as i8 / 2),
                        powerup,
                        frame,
                    )?
                }
                _ => (),
            }
            context.restore();
        }

        Ok(())
    }
}

pub struct ArenaMenu {
    interface: Interface,
    lobby_id: u16,
    particle_system: ParticleSystem,
    pan_offset: (f64, f64),
    pan_start: Option<(f64, f64)>,
    level_portals: HashMap<(isize, isize), LevelPortal>,
    board_dirty: bool,
}

impl ArenaMenu {
    fn drag_offset(&self, pointer: &Pointer) -> (f64, f64) {
        let pointer_floc = tuple_as!(pointer.location, f64);
        if let Some(pan_start) = self.pan_start {
            (pointer_floc.0 - pan_start.0, pointer_floc.1 - pan_start.1)
        } else {
            (0.0, 0.0)
        }
    }
}

const BUTTON_TELEPORT: usize = 20;
const BUTTON_BACK: usize = 21;
impl State for ArenaMenu {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        let frame = app_context.frame;
        let pointer = &app_context.pointer;

        if self.board_dirty {
            self.board_dirty = false;
            draw_board(atlas, 256.0, 0.0, 2, 2, 2, 2, (0, 0)).unwrap();
        }

        let drag_offset = self.drag_offset(pointer);

        context.save();

        context.translate(
            96.0 + self.pan_offset.0 + drag_offset.0,
            96.0 + self.pan_offset.1 + drag_offset.1,
        )?;

        if self.pan_offset.0 > 0.0 {
            self.pan_offset.0 -= self.pan_offset.0 * 0.25;
        }

        self.pan_offset.0 += ((self.pan_offset.0 / 96.0).round() * 96.0 - self.pan_offset.0) * 0.25;
        self.pan_offset.1 += ((self.pan_offset.1 / 96.0).round() * 96.0 - self.pan_offset.1) * 0.25;

        self.pan_offset.0 = self.pan_offset.0.floor();
        self.pan_offset.1 = self.pan_offset.1.floor();

        for (offset, portal) in &self.level_portals {
            context.save();
            portal.draw_background(context, atlas, &mut self.particle_system, *offset, frame)?;
            context.restore();
        }

        // draw_text(context, atlas, 8.0, 32.0, &format!("{:?}", self.pan_offset));
        // draw_text(context, atlas, 8.0, 48.0, &format!("{:?}", self.pan_start));

        self.particle_system.tick_and_draw(context, atlas, frame)?;

        for (offset, portal) in &self.level_portals {
            context.save();
            portal.draw(context, atlas, &mut self.particle_system, *offset, frame)?;
            context.restore();
        }

        context.restore();

        self.interface
            .draw(interface_context, atlas, pointer, frame)?;

        Ok(())
    }

    fn tick(
        &mut self,
        _text_input: &HtmlInputElement,
        app_context: &AppContext,
    ) -> Option<StateSort> {
        let pointer = &app_context.pointer;
        let pointer_floc = tuple_as!(pointer.location, f64);

        if let Some(UIEvent::ButtonClick(value)) = self.interface.tick(pointer) {
            match value {
                BUTTON_BACK => {
                    return Some(StateSort::MainMenu(MainMenu::default()));
                }
                _ => (),
            }
        } else if pointer.clicked() {
            self.pan_start = Some(pointer_floc);
        } else if !pointer.button {
            let drag_offset = self.drag_offset(pointer);
            self.pan_offset.0 += drag_offset.0;
            self.pan_offset.1 += drag_offset.1;
            self.pan_start = None;
        }

        None
    }
}

impl Default for ArenaMenu {
    fn default() -> ArenaMenu {
        let button_back = ButtonElement::new(
            (128 - 44, 224),
            (88, 16),
            BUTTON_BACK,
            LabelTrim::Return,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Back".to_string(), Alignment::Center),
        );

        let root_element = Interface::new(vec![button_back.boxed()]);

        let mut level_portals = HashMap::new();

        level_portals.insert((0, 0), LevelPortal::from_level("hg12g014cm0j800".into()));
        // 1v1 basic

        level_portals.insert(
            (1, 0),
            LevelPortal::from_level("e01jg1148m0j8k834g00".into()),
        );
        // 1v2 basic

        level_portals.insert(
            (2, 0),
            LevelPortal::from_level("j0228014cm0j8v804gp04900".into()),
        );
        // 2v2 easy

        level_portals.insert(
            (5, 1),
            LevelPortal::from_level("hg2280a4d40490008g6g2h02cg12g00".into()),
        );
        level_portals.insert(
            (5, 2),
            LevelPortal::from_level("hg2680t44m048a028hmg2h04000gr0mc06004".into()),
        );

        ArenaMenu {
            interface: root_element,
            lobby_id: 0,
            particle_system: ParticleSystem::default(),
            pan_offset: (0.0, 0.0),
            pan_start: None,
            board_dirty: true,
            level_portals,
        }
    }
}
