use std::mem;

use shared::{Board, Level, LobbySettings, Mage, Mages, Position, Team};
use wasm_bindgen::JsValue;
use web_sys::{console, CanvasRenderingContext2d, HtmlCanvasElement};

use super::{LobbyState, State};
use crate::{
    app::{
        Alignment, App, AppContext, ButtonElement, ConfirmButtonElement, Interface, LabelTheme,
        LabelTrim, Particle, ParticleSort, StateSort, ToggleButtonElement, UIElement, UIEvent,
        BOARD_SCALE,
    },
    draw::{
        draw_board, draw_crosshair, draw_mage, draw_mana, draw_particle, draw_spell_pattern,
        draw_sprite,
    },
    tuple_as, window,
};

#[derive(PartialEq)]
pub enum EditorSelection {
    Mage(Mage),
    Tile(Position),
    None,
}

pub struct EditorState {
    button_menu: ToggleButtonElement,
    interface: Interface,
    menu_interface: Interface,
    mage_interface: Interface,
    no_mage_interface: Interface,
    board: Board,
    board_dirty: bool,
    mages: Vec<Mage>,
    mage_index: usize,
    particles: Vec<Particle>,
    selection: EditorSelection,
}

const BUTTON_MENU: usize = 0;
const BUTTON_MODE_TOGGLE: usize = 10;
const BUTTON_SAVE: usize = 11;

const BUTTON_WIDTH_MINUS: usize = 20;
const BUTTON_WIDTH_PLUS: usize = 21;
const BUTTON_HEIGHT_MINUS: usize = 22;
const BUTTON_HEIGHT_PLUS: usize = 23;

const BUTTON_TEAM_LEFT: usize = 30;
const BUTTON_TEAM_RIGHT: usize = 31;
const BUTTON_SPELL_LEFT: usize = 32;
const BUTTON_SPELL_RIGHT: usize = 33;
const BUTTON_MANA_LEFT: usize = 34;
const BUTTON_MANA_RIGHT: usize = 35;
const BUTTON_DELETE: usize = 39;

const BUTTON_ADD: usize = 40;

const BUTTON_SIMULATE: usize = 50;
const BUTTON_LEAVE: usize = 100;

impl EditorState {
    pub fn new(board: Board, mut mages: Vec<Mage>) -> EditorState {
        let mut state = EditorState::default();

        state.mages = mages
            .iter_mut()
            .enumerate()
            .map(|(i, mage)| {
                mage.index = i;
                mage.clone()
            })
            .collect();
        state.mage_index = mages.len();
        state.board = board;

        state
    }

    pub fn with_level(level: Level) -> EditorState {
        EditorState::new(level.board, level.mages)
    }

    pub fn board_offset(&self) -> (i32, i32) {
        (
            ((8 - self.board.width) as i32 * BOARD_SCALE.0) / 2,
            ((8 - self.board.height) as i32 * BOARD_SCALE.1) / 2,
        )
    }

    pub fn is_interface_active(&self) -> bool {
        self.button_menu.selected()
    }

    pub fn level(&self) -> Level {
        Level::new(self.board.clone(), self.mages.clone(), Team::Red)
    }
}

impl State for EditorState {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        let board_scale = tuple_as!(BOARD_SCALE, f64);
        let board_offset = self.board_offset();

        let frame = app_context.frame;
        let pointer = &app_context.pointer;

        if self.board_dirty {
            self.board_dirty = false;
            draw_board(atlas, 256.0, 0.0, self.board.width, self.board.height, 8, 8).unwrap();
        }

        context.save();

        context.translate(-32.0, 0.0)?;

        draw_sprite(context, atlas, 256.0, 0.0, 256.0, 256.0, 0.0, 0.0)?;
        draw_sprite(context, atlas, 256.0, 256.0, 64.0, 64.0, 276.0, 8.0)?;

        context.translate(board_offset.0 as f64, board_offset.1 as f64)?;

        for particle in self.particles.iter_mut() {
            particle.tick();
            draw_particle(context, atlas, &particle, frame)?;
        }

        self.particles.drain_filter(|particle| !particle.is_alive());

        let mut mage_heap: Vec<&Mage> = self.mages.iter().collect();
        mage_heap.sort_by(|a, b| a.position.1.cmp(&b.position.1));

        // DRAW mages
        for mage in mage_heap {
            context.save();

            context.translate(
                16.0 + mage.position.0 as f64 * board_scale.0,
                16.0 + mage.position.1 as f64 * board_scale.1,
            )?;

            draw_mage(context, atlas, mage, frame, mage.team, true, None, true)?;

            context.restore();
        }

