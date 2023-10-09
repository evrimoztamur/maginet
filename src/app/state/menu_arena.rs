use std::{
    collections::HashMap,
    f64::consts::{PI, TAU},
};

use shared::{Board, GameResult, Level, LobbySettings, Mage, Position, PowerUp, Team};
use wasm_bindgen::JsValue;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{Game, MainMenu, State};
use crate::{
    app::{
        Alignment, App, AppContext, ButtonElement, Interface, LabelTheme, LabelTrim, Particle,
        ParticleSort, ParticleSystem, Pointer, StateSort, UIElement, UIEvent,
    },
    draw::{draw_board, draw_mage, draw_powerup, draw_sprite, draw_text_centered},
    tuple_as,
};

enum PreviewEntity {
    Mage(Mage),
    PowerUp(PowerUp),
}

#[derive(PartialEq, Eq)]
enum PortalStatus {
    Locked,
    Unlocked,
    Won,
}

struct LevelPortal {
    level: Level,
    status: PortalStatus,
    title: String,
    preview: [Option<PreviewEntity>; 4],
}

impl LevelPortal {
    fn from_level(level: Level, title: String, status: PortalStatus) -> LevelPortal {
        let mut preview = [None, None, None, None];

        for mage in &level.mages {
            let dx = (mage.position.0 >= level.board.width as i8 / 2) as usize;
            let dy = (mage.position.1 >= level.board.height as i8 / 2) as usize;

            if preview[dx + dy * 2].is_none() {
                preview[dx + dy * 2] = Some(PreviewEntity::Mage(mage.clone()));
            }
        }

        for (position, powerup) in &level.powerups {
            let dx = (position.0 >= level.board.width as i8 / 2) as usize;
            let dy = (position.1 >= level.board.height as i8 / 2) as usize;

            if preview[dx + dy * 2].is_none() {
                preview[dx + dy * 2] = Some(PreviewEntity::PowerUp(*powerup));
            }

            if let Some(PreviewEntity::Mage(mage)) = &preview[dx + dy * 2] {
                if (dx + dy) % 2 == 0 {
                    preview[dx + dy * 2] = Some(PreviewEntity::PowerUp(*powerup));
                }
            }
        }

        LevelPortal {
            level,
            title,
            status,
            preview,
        }
    }

