use std::{collections::HashMap, f64::consts::TAU};

use shared::{Board, BoardStyle, GameResult, Level, LobbySettings, Mage, Position, PowerUp, Team};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{tutorial::TUTORIAL_CODE, Game, MainMenu, State, Tutorial};

pub(super) const TUTORIAL_POSITION: (isize, isize) = (0, 1);
const DIRS: [(isize, isize); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];
use crate::{
    app::{
        Alignment, App, AppContext, ButtonElement, ClipId, Interface, LabelTheme, LabelTrim,
        Particle, ParticleSort, ParticleSystem, Pointer, StateSort, UIElement, UIEvent,
    },
    draw::{
        draw_board, draw_mage, draw_powerup, draw_sprite, draw_text, draw_text_centered,
        text_length,
    },
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
    title_visible: bool,
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

            if let Some(PreviewEntity::Mage(_)) = &preview[dx + dy * 2] {
                if (dx + dy) % 2 == 0 {
                    preview[dx + dy * 2] = Some(PreviewEntity::PowerUp(*powerup));
                }
            }
        }

        LevelPortal {
            level,
            title,
            title_visible: false,
            status,
            preview,
        }
    }

    fn draw_background(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        _particle_system: &mut ParticleSystem,
        (x, y): (isize, isize),
        _frame: u64,
    ) -> Result<(), JsValue> {
        context.translate(x as f64 * 128.0, y as f64 * 128.0)?;

        let (sx, sy) = portal_atlas_offset(&self.level.board.style);
        draw_sprite(context, atlas, sx, sy, 64.0, 64.0, -32.0, -32.0)?;

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
            let bounce = ((frame * 0.2).sin() * 8.0, (frame * 0.1).cos() * 8.0);

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
                        (x * 4) as f64 - 0.5 + (frame * 0.2).sin() * 0.5,
                        (y * 4) as f64 - 1.0 + (frame * 0.1).cos() * 0.5,
                    ),
                    (d.cos() * v, d.sin() * v),
                    (js_sys::Math::random() * 20.0) as u64,
                    ParticleSort::Shield,
                ));
            }
        }

        if self.status == PortalStatus::Locked {
            context.set_global_alpha(0.25);
        }

        draw_text_centered(
            context,
            atlas,
            16.0,
            60.0,
            if self.title_visible {
                &self.title
            } else {
                "???"
            },
        )?;
        context.set_global_alpha(1.0);

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
    sparkle_due: bool,
}

impl ArenaMenu {
    pub fn at_position(position: (isize, isize), newly_won: bool) -> ArenaMenu {
        let mut arena_menu = ArenaMenu {
            pan_offset: (-position.0 as f64 * 128.0, -position.1 as f64 * 128.0),
            ..Default::default()
        };

        let sparkle_due = if let Some(portal) = arena_menu.level_portals.get(&position) {
            portal.status == PortalStatus::Won
        } else {
            false
        };

        arena_menu.sparkle_due = newly_won && sparkle_due;

        arena_menu
    }

    fn draw_star_counter(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        let earned = self
            .level_portals
            .values()
            .filter(|portal| portal.status == PortalStatus::Won)
            .count()
            .to_string();
        let total = self.level_portals.len().to_string();

        context.save();
        // Stay in the canvas corner, independent of map panning and board centering.
        context.translate(
            8.0 - app_context.canvas_settings.padding_x() as f64,
            8.0 - app_context.canvas_settings.padding_y() as f64,
        )?;
        let bob = ((app_context.frame as f64 * 0.06).sin() * 2.0).round();
        draw_sprite(context, atlas, 32.0, 320.0, 32.0, 32.0, 0.0, bob)?;

        // Anchor the counts around the slash as their digit counts change.
        draw_text(
            context,
            atlas,
            44.0 - text_length(&earned) as f64,
            9.0,
            &earned,
        )?;
        draw_text(context, atlas, 44.0, 12.0, "/")?;
        context.set_filter("brightness(0.55)");
        draw_text(context, atlas, 52.0, 16.0, &total)?;
        context.restore();
        Ok(())
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
            for style in [
                BoardStyle::Grass,
                BoardStyle::Desert,
                BoardStyle::Flesh,
                BoardStyle::Crust,
                BoardStyle::Eldritch,
            ] {
                let (sx, sy) = portal_atlas_offset(&style);
                draw_board(atlas, sx, sy, &Board::unchecked(2, 2, style), 2, 2)?;
            }
        }

