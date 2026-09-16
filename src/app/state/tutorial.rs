pub(super) use shared::TUTORIAL_CODE;
use shared::{GameResult, Level, LoadoutMethod, LobbySettings, LobbySort, Position, PowerUp, Team};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{menu_arena::TUTORIAL_POSITION, ArenaMenu, Game, MainMenu, State};
use crate::{
    app::{
        Alignment::Center, AppContext, ButtonElement, ContentElement::Text, LabelTheme, LabelTrim,
        Particle, ParticleSort, StateSort, UIElement, BOARD_SCALE,
    },
    draw::{draw_label, draw_powerup, draw_text_centered, text_length},
    window,
};

#[derive(Debug, PartialEq)]
enum TutorialStage {
    Movement,
    Attacking,
    FinalBlow,
    Victory,
    Retry,
}

impl TutorialStage {
    fn for_game(game: &shared::Game) -> Self {
        match game.result() {
            Some(GameResult::Win(Team::Red)) => Self::Victory,
            Some(_) => Self::Retry,
            None if game.turns() == 0 => Self::Movement,
            None if can_finish(game) => Self::FinalBlow,
            None => Self::Attacking,
        }
    }
}

fn can_finish(game: &shared::Game) -> bool {
    if game.turn_for() != Team::Red
        || game.result().is_some()
        || game
            .iter_mages()
            .any(|mage| mage.team == Team::Blue && mage.mana > 1)
    {
        return false;
    }
    game.legal_turns().iter().any(|turn| {
        let mut next = game.clone();
        next.take_move(turn.0, turn.1).is_some()
            && next.result() == Some(GameResult::Win(Team::Red))
            && next
                .iter_mages()
                .all(|mage| mage.team != Team::Blue || !mage.is_alive())
    })
}

pub struct Tutorial {
    pub game_state: Game,
    tutorial_stage: TutorialStage,
    campaign: bool,
    item_slide: Option<usize>,
}

const ITEMS: [(PowerUp, &str, &[&str]); 3] = [
    (
        PowerUp::Diagonal,
        "Diagonal rune",
        &[
            "Pick up the green rune to move",
            "diagonally as well as up, down,",
            "left and right. Your spell stays",
            "the same. This lasts until you",
            "pick up another item.",
        ],
    ),
    (
        PowerUp::Shield,
        "Shield",
        &[
            "Enemies that move into your",
            "attack pattern lose 1 mana,",
            "even on their turn. You still",
            "take damage when hit. Lasts",
            "until you pick up another item.",
        ],
    ),
    (
        PowerUp::Beam,
        "Beam",
        &[
            "Move onto a beam to fire once.",
            "It hits every other mage in its",
            "row and column, including allies.",
            "It uses your turn and replaces",
            "your usual attack for that move.",
        ],
    ),
];

impl Tutorial {
    pub fn spark_board(&mut self) {
        let board_size = self.game_state.visual_game().board_size();
        for _ in 0..board_size.0 * 8 {
            let d = js_sys::Math::random() * std::f64::consts::TAU;
            let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.1;
            self.game_state.particle_system().add(Particle::new(
                (js_sys::Math::random() * board_size.0 as f64 - 0.5, -0.5),
                (d.sin() * v * 0.2, -v),
                (js_sys::Math::random() * 40.0) as u64,
                ParticleSort::Diagonals,
            ));
        }
    }

    fn slide_button(back: bool, last: bool) -> ButtonElement {
        ButtonElement::new(
            (if back { 14 } else { 146 }, 220),
            (96, 24),
            if back { 0 } else { 1 },
            if back {
                LabelTrim::Return
            } else {
                LabelTrim::Round
            },
            if back {
                LabelTheme::Default
            } else {
                LabelTheme::Action
            },
            Text(
                if back {
                    "Back"
                } else if last {
                    "Continue"
                } else {
                    "Next"
                }
                .into(),
                Center,
            ),
        )
    }
}