    fn draw_background(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        particle_system: &mut ParticleSystem,
        (x, y): (isize, isize),
        frame: u64,
    ) -> Result<(), JsValue> {
        context.translate(x as f64 * 128.0, y as f64 * 128.0)?;

        draw_sprite(context, atlas, 256.0, 0.0, 64.0, 64.0, -32.0, -32.0)?;

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
        context.translate(x as f64 * 128.0 - 16.0, y as f64 * 128.0 - 16.0)?;

        for (i, preview) in self.preview.iter().enumerate() {
            context.save();
            context.translate((i % 2) as f64 * 32.0, (i / 2) as f64 * 32.0)?;
            match preview {
                Some(PreviewEntity::Mage(mage)) => draw_mage(
                    context,
                    atlas,
                    mage,
                    frame,
                    shared::Team::Red,
                    self.is_available(),
                    if self.status == PortalStatus::Won {
                        Some(GameResult::Win(Team::Red))
                    } else {
                        None
                    },
                )?,
                Some(PreviewEntity::PowerUp(powerup)) => {
                    if let Some(particle_sort) = ParticleSort::for_powerup(powerup) {
                        for _ in 0..1 {
                            let d = js_sys::Math::random() * std::f64::consts::TAU;
                            let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.05;
                            particle_system.add(Particle::new(
                                (
                                    (i as isize % 2 + x * 4) as f64 - 1.0,
                                    (i as isize / 2 + y * 4) as f64 - 1.0,
                                ),
                                (d.cos() * v, d.sin() * v),
                                (js_sys::Math::random() * 20.0) as u64,
                                particle_sort,
                            ));
                        }
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

        if self.status == PortalStatus::Won {
            let frame = frame as f64 + x as f64 * 7.0 + y as f64 * 13.0;
            let bounce = (
                (frame as f64 * 0.2).sin() * 8.0,
                (frame as f64 * 0.1).cos() * 8.0,
            );

            draw_sprite(
                context,
                atlas,
                32.0,
                320.0,
                32.0,
                32.0,
                0.0 + bounce.0.round(),
                -16.0 + bounce.1.round(),
            )?;

            for _ in 0..(frame as i64 / 4) % 2 {
                let d = js_sys::Math::random() * TAU;
                let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;
                particle_system.add(Particle::new(
                    (
                        (x * 4) as f64 - 0.5 + (frame as f64 * 0.2).sin() * 0.5,
                        (y * 4) as f64 - 1.0 + (frame as f64 * 0.1).cos() * 0.5,
                    ),
                    (d.cos() * v, d.sin() * v),
                    (js_sys::Math::random() * 20.0) as u64,
                    ParticleSort::Shield,
                ));
            }
        }

        draw_text_centered(context, atlas, 16.0, 60.0, &self.title)?;

        Ok(())
    }

    fn is_available(&self) -> bool {
        match self.status {
            PortalStatus::Locked => false,
            PortalStatus::Unlocked => true,
            PortalStatus::Won => true,
        }
    }
}

pub struct ArenaMenu {
    interface: Interface,
    button_locked: ButtonElement,
    button_battle: ButtonElement,
    particle_system: ParticleSystem,
    pan_offset: (f64, f64),
    pan_target: Option<(f64, f64)>,
    pan_start: Option<(f64, f64)>,
    level_portals: HashMap<(isize, isize), LevelPortal>,
    board_dirty: bool,
}

impl ArenaMenu {
    pub fn at_position(position: (isize, isize)) -> ArenaMenu {
        ArenaMenu {
            pan_offset: (-position.0 as f64 * 128.0, -position.1 as f64 * 128.0),
            ..Default::default()
        }
    }

    fn drag_offset(&self, pointer: &Pointer) -> (f64, f64) {
        let pointer_floc = tuple_as!(pointer.location, f64);

        if let Some(pan_start) = self.pan_start {
            (pointer_floc.0 - pan_start.0, pointer_floc.1 - pan_start.1)
        } else {
            (0.0, 0.0)
        }
    }

    fn level_position(&self) -> (isize, isize) {
        (
            (-self.pan_offset.0 / 128.0).round() as isize,
            (-self.pan_offset.1 / 128.0).round() as isize,
        )
    }
}

const BUTTON_BATTLE: usize = 20;
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
            draw_board(
                atlas,
                256.0,
                0.0,
                &Board::unchecked(2, 2, shared::BoardStyle::Grass),
                2,
                2,
            )
            .unwrap();
        }

        let drag_offset = self.drag_offset(pointer);

        context.save();

        context.translate(
            128.0 + self.pan_offset.0 + drag_offset.0,
            128.0 + self.pan_offset.1 + drag_offset.1,
        )?;

        if self.pan_offset.0 > 0.0 {
            self.pan_offset.0 -= self.pan_offset.0 * 0.25;
        }

        if let Some(pan_target) = self.pan_target {
            self.pan_offset.0 += (pan_target.0 - self.pan_offset.0) * 0.25;
            self.pan_offset.1 += (pan_target.1 - self.pan_offset.1) * 0.25;
        } else {
            self.pan_offset.0 +=
                ((self.pan_offset.0 / 128.0).round() * 128.0 - self.pan_offset.0) * 0.25;
            self.pan_offset.1 +=
                ((self.pan_offset.1 / 128.0).round() * 128.0 - self.pan_offset.1) * 0.25;
        }

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

        let selected_position = self.level_position();

        for (offset, portal) in &self.level_portals {
            context.save();
            portal.draw(context, atlas, &mut self.particle_system, *offset, frame)?;

            if offset == &selected_position {}

            context.restore();
        }

        context.restore();

        self.interface
            .draw(interface_context, atlas, pointer, frame)?;

        let selected_level = self.level_portals.get(&selected_position);

        if let Some(portal) = selected_level {
            if portal.is_available() {
                self.button_battle.draw(context, atlas, pointer, frame)?
            } else {
                self.button_locked.draw(context, atlas, pointer, frame)?
            }
        }

        Ok(())
    }

    fn tick(
        &mut self,
        _text_input: &HtmlInputElement,
        app_context: &AppContext,
    ) -> Option<StateSort> {
        let pointer = &app_context.pointer;
        let pointer_floc = tuple_as!(pointer.location, f64);

        if let Some(UIEvent::ButtonClick(BUTTON_BACK)) = self.interface.tick(pointer) {
            return Some(StateSort::MainMenu(MainMenu::default()));
        } else if let Some(UIEvent::ButtonClick(BUTTON_BATTLE)) = self.button_battle.tick(pointer) {
            let selected_position = self.level_position();
            let selected_level = self.level_portals.get(&selected_position);

            if let Some(portal) = selected_level {
                if portal.is_available() {
                    return Some(StateSort::Game(Game::new(LobbySettings {
                        lobby_sort: shared::LobbySort::LocalAI,
                        loadout_method: shared::LoadoutMethod::Arena(
                            portal.level.clone(),
                            selected_position,
                        ),
                        ..Default::default()
                    })));
                }
            }
        } else if pointer.clicked() {
            self.pan_start = Some(pointer_floc);
            self.pan_target = None;
        } else if !pointer.button && self.pan_start.is_some() {
            let drag_offset = self.drag_offset(pointer);

            let lloc = (
                ((-self.pan_offset.0 + (pointer_floc.0 - 128.0)) / 128.0).round() as isize,
                ((-self.pan_offset.1 + (pointer_floc.1 - 128.0)) / 128.0).round() as isize,
            );

            if drag_offset.0.hypot(drag_offset.1) < 3.0 {
                if self.level_portals.get(&lloc).is_some() {
                    self.pan_target = Some((
                        -((-self.pan_offset.0 + pointer_floc.0 - 128.0) / 128.0).round() * 128.0,
                        -((-self.pan_offset.1 + pointer_floc.1 - 128.0) / 128.0).round() * 128.0,
                    ));
                }
            } else {
                self.pan_offset.0 += drag_offset.0;
                self.pan_offset.1 += drag_offset.1;
            }

            self.pan_start = None;
        }

        None
    }
}

impl Default for ArenaMenu {
    fn default() -> ArenaMenu {
        let button_battle = ButtonElement::new(
            (64, 192),
            (128, 24),
            BUTTON_BATTLE,
            LabelTrim::Glorious,
            LabelTheme::Action,
            crate::app::ContentElement::Text("Battle".to_string(), Alignment::Center),
        );

        let button_locked = ButtonElement::new(
            (68, 192),
            (120, 24),
            BUTTON_BATTLE,
            LabelTrim::Round,
            LabelTheme::Disabled,
            crate::app::ContentElement::Text("Locked".to_string(), Alignment::Center),
        );

        let button_back = ButtonElement::new(
            (84, 224),
            (88, 16),
            BUTTON_BACK,
            LabelTrim::Return,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Back".to_string(), Alignment::Center),
        );

        let root_element = Interface::new(vec![button_back.boxed()]);

        let mut level_portals = HashMap::new();

        level_portals.insert(
            (0, 0),
            LevelPortal::from_level(
                "hg12g014cm0j800".into(),
                "Basics I".to_string(),
                PortalStatus::Unlocked,
            ),
        );
        // 1v1 basic

        level_portals.insert(
            (1, 0),
            LevelPortal::from_level(
                "e01jg1148m0j8k834g00".into(),
                "Basics II".to_string(),
                PortalStatus::Locked,
            ),
        );
        // 1v2 basic

        level_portals.insert(
            (2, 0),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04900".into(),
                "Basics III".to_string(),
                PortalStatus::Locked,
            ),
        );
        // 2v2 easy