        let drag_offset = self.drag_offset(pointer);

        context.save();

        context.translate(
            128.0 + self.pan_offset.0 + drag_offset.0,
            128.0 + self.pan_offset.1 + drag_offset.1,
        )?;

        // context.scale(0.25, 0.25)?;

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

        self.draw_star_counter(interface_context, atlas, app_context)?;

        Ok(())
    }

    fn tick(
        &mut self,
        _text_input: &HtmlInputElement,
        app_context: &AppContext,
    ) -> Option<StateSort> {
        let pointer = &app_context.pointer;
        let pointer_floc = tuple_as!(pointer.location, f64);

        let previous_selected_position = self.level_position();

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

        if let Some(UIEvent::ButtonClick(BUTTON_BACK, clip_id)) = self.interface.tick(pointer) {
            app_context.audio_system.play_clip_option(clip_id);

            return Some(StateSort::MainMenu(MainMenu::default()));
        } else if let Some(UIEvent::ButtonClick(BUTTON_BATTLE, clip_id)) =
            self.button_battle.tick(pointer)
        {
            app_context.audio_system.play_clip_option(clip_id);

            let selected_position = self.level_position();
            let selected_level = self.level_portals.get(&selected_position);

            if let Some(portal) = selected_level {
                if portal.is_available() {
                    if selected_position == TUTORIAL_POSITION {
                        return Some(StateSort::Tutorial(Tutorial::campaign()));
                    }
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

        let selected_position = self.level_position();

        if selected_position != previous_selected_position {
            app_context.audio_system.play_clip(ClipId::LevelEnter);
        }
        if self.sparkle_due {
            app_context.audio_system.play_clip(ClipId::StarSparkle);
            self.sparkle_due = false;
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

        let level_portals = campaign_portals(|code| App::kv_get(code) == "win");
        let tutorial_done = level_portals[&TUTORIAL_POSITION].status == PortalStatus::Won;

        ArenaMenu {
            interface: root_element,
            button_locked,
            button_battle,
            particle_system: ParticleSystem::default(),
            pan_offset: if tutorial_done {
                (0.0, 0.0)
            } else {
                (0.0, -128.0)
            },
            pan_target: None,
            pan_start: None,
            board_dirty: true,
            sparkle_due: false,
            level_portals,
        }
    }
}

// Board styles are presentation metadata; level codes (and saved progress keys) stay stable.
fn campaign_style(column: isize) -> BoardStyle {
    match column {
        ..=2 => BoardStyle::Grass,
        3..=4 => BoardStyle::Desert,
        5 => BoardStyle::Flesh,
        6..=7 => BoardStyle::Crust,
        _ => BoardStyle::Eldritch,
    }
}

fn portal_atlas_offset(style: &BoardStyle) -> (f64, f64) {
    let index = match style {
        BoardStyle::Grass | BoardStyle::Teleport => 0,
        BoardStyle::Desert => 1,
        BoardStyle::Flesh => 2,
        BoardStyle::Crust => 3,
        BoardStyle::Eldritch => 4,
    };
    (256.0 + (index % 4) as f64 * 64.0, (index / 4) as f64 * 64.0)
}

fn campaign_portals(completed: impl Fn(&str) -> bool) -> HashMap<(isize, isize), LevelPortal> {
    let mut level_portals = HashMap::new();

    level_portals.insert(
        (0, 0),
        LevelPortal::from_level(
            "hg12g014cm0j800".into(),
            "Basics I".to_string(),
            PortalStatus::Locked,
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

    #[cfg(not(feature = "demo"))]
    {
        level_portals.insert(
            (3, -1),
            LevelPortal::from_level(
                "pg32a0j4gm148t818h602h1g092900j409r06h03".into(),
                "Patterns I".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (4, -1),
            LevelPortal::from_level(
                "pg2620a48m1m8c038ht02h04gg1jr0wg0d406".into(),
                "Patterns II".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (5, -1),
            LevelPortal::from_level(
                "pg3220j4g41m8h818gr06h4g052780j400".into(),
                "Patterns III".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (4, 0),
            LevelPortal::from_level(
                "h0120124d42480t40e204102".into(),
                "Diagonals I".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (4, 1),
            LevelPortal::from_level(
                "f02220t4840m8e018hc06h04a014g0sg0cm04".into(),
                "Diagonals II".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (4, 2),
            LevelPortal::from_level(
                "bg3200240g248h038gcg6h2s0h23t02408r04b02".into(),
                "Diagonals III".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (4, 3),
            LevelPortal::from_level(
                "k036202444148h818ha02h1r0127g0j40m604k01dg1jr0wc08".into(),
                "Diagonals IV".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (5, -2),
            LevelPortal::from_level(
                "j02620t4441m8c038hr06h055g1g00wg0dj06j01".into(),
                "Beams I".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (5, -3),
            LevelPortal::from_level(
                "eg3020t4c40489818gr02h0m0d2780240gp06a03d00pr08".into(),
                "Beams II".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (6, -3),
            LevelPortal::from_level(
                "qg22j0t4h41m8d038ja06h04gg13g0mr04j02".into(),
                "Beams III".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (5, 2),
            LevelPortal::from_level(
                "x01420a4900m81a402204903rg1680r".into(),
                "Shields I".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (6, 2),
            LevelPortal::from_level(
                "xg2420a4r40m9b018gp02h06x00080140f406t02gg10".into(),
                "Shields II".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (7, 2),
            LevelPortal::from_level(
                "j02280j4500m8t818hpg4h025g06800".into(),
                "Shields III".to_string(),
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
            (6, -1),
            LevelPortal::from_level(
                "hg2680t44m048a028hmg2h04000gr0mc06004".into(),
                "Challenge II".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (7, -3),
            LevelPortal::from_level(
                "q03220t4840m98828gw02h2r0d2bg0j40x804j03dg0k00s80a807200".into(),
                "Challenge III".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (7, 1),
            LevelPortal::from_level(
                "qg3200t4000m90048jeg6h5x0523t1241e606c03701s00wm02206j015g1k80h802404".into(),
                "Challenge IV".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (7, -2),
            LevelPortal::from_level(
                "t04420a4041m90818k0g6h2g052900a4t01m84038g2tr0n80cm06902d00g".into(),
                "Rite I".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (7, -1),
            LevelPortal::from_level(
                "zg2220t4r4048f008ke06h0chg1pr0wr0b406w03j01qg0340ba06d03gg02g0r".into(),
                "Rite II".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (7, 0),
            LevelPortal::from_level(
                "pg3820a44m2482808jp00h4g0h2380a410r04000m01r80nm00a06a03hg1g".into(),
                "Rite III".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (8, 0),
            LevelPortal::from_level(
                "pg3820a44m2482808jp00h4g0h2380a410r04000m01r80nm00a06a03hg1g".into(),
                "Rite IV".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
            (8, -1),
            LevelPortal::from_level(
                "zg322024w42499828hw04h6w0h25r02410t05j02n01j80vg0et00k01v01g".into(),
                "Ascension I".to_string(),
                PortalStatus::Locked,
            ),
        );

        level_portals.insert(
                (9, -1),
                LevelPortal::from_level(
                    "zg4200t4000m90048kg00h4x0d2bt0a47m249z808g78r0sg0cw07403jg0f80w40d403m025g1k80h802405b03".into(),
                    "Ascension II".to_string(),
                    PortalStatus::Locked,
                ),
            );
    }

    level_portals.insert(
        TUTORIAL_POSITION,
        LevelPortal::from_level(
            TUTORIAL_CODE.into(),
            "Tutorial".to_string(),
            PortalStatus::Unlocked,
        ),
    );

    let tutorial_done = completed(&Level::from(TUTORIAL_CODE).as_code());
    for (position, portal) in &mut level_portals {
        portal.level.board.style = campaign_style(position.0);
        portal.status = if (*position == TUTORIAL_POSITION || tutorial_done)
            && completed(&portal.level.as_code())
        {
            PortalStatus::Won
        } else if *position == TUTORIAL_POSITION {
            PortalStatus::Unlocked
        } else {
            PortalStatus::Locked
        };
    }

    let to_unlock: Vec<_> = level_portals
        .keys()
        .copied()
        .filter(|position| {
            tutorial_done
                && DIRS.iter().any(|dir| {
                    level_portals
                        .get(&(position.0 + dir.0, position.1 + dir.1))
                        .is_some_and(|neighbour| neighbour.status == PortalStatus::Won)
                })
        })
        .collect();
    for position in to_unlock {
        let portal = level_portals.get_mut(&position).unwrap();
        if portal.status == PortalStatus::Locked {
            portal.status = PortalStatus::Unlocked;
        }
    }

    let visible: Vec<_> = level_portals
        .iter()
        .filter_map(|(position, portal)| {
            (portal.is_available()
                || DIRS.iter().any(|dir| {
                    level_portals
                        .get(&(position.0 + dir.0, position.1 + dir.1))
                        .is_some_and(LevelPortal::is_available)
                }))
            .then_some(*position)
        })
        .collect();
    for position in visible {
        level_portals.get_mut(&position).unwrap().title_visible = true;
    }
    level_portals
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn tutorial_is_the_only_entry_and_gates_existing_campaign_saves() {
        let portals = campaign_portals(|_| false);
        assert!(portals[&TUTORIAL_POSITION].is_available());
        assert_eq!(
            portals
                .values()
                .filter(|portal| portal.is_available())
                .count(),
            1
        );
        assert_eq!(
            portals
                .values()
                .filter(|portal| portal.title == "Tutorial")
                .count(),
            1
        );
        assert!(portals[&(0, 0)].title_visible);
        assert!(!portals[&(1, 0)].title_visible);

        let tutorial_code = portals[&TUTORIAL_POSITION].level.as_code();
        let old_save = campaign_portals(|code| code != tutorial_code);
        assert_eq!(
            old_save
                .values()
                .filter(|portal| portal.is_available())
                .count(),
            1
        );
    }

    #[test]
    fn tutorial_completion_unlocks_basics_and_names_reveal_one_step_ahead() {
        let tutorial_code = Level::from(TUTORIAL_CODE).as_code();
        let portals = campaign_portals(|code| code == tutorial_code);
        assert!(portals[&(0, 0)].status == PortalStatus::Unlocked);
        assert!(portals[&(1, 0)].status == PortalStatus::Locked);
        assert!(portals[&(1, 0)].title_visible);
        assert!(!portals[&(2, 0)].title_visible);

        let basics_code = portals[&(0, 0)].level.as_code();
        let portals = campaign_portals(|code| code == tutorial_code || code == basics_code);
        assert!(portals[&(1, 0)].status == PortalStatus::Unlocked);
        assert!(portals[&(2, 0)].title_visible);
        assert!(!portals[&(2, -1)].title_visible);
    }

    #[test]
    fn all_levels_remain_reachable_from_the_tutorial() {
        let mut completed = HashSet::new();
        let count = campaign_portals(|_| false).len();
        for _ in 0..count {
            let portals = campaign_portals(|code| completed.contains(code));
            completed.extend(
                portals
                    .values()
                    .filter(|portal| portal.is_available())
                    .map(|portal| portal.level.as_code()),
            );
        }
        assert!(campaign_portals(|code| completed.contains(code))
            .values()
            .all(|portal| portal.status == PortalStatus::Won));
    }

    #[test]
    fn map_and_battles_share_styles_without_changing_save_keys() {
        let portals = campaign_portals(|_| true);
        let mut atlas_cells = HashSet::new();
        for portal in portals.values() {
            let level = &portal.level;
            let game = shared::Game::new(level, false).unwrap();
            assert_eq!(
                game.board().style.sprite_offset(),
                level.board.style.sprite_offset()
            );
            let mut original = level.clone();
            original.board.style = BoardStyle::Grass;
            assert_eq!(original.as_code(), level.as_code());
            let (x, y) = portal_atlas_offset(&level.board.style);
            assert!(x >= 256.0 && x + 64.0 <= 512.0 && y + 64.0 <= 256.0);
            atlas_cells.insert((x as usize, y as usize));
        }
        #[cfg(not(feature = "demo"))]
        assert_eq!(atlas_cells.len(), 5);
        #[cfg(feature = "demo")]
        assert_eq!(atlas_cells.len(), 1);
    }
}