        let selected_tile = self.board.location_as_position(
            pointer.location,
            (board_offset.0 - 32, board_offset.1),
            BOARD_SCALE,
        );

        if let Some(selected_tile) = selected_tile {
            draw_crosshair(context, atlas, &selected_tile, (32.0, 32.0), frame)?;
        }

        let board_scale = tuple_as!(BOARD_SCALE, f64);
        let board_offset = tuple_as!(board_offset, f64);

        match &self.selection {
            EditorSelection::Mage(mage) => {
                if let Some(selected_tile) = selected_tile {
                    for position in &mage.targets(&self.board, &selected_tile) {
                        draw_crosshair(context, atlas, position, (80.0, 32.0), 0)?;
                    }
                }

                interface_context.save();
                interface_context.translate(
                    (pointer.location.0 as f64).clamp(
                        board_offset.0 - 16.0,
                        board_offset.0 - 48.0 + board_scale.0 * self.board.width as f64,
                    ),
                    (pointer.location.1 as f64).clamp(
                        board_offset.1 + 16.0,
                        board_offset.1 - 16.0 + board_scale.1 * self.board.height as f64,
                    ),
                )?;
                draw_mage(
                    interface_context,
                    atlas,
                    mage,
                    frame,
                    mage.team,
                    true,
                    None,
                    false,
                )?;
                interface_context.restore();

                interface_context.save();
                interface_context.translate(276.0, 40.0)?;
                draw_mage(
                    interface_context,
                    atlas,
                    mage,
                    frame,
                    mage.team,
                    true,
                    None,
                    true,
                )?;
                interface_context.restore();
            }
            EditorSelection::Tile(position) => {
                draw_crosshair(context, atlas, &position, (48.0, 32.0), frame)?;

                if let Some(mage) = self.mages.occupant(&position) {
                    for position in &mage.targets(&self.board, position) {
                        draw_crosshair(context, atlas, position, (80.0, 32.0), 0)?;
                    }

                    self.mage_interface
                        .draw(interface_context, atlas, pointer, frame)?;

                    interface_context.save();
                    interface_context.translate(276.0, 40.0)?;
                    draw_mage(
                        interface_context,
                        atlas,
                        mage,
                        frame,
                        mage.team,
                        true,
                        None,
                        true,
                    )?;

                    interface_context.translate(-20.0, 40.0)?;

                    draw_spell_pattern(interface_context, atlas, mage)?;

                    interface_context.translate(20.0, 44.0)?;

                    if mage.mana > 0 {
                        draw_mana(interface_context, atlas, mage)?;
                    } else {
                        draw_sprite(interface_context, atlas, 112.0, 16.0, 16.0, 16.0, -8.0, 4.0)?;
                    }

                    interface_context.restore();
                } else {
                    self.no_mage_interface
                        .draw(interface_context, atlas, pointer, frame)?;
                }
            }
            EditorSelection::None => (),
        }

        context.restore();

        self.interface
            .draw(interface_context, atlas, pointer, frame)?;

        self.button_menu
            .draw(interface_context, atlas, pointer, frame)?;

        if self.is_interface_active() {
            self.menu_interface
                .draw(interface_context, atlas, pointer, frame)?;
        }