impl State for Tutorial {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app: &AppContext,
    ) -> Result<(), JsValue> {
        if let Some(slide) = self.item_slide {
            let (powerup, title, lines) = ITEMS[slide];
            draw_label(
                context,
                atlas,
                (24, 16),
                (208, 28),
                "#557F55",
                &Text(title.into(), Center),
                &app.pointer,
                app.frame,
                &LabelTrim::Glorious,
                false,
            )?;
            context.save();
            context.translate(128.0, 80.0)?;
            draw_powerup(context, atlas, &Position(0, 0), &powerup, app.frame)?;
            context.restore();
            for (i, line) in lines.iter().enumerate() {
                draw_text_centered(
                    interface_context,
                    atlas,
                    128.0,
                    112.0 + i as f64 * 16.0,
                    line,
                )?;
            }
            draw_text_centered(
                interface_context,
                atlas,
                128.0,
                196.0,
                &format!("{} / 3", slide + 1),
            )?;
            if slide > 0 {
                Self::slide_button(true, false).draw(
                    interface_context,
                    atlas,
                    &app.pointer,
                    app.frame,
                )?;
            }
            Self::slide_button(false, slide == 2).draw(
                interface_context,
                atlas,
                &app.pointer,
                app.frame,
            )?;
            return Ok(());
        }
        self.game_state
            .draw(context, interface_context, atlas, app)?;
        let touch = Game::touch_enabled(app);
        let (title, lines): (&str, Vec<&str>) = match self.tutorial_stage {
            TutorialStage::Movement => (
                "Movement",
                if touch {
                    vec![
                        "Tap the Red Mage.",
                        "Tap an adjacent square",
                        "twice to move.",
                    ]
                } else {
                    vec!["Click the Red Mage.", "Click an adjacent square to move."]
                },
            ),
            TutorialStage::Attacking => (
                "Attacking",
                vec![
                    "Mages attack when they move.",
                    "Each mage has its own pattern.",
                    "Back arrow: twice to undo.",
                ],
            ),
            TutorialStage::FinalBlow => ("Final Blow", vec!["Deal the final blow!"]),
            TutorialStage::Victory => (
                "Victory!",
                vec!["Congratulations!", "Continue to learn about items."],
            ),
            TutorialStage::Retry => ("Try Again", vec!["Choose Rematch to try again."]),
        };
        let (width, height) = self.game_state.visual_game().board_size();
        let (board_x, board_y) = self.game_state.board_offset();
        let center_x = board_x + width as i32 * BOARD_SCALE.0 / 2;
        let title_height = 24;
        let margin = 16;
        draw_label(
            context,
            atlas,
            (center_x - 48, board_y - margin - title_height),
            (96, title_height),
            "#557F55",
            &Text(title.into(), Center),
            &app.pointer,
            app.frame,
            &LabelTrim::Glorious,
            false,
        )?;
        // Anchor the first glyph row to the board, identically for mouse and touch.
        let text_top = (board_y + height as i32 * BOARD_SCALE.1 + margin) as f64;
        let text_width = lines
            .iter()
            .map(|line| text_length(line))
            .max()
            .unwrap_or(0) as f64
            + 8.0;
        interface_context.set_fill_style(&"#002a2a".into());
        interface_context.fill_rect(
            center_x as f64 - text_width / 2.0,
            text_top,
            text_width,
            lines.len() as f64 * 14.0,
        );
        for (i, line) in lines.iter().enumerate() {
            draw_text_centered(
                interface_context,
                atlas,
                center_x as f64,
                text_top + 4.0 + i as f64 * 14.0,
                line,
            )?;
        }
        Ok(())
    }

    fn tick(&mut self, text_input: &HtmlInputElement, app: &AppContext) -> Option<StateSort> {
        let mut pointer = app.pointer.clone();
        if pointer.is_touch()
            && pointer
                .gestures
                .iter()
                .any(|event| matches!(event, crate::app::pointer::GestureEvent::Release(_)))
        {
            pointer.pending_click = true;
        }
        if let Some(slide) = self.item_slide {
            if slide > 0 && Self::slide_button(true, false).tick(&pointer).is_some() {
                self.item_slide = Some(slide - 1);
            } else if Self::slide_button(false, slide == 2)
                .tick(&pointer)
                .is_some()
            {
                if slide == 2 {
                    return Some(if self.campaign {
                        StateSort::ArenaMenu(ArenaMenu::at_position(
                            TUTORIAL_POSITION,
                            self.game_state.newly_won(),
                        ))
                    } else {
                        StateSort::MainMenu(MainMenu::default())
                    });
                }
                self.item_slide = Some(slide + 1);
            }
            return None;
        }
        let game = self.game_state.visual_game();
        let won = game.result() == Some(GameResult::Win(Team::Red));
        if !self.game_state.is_animating() {
            let stage = TutorialStage::for_game(game);
            if self.tutorial_stage != stage {
                self.tutorial_stage = stage;
                self.spark_board();
            }
        }
        match self.game_state.tick(text_input, app) {
            Some(StateSort::Game(_)) => Some(StateSort::Tutorial(Tutorial::new(self.campaign))),
            Some(StateSort::ArenaMenu(_)) | Some(StateSort::SkirmishMenu(_)) if won => {
                self.item_slide = Some(0);
                None
            }
            Some(StateSort::SkirmishMenu(_)) => Some(StateSort::MainMenu(MainMenu::default())),
            next => next,
        }
    }
}