        level_portals.insert(
            (2, -1),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Basics IV".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (3, -1),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Patterns I".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (4, -1),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Patterns II".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (5, -1),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Patterns III".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (4, 0),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Diagonals I".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (4, 1),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Diagonals II".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (4, 2),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Diagonals III".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (5, -2),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Beams I".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (5, -3),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Beams II".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (6, -2),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Shields I".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (7, -2),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Shields II".to_string(),
                PortalStatus::Locked,
            ),
        );
        level_portals.insert(
            (2, 1),
            LevelPortal::from_level(
                "hg2280a4d40490008g6g2h02cg12g00".into(),
                "Challenge I".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (3, 1),
            LevelPortal::from_level(
                "j012809mdm0k81a809807303801g80r".into(),
                "Tutorial".to_string(),
                PortalStatus::Locked,
            ),
        );
        level_portals.insert(
            (5, 2),
            LevelPortal::from_level(
                "hg2680t44m048a028hmg2h04000gr0mc06004".into(),
                "Challenge II".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (4, -3),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Challenge III".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (7, -1),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Challenge IV".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (6, 2),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Rite I".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (7, 2),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Rite II".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (7, 0),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Rite III".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (7, 1),
            LevelPortal::from_level(
                "j0228014cm0j8v804gp04906201g00s80dm07403g01g".into(),
                "Ascension".to_string(),
                PortalStatus::Locked,
            ),
        );

        // Update portal locking statuses based on the KV-store.

        for portal in level_portals.values_mut() {
            let level_code = portal.level.as_code();
            let level_result = App::kv_get(&level_code);

            if level_result == "win" {
                portal.status = PortalStatus::Won;
            }
        }

        // For each portal with an adjacent won portal, unlock it.

        const DIRS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

        let mut to_unlock = Vec::new();

        for position in level_portals.keys() {
            for dir in DIRS {
                let neighbour = level_portals.get(&(position.0 + dir.0, position.1 + dir.1));

                if let Some(neighbour) = neighbour {
                    if neighbour.status == PortalStatus::Won {
                        to_unlock.push(*position);
                    }
                }
            }
        }

        for position in to_unlock {
            let portal = level_portals.get_mut(&position);

            if let Some(portal) = portal {
                if portal.status == PortalStatus::Locked {
                    portal.status = PortalStatus::Unlocked;
                }
            }
        }

        ArenaMenu {
            interface: root_element,
            button_locked,
            button_battle,
            particle_system: ParticleSystem::default(),
            pan_offset: (0.0, 0.0),
            pan_target: None,
            pan_start: None,
            board_dirty: true,
            level_portals,
        }
    }
}