        Ok(())
    }

    fn tick(&mut self, app_context: &AppContext) -> Option<StateSort> {
        let board_offset = self.board_offset();
        let pointer = &app_context.pointer;

        if self.is_interface_active() {
            if let Some(UIEvent::ButtonClick(value)) = self.menu_interface.tick(&pointer) {
                match value {
                    BUTTON_SIMULATE => {
                        let simulations = Level::simulate(
                            Level::new(self.board.clone(), self.mages.clone(), Team::Red),
                            5,
                            window().performance().unwrap().now() as u64,
                        );

                        console::log_1(
                            &format!(
                                "{}",
                                simulations
                                    .iter()
                                    .map(|game| {
                                        console::log_1(
                                            &format!(
                                                "{} {} {:?}",
                                                game.turns(),
                                                game.evaluate(),
                                                game
                                            )
                                            .into(),
                                        );
                                        game.evaluate()
                                    })
                                    .sum::<isize>()
                                    / 5
                            )
                            .into(),
                        );
                    }
                    _ => (),
                }
            }
        } else if let Some(UIEvent::ButtonClick(value)) = self.interface.tick(pointer) {
            match value {
                BUTTON_SAVE => {
                    App::save_level(0, self.level());
                }
                BUTTON_MODE_TOGGLE => {
                    return Some(StateSort::Lobby(LobbyState::new(LobbySettings {
                        lobby_sort: shared::LobbySort::Local,
                        loadout_method: shared::LoadoutMethod::Prefab(self.mages.clone()),
                        board: self.board.clone(),
                        ..Default::default()
                    })));
                }
                BUTTON_WIDTH_MINUS => {
                    let min_width = self
                        .mages
                        .iter()
                        .map(|mage| mage.position.0)
                        .reduce(|acc, e| acc.max(e))
                        .unwrap_or_default() as usize;

                    if self.board.width - 1 <= min_width {
                        for _ in 0..self.board.width * 5 {
                            let d = js_sys::Math::random() * std::f64::consts::TAU;
                            let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;
                            self.particles.push(Particle::new(
                                (
                                    self.board.width as f64 - 0.5,
                                    js_sys::Math::random() * self.board.height as f64 - 0.5,
                                ),
                                (-v, d.sin() * v * 0.1),
                                (js_sys::Math::random() * 40.0) as u64,
                                ParticleSort::Diagonals,
                            ));
                        }
                    } else {
                        if let Ok(board) = Board::new(self.board.width - 1, self.board.height) {
                            self.board = board;
                            self.board_dirty = true;
                        }
                    }
                }
                BUTTON_WIDTH_PLUS => {
                    if let Ok(board) = Board::new(self.board.width + 1, self.board.height) {
                        self.board = board;
                        self.board_dirty = true;
                    }
                }
                BUTTON_HEIGHT_MINUS => {
                    let min_height = self
                        .mages
                        .iter()
                        .map(|mage| mage.position.1)
                        .reduce(|acc, e| acc.max(e))
                        .unwrap_or_default() as usize;

                    if self.board.height - 1 <= min_height {
                        for _ in 0..self.board.height * 5 {
                            let d = js_sys::Math::random() * std::f64::consts::TAU;
                            let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;
                            self.particles.push(Particle::new(
                                (
                                    js_sys::Math::random() * self.board.width as f64 - 0.5,
                                    self.board.height as f64 - 0.5,
                                ),
                                (d.cos() * v * 0.1, -v),
                                (js_sys::Math::random() * 40.0) as u64,
                                ParticleSort::Diagonals,
                            ));
                        }
                    } else {
                        if let Ok(board) = Board::new(self.board.width, self.board.height - 1) {
                            self.board = board;
                            self.board_dirty = true;
                        }
                    }
                }
                BUTTON_HEIGHT_PLUS => {
                    if let Ok(board) = Board::new(self.board.width, self.board.height + 1) {
                        self.board = board;
                        self.board_dirty = true;
                    }
                }

                _ => (),
            }
        } else if let Some(UIEvent::ButtonClick(value)) = self.no_mage_interface.tick(pointer) {
            match value {
                BUTTON_ADD => match self.selection {
                    EditorSelection::Tile(position) => {
                        if !self.mages.occupied(&position) {
                            self.mages.push(Mage::new(
                                self.mage_index,
                                Team::Red,
                                shared::MageSort::Cross,
                                position,
                            ));
                            self.mage_index += 1;

                            for _ in 0..40 {
                                let d = js_sys::Math::random() * std::f64::consts::TAU;
                                let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;
                                self.particles.push(Particle::new(
                                    (position.0 as f64, position.1 as f64),
                                    (d.cos() * v * 2.0, d.sin() * v),
                                    (js_sys::Math::random() * 20.0) as u64,
                                    ParticleSort::Missile,
                                ));
                            }
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        } else if let Some(UIEvent::ButtonClick(value)) = self.mage_interface.tick(pointer) {
            match value {
                BUTTON_TEAM_LEFT => match self.selection {
                    EditorSelection::Tile(position) => {
                        if let Some(selected_mage) = self.mages.occupant_mut(&position) {
                            selected_mage.team = selected_mage.team.enemy();
                        }
                    }
                    _ => (),
                },
                BUTTON_TEAM_RIGHT => match self.selection {
                    EditorSelection::Tile(position) => {
                        if let Some(selected_mage) = self.mages.occupant_mut(&position) {
                            selected_mage.team = selected_mage.team.enemy();
                        }
                    }
                    _ => (),
                },
                BUTTON_SPELL_LEFT => match self.selection {
                    EditorSelection::Tile(position) => {
                        if let Some(selected_mage) = self.mages.occupant_mut(&position) {
                            *selected_mage = Mage::new(
                                selected_mage.index,
                                selected_mage.team,
                                selected_mage.sort.previous(),
                                selected_mage.position,
                            );
                        }
                    }
                    _ => (),
                },
                BUTTON_SPELL_RIGHT => match self.selection {
                    EditorSelection::Tile(position) => {
                        if let Some(selected_mage) = self.mages.occupant_mut(&position) {
                            *selected_mage = Mage::new(
                                selected_mage.index,
                                selected_mage.team,
                                selected_mage.sort.next(),
                                selected_mage.position,
                            );
                        }
                    }
                    _ => (),
                },
                BUTTON_MANA_LEFT => match self.selection {
                    EditorSelection::Tile(position) => {
                        if let Some(selected_mage) = self.mages.occupant_mut(&position) {
                            selected_mage.mana -= 1;
                        }
                    }
                    _ => (),
                },
                BUTTON_MANA_RIGHT => match self.selection {
                    EditorSelection::Tile(position) => {
                        if let Some(selected_mage) = self.mages.occupant_mut(&position) {
                            selected_mage.mana += 1;
                        }
                    }
                    _ => (),
                },
                BUTTON_DELETE => match self.selection {
                    EditorSelection::Tile(position) => {
                        self.mages.drain_filter(|mage| mage.position == position);

                        for _ in 0..40 {
                            let d = js_sys::Math::random() * std::f64::consts::TAU;
                            let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;
                            self.particles.push(Particle::new(
                                (
                                    position.0 as f64 + d.cos() * 1.25,
                                    position.1 as f64 + d.sin() * 1.25,
                                ),
                                (-d.cos() * v, -d.sin() * v),
                                (js_sys::Math::random() * 20.0) as u64,
                                ParticleSort::Missile,
                            ));
                        }
                    }
                    _ => (),
                },
                _ => (),
            }
        } else if let Some(selected_tile) = self.board.location_as_position(
            pointer.location,
            (board_offset.0 - 32, board_offset.1),
            BOARD_SCALE,
        ) {
            if pointer.clicked() {
                for _ in 0..10 {
                    let d = js_sys::Math::random() * std::f64::consts::TAU;
                    let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;
                    self.particles.push(Particle::new(
                        (selected_tile.0 as f64, selected_tile.1 as f64),
                        (d.cos() * v, d.sin() * v),
                        (js_sys::Math::random() * 20.0) as u64,
                        ParticleSort::Missile,
                    ));
                }

                if let Some(selected_mage) = self.mages.occupant(&selected_tile).cloned() {
                    match &mut self.selection {
                        EditorSelection::Tile(tile) if *tile == selected_tile => {
                            self.selection = EditorSelection::Mage(selected_mage);
                        }
                        EditorSelection::Mage(mage) => {
                            mage.position = selected_tile;
                            self.mages.push(mage.clone());

                            for _ in 0..40 {
                                let d = js_sys::Math::random() * std::f64::consts::TAU;
                                let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;
                                self.particles.push(Particle::new(
                                    (selected_tile.0 as f64, selected_tile.1 as f64),
                                    (d.cos() * v * 2.0, d.sin() * v),
                                    (js_sys::Math::random() * 20.0) as u64,
                                    ParticleSort::Missile,
                                ));
                            }

                            self.selection = EditorSelection::Mage(selected_mage);
                        }
                        _ => self.selection = EditorSelection::Tile(selected_tile),
                    }
                } else {
                    if let EditorSelection::Mage(mut selected_mage) =
                        mem::replace(&mut self.selection, EditorSelection::Tile(selected_tile))
                    {
                        selected_mage.position = selected_tile;
                        self.mages.push(selected_mage);

                        for _ in 0..40 {
                            let d = js_sys::Math::random() * std::f64::consts::TAU;
                            let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;
                            self.particles.push(Particle::new(
                                (selected_tile.0 as f64, selected_tile.1 as f64),
                                (d.cos() * v * 2.0, d.sin() * v),
                                (js_sys::Math::random() * 20.0) as u64,
                                ParticleSort::Missile,
                            ));
                        }
                    }
                }
            } else if pointer.alt_clicked() {
                self.selection = EditorSelection::None;
            }
        }

        match &mut self.selection {
            EditorSelection::Mage(selected_mage) => {
                self.mages
                    .drain_filter(|mage| mage.index == selected_mage.index);
            }
            EditorSelection::Tile(position) => {
                *position = self.board.clamp_position(*position);
            }
            EditorSelection::None => (),
        }

        self.button_menu.tick(pointer);

        None
    }
}

impl Default for EditorState {
    fn default() -> Self {
        let button_menu = ToggleButtonElement::new(
            (-60, 118),
            (20, 20),
            BUTTON_MENU,
            LabelTrim::Round,
            LabelTheme::Bright,
            crate::app::ContentElement::Sprite((112, 32), (16, 16)),
        );

        let button_mode_toggle = ButtonElement::new(
            (236, 228),
            (80, 24),
            BUTTON_MODE_TOGGLE,
            LabelTrim::Glorious,
            LabelTheme::Action,
            crate::app::ContentElement::Text("Battle".to_string(), Alignment::Center),
        );

        let button_save = ButtonElement::new(
            (244, 204),
            (64, 16),
            BUTTON_SAVE,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Save".to_string(), Alignment::Center),
        );

        let button_width_minus = ButtonElement::new(
            (82, 248),
            (12, 12),
            BUTTON_WIDTH_MINUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((88, 24), (8, 8)),
        );

        let button_width_plus = ButtonElement::new(
            (98, 248),
            (12, 12),
            BUTTON_WIDTH_PLUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((80, 24), (8, 8)),
        );

        let button_height_minus = ButtonElement::new(
            (216, 114),
            (12, 12),
            BUTTON_HEIGHT_MINUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((88, 24), (8, 8)),
        );

        let button_height_plus = ButtonElement::new(
            (216, 130),
            (12, 12),
            BUTTON_HEIGHT_PLUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((80, 24), (8, 8)),
        );

        let button_team_left = ButtonElement::new(
            (240, 122 - 92),
            (12, 20),
            BUTTON_TEAM_LEFT,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((64, 24), (8, 8)),
        );

        let button_team_right = ButtonElement::new(
            (300, 122 - 92),
            (12, 20),
            BUTTON_TEAM_RIGHT,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((72, 24), (8, 8)),
        );

        let button_spell_left = ButtonElement::new(
            (240, 122 - 38),
            (12, 32),
            BUTTON_SPELL_LEFT,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((64, 24), (8, 8)),
        );

        let button_spell_right = ButtonElement::new(
            (300, 122 - 38),
            (12, 32),
            BUTTON_SPELL_RIGHT,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((72, 24), (8, 8)),
        );

        let button_mana_left = ButtonElement::new(
            (244, 122 + 8),
            (12, 12),
            BUTTON_MANA_LEFT,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((64, 24), (8, 8)),
        );

        let button_mana_right = ButtonElement::new(
            (296, 122 + 8),
            (12, 12),
            BUTTON_MANA_RIGHT,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((72, 24), (8, 8)),
        );

        let button_delete = ButtonElement::new(
            (260, 160),
            (32, 20),
            BUTTON_DELETE,
            LabelTrim::Round,
            LabelTheme::Default,
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

        let button_add = ButtonElement::new(
            (260, 122 - 44),
            (32, 20),
            BUTTON_ADD,
            LabelTrim::Glorious,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((144, 32), (16, 16)),
        );

        let no_mage_interface = Interface::new(vec![Box::new(button_add)]);

        let root_element = Interface::new(vec![
            Box::new(button_mode_toggle),
            Box::new(button_save),
            Box::new(button_width_minus),
            Box::new(button_width_plus),
            Box::new(button_height_minus),
            Box::new(button_height_plus),
        ]);

        let button_simulate = ButtonElement::new(
            (96 - 44, 128 - 24),
            (88, 24),
            BUTTON_SIMULATE,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Simulate".to_string(), Alignment::Center),
        );

        let button_leave = ConfirmButtonElement::new(
            (96 - 36, 128 + 8),
            (72, 16),
            BUTTON_LEAVE,
            LabelTrim::Glorious,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Leave".to_string(), Alignment::Center),
        );

        let menu_interface =
            Interface::new(vec![Box::new(button_leave), Box::new(button_simulate)]);

        let level = App::load_level(0).unwrap_or_default();

        EditorState {
            button_menu,
            interface: root_element,
            mage_interface,
            menu_interface,
            no_mage_interface,
            board: level.board,
            mages: level.mages,
            mage_index: 0,
            particles: Vec::new(),
            selection: EditorSelection::None,
            board_dirty: true,
        }
    }
}
