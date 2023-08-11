use shared::{GameResult, Level, LoadoutMethod, LobbySettings, LobbySort, Team};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{BaseState, LobbyState, State};
use crate::{
    app::{
        Alignment::Center, AppContext, ContentElement::Text, LabelTrim, Particle, ParticleSort,
        StateSort,
    },
    draw::{draw_label, draw_text_centered},
    window,
};

#[derive(PartialEq)]
enum TutorialStage {
    Movement,
    Attacking,
    Charging,
    FinalBlow,
    Victory,
}

pub struct TutorialState {
    pub game_state: LobbyState,
    tutorial_stage: TutorialStage,
}

impl TutorialState {
    pub fn spark_board(&mut self) {
        let board_size = self.game_state.lobby().game.board_size();

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
}

impl State for TutorialState {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        match self.tutorial_stage {
            TutorialStage::Movement => {
                draw_label(
                    context,
                    atlas,
                    (80, 24),
                    (96, 16),
                    &"#557F55",
                    &Text("Movement".to_string(), Center),
                    &app_context.pointer,
                    app_context.frame,
                    &LabelTrim::Glorious,
                )?;

                draw_text_centered(
                    interface_context,
                    atlas,
                    128.0,
                    224.0,
                    &"Click the Red Mage".to_string(),
                )?;
                draw_text_centered(
                    interface_context,
                    atlas,
                    128.0,
                    240.0,
                    &"Then pick a square to move to".to_string(),
                )?;
            }
            TutorialStage::Attacking => {
                draw_label(
                    context,
                    atlas,
                    (80, 24),
                    (96, 16),
                    &"#557F55",
                    &Text("Attacking".to_string(), Center),
                    &app_context.pointer,
                    app_context.frame,
                    &LabelTrim::Glorious,
                )?;

                draw_text_centered(
                    interface_context,
                    atlas,
                    128.0,
                    224.0,
                    &"You attack when you move".to_string(),
                )?;
                draw_text_centered(
                    interface_context,
                    atlas,
                    128.0,
                    240.0,
                    &"Zap the Blue Mage!".to_string(),
                )?;
            }
            TutorialStage::Charging => {
                draw_label(
                    context,
                    atlas,
                    (80, 24),
                    (96, 16),
                    &"#557F55",
                    &Text("Charging".to_string(), Center),
                    &app_context.pointer,
                    app_context.frame,
                    &LabelTrim::Glorious,
                )?;

                draw_text_centered(
                    interface_context,
                    atlas,
                    128.0,
                    224.0,
                    &"Mages Charge when low on mana".to_string(),
                )?;
                draw_text_centered(
                    interface_context,
                    atlas,
                    128.0,
                    240.0,
                    &"Charged Mages move to diagonals".to_string(),
                )?;
            }
            TutorialStage::FinalBlow => {
                draw_label(
                    context,
                    atlas,
                    (80, 24),
                    (96, 16),
                    &"#557F55",
                    &Text("Final Blow".to_string(), Center),
                    &app_context.pointer,
                    app_context.frame,
                    &LabelTrim::Glorious,
                )?;
                draw_text_centered(
                    interface_context,
                    atlas,
                    128.0,
                    232.0,
                    &"Deal the final blow!".to_string(),
                )?;
            }
            TutorialStage::Victory => {
                draw_label(
                    context,
                    atlas,
                    (80, 24),
                    (96, 16),
                    &"#557F55",
                    &Text("Victory!".to_string(), Center),
                    &app_context.pointer,
                    app_context.frame,
                    &LabelTrim::Glorious,
                )?;
                draw_text_centered(
                    interface_context,
                    atlas,
                    128.0,
                    224.0,
                    &"Congratulations!".to_string(),
                )?;
                draw_text_centered(
                    interface_context,
                    atlas,
                    128.0,
                    240.0,
                    &"You won your first battle".to_string(),
                )?;
            }
        }

        self.game_state
            .draw(context, interface_context, atlas, app_context)
    }

    fn tick(
        &mut self,
        text_input: &HtmlInputElement,
        app_context: &AppContext,
    ) -> Option<StateSort> {
        match self.tutorial_stage {
            TutorialStage::Movement => {
                if self.game_state.lobby().game.turns() > 0 {
                    self.tutorial_stage = TutorialStage::Attacking;

                    self.spark_board();
                }
            }
            TutorialStage::Attacking => {
                if self
                    .game_state
                    .lobby()
                    .game
                    .iter_mages()
                    .any(|mage| mage.has_diagonals())
                {
                    self.tutorial_stage = TutorialStage::Charging;

                    self.spark_board();
                }
            }
            TutorialStage::Charging => {
                if self
                    .game_state
                    .lobby()
                    .game
                    .iter_mages()
                    .all(|mage| mage.has_diagonals())
                {
                    self.tutorial_stage = TutorialStage::FinalBlow;

                    self.spark_board();
                }
            }
            _ => {}
        }

        if self.tutorial_stage != TutorialStage::Victory
            && self.game_state.lobby().game.result() == Some(GameResult::Win(Team::Red))
        {
            self.tutorial_stage = TutorialStage::Victory;

            self.spark_board();
        }

        let next_state = self.game_state.tick(text_input, app_context);

        match next_state {
            Some(StateSort::Lobby(_)) => Some(StateSort::Tutorial(TutorialState::default())),
            Some(StateSort::MenuMain(_)) => Some(StateSort::Base(BaseState::default())),
            _ => next_state,
        }
    }
}

impl Default for TutorialState {
    fn default() -> Self {
        let level: Level = "p24g091804j0".into();

        TutorialState {
            game_state: LobbyState::new(LobbySettings {
                lobby_sort: LobbySort::LocalAI,
                loadout_method: LoadoutMethod::Prefab(level.mages),
                seed: window().performance().unwrap().now() as u64,
                board: level.board,
                can_stalemate: false,
            }),
            tutorial_stage: TutorialStage::Movement,
        }
    }
}