impl Default for Tutorial {
    fn default() -> Self {
        Self::new(false)
    }
}
impl Tutorial {
    pub fn campaign() -> Self {
        Self::new(true)
    }
    fn new(campaign: bool) -> Self {
        let level: Level = TUTORIAL_CODE.into();
        Self {
            game_state: Game::new(LobbySettings {
                player_team: Team::Red,
                lobby_sort: LobbySort::LocalAI,
                loadout_method: if campaign {
                    LoadoutMethod::Arena(level, TUTORIAL_POSITION)
                } else {
                    LoadoutMethod::Prefab(level)
                },
                seed: window().performance().unwrap().now() as u64,
                can_stalemate: false,
            })
            .with_tutorial(),
            tutorial_stage: TutorialStage::Movement,
            campaign,
            item_slide: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use shared::{Board, Mage, MageSort};

    use super::*;

    #[test]
    fn final_blow_requires_a_legal_move_that_finishes_every_enemy() {
        let mut level = Level::default();
        level.board = Board::new(6, 6).unwrap();
        level.mages = vec![
            Mage::new(0, Team::Red, MageSort::Plus, Position(1, 2)),
            Mage::new(1, Team::Blue, MageSort::Plus, Position(4, 2)),
        ];
        assert!(!can_finish(&shared::Game::new(&level, false).unwrap()));
        level.mages[1].mana.0 = 1;
        assert!(can_finish(&shared::Game::new(&level, false).unwrap()));
        level.mages[1].position = Position(5, 5);
        assert!(!can_finish(&shared::Game::new(&level, false).unwrap()));
        level.mages[1].position = Position(4, 2);
        level
            .mages
            .push(Mage::new(2, Team::Blue, MageSort::Plus, Position(5, 5)));
        level.mages[2].mana.0 = 1;
        assert!(!can_finish(&shared::Game::new(&level, false).unwrap()));
        level.mages.pop();
        level.starting_team = Team::Blue;
        assert!(!can_finish(&shared::Game::new(&level, false).unwrap()));
    }

    #[test]
    fn tutorial_stays_in_attacking_after_nonlethal_opening_moves() {
        let mut game = shared::Game::new(&TUTORIAL_CODE.into(), false).unwrap();
        assert_eq!(TutorialStage::for_game(&game), TutorialStage::Movement);
        for (from, to) in [
            (Position(1, 1), Position(2, 1)),
            (Position(4, 1), Position(4, 2)),
            (Position(2, 1), Position(2, 2)),
            (Position(4, 2), Position(4, 1)),
        ] {
            assert!(game.take_move(from, to).is_some());
            assert_eq!(TutorialStage::for_game(&game), TutorialStage::Attacking);
        }
    }
}
