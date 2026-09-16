pub(super) use shared::TUTORIAL_CODE;
use shared::{GameResult, Level, LoadoutMethod, LobbySettings, LobbySort, Team};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{learning::BoardHint, menu_arena::TUTORIAL_POSITION, Game, MainMenu, State};
use crate::{
    app::{AppContext, Particle, ParticleSort, StateSort},
    window,
};

#[derive(Debug, PartialEq)]
enum TutorialStage {
    Movement,
    Attacking,
    Undo,
    FinalBlow,
    Victory,
    Retry,
}

impl TutorialStage {
    fn for_game(game: &shared::Game, undo: bool) -> Self {
        match game.result() {
            Some(GameResult::Win(Team::Red)) => Self::Victory,
            Some(_) => Self::Retry,
            None if game.turns() == 0 => Self::Movement,
            None if can_finish(game) => Self::FinalBlow,
            None if undo => Self::Undo,
            None => Self::Attacking,
        }
    }
}

#[derive(Default)]
struct TutorialHints {
    low_mana_seen: bool,
    undo_turn: Option<usize>,
}

impl TutorialHints {
    fn stage(&mut self, game: &shared::Game) -> TutorialStage {
        // Moving on or undoing dismisses the hint; later hits must not repeat it.
        if self.undo_turn != Some(game.turns()) {
            self.undo_turn = None;
        }
        if !self.low_mana_seen
            && game
                .iter_mages()
                .any(|mage| mage.team == Team::Red && mage.mana == 1)
        {
            self.low_mana_seen = true;
            self.undo_turn = Some(game.turns());
        }
        TutorialStage::for_game(game, self.undo_turn.is_some())
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
    hints: TutorialHints,
    campaign: bool,
}

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
}

impl State for Tutorial {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app: &AppContext,
    ) -> Result<(), JsValue> {
        self.game_state
            .draw(context, interface_context, atlas, app)?;
        if self.tutorial_stage == TutorialStage::Undo {
            self.game_state
                .draw_undo_hint(interface_context, atlas, app)?;
        }
        let touch = Game::touch_enabled(app);
        let (title, lines): (&str, &[&str]) = match self.tutorial_stage {
            TutorialStage::Movement => (
                "Movement",
                if touch {
                    &[
                        "Tap the Red Mage.",
                        "Tap an adjacent square",
                        "twice to move.",
                    ]
                } else {
                    &["Click the Red Mage.", "Click an adjacent square to move."]
                },
            ),
            TutorialStage::Attacking => (
                "Attacking",
                &[
                    "Mages attack when they move.",
                    "Each mage has its own pattern.",
                ],
            ),
            TutorialStage::Undo => (
                "Undo",
                &["Only 1 mana left!", "You can always undo your moves."],
            ),
            TutorialStage::FinalBlow => ("Final Blow", &["Deal the final blow!"]),
            TutorialStage::Victory => ("Victory!", &["Congratulations!"]),
            TutorialStage::Retry => ("Try Again", &["Choose Rematch to try again."]),
        };
        BoardHint { title, lines }.draw(
            context,
            interface_context,
            atlas,
            app,
            self.game_state.board_offset(),
            self.game_state.visual_game().board_size(),
        )
    }

    fn tick(&mut self, text_input: &HtmlInputElement, app: &AppContext) -> Option<StateSort> {
        let game = self.game_state.visual_game();
        if !self.game_state.is_animating() {
            let stage = self.hints.stage(game);
            if self.tutorial_stage != stage {
                self.tutorial_stage = stage;
                self.spark_board();
            }
        }
        match self.game_state.tick(text_input, app) {
            Some(StateSort::Game(_)) => Some(StateSort::Tutorial(Tutorial::new(self.campaign))),
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
            hints: TutorialHints::default(),
            campaign,
        }
    }
}

#[cfg(test)]
mod tests {
    use shared::{Board, Mage, MageSort, Position};

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
        assert_eq!(
            TutorialStage::for_game(&game, false),
            TutorialStage::Movement
        );
        for (from, to) in [
            (Position(1, 1), Position(2, 1)),
            (Position(4, 1), Position(4, 2)),
            (Position(2, 1), Position(2, 2)),
            (Position(4, 2), Position(4, 1)),
        ] {
            assert!(game.take_move(from, to).is_some());
            assert_eq!(
                TutorialStage::for_game(&game, false),
                TutorialStage::Attacking
            );
        }
    }

    fn opening_hit(game: &mut shared::Game, hints: &mut TutorialHints) -> TutorialStage {
        assert!(game.take_move(Position(1, 1), Position(2, 1)).is_some());
        assert_eq!(hints.stage(game), TutorialStage::Attacking);
        assert!(game.take_move(Position(4, 1), Position(3, 1)).is_some());
        hints.stage(game)
    }

    #[test]
    fn undo_hint_waits_for_one_mana_and_clears_after_moving_on() {
        let mut game = shared::Game::new(&TUTORIAL_CODE.into(), false).unwrap();
        let mut hints = TutorialHints::default();
        assert_eq!(hints.stage(&game), TutorialStage::Movement);
        assert_eq!(opening_hit(&mut game, &mut hints), TutorialStage::Attacking);
        let red = game.iter_mages().find(|m| m.team == Team::Red).unwrap();
        assert_eq!(
            red.mana.0, 3,
            "Plus opponent can hit Red on its first reply"
        );
        assert!(game.take_move(Position(2, 1), Position(2, 2)).is_some());
        assert_eq!(hints.stage(&game), TutorialStage::Attacking);
        assert!(game.take_move(Position(3, 1), Position(3, 2)).is_some());
        assert_eq!(
            hints.stage(&game),
            TutorialStage::Attacking,
            "two mana is not yet the undo lesson"
        );
        assert!(game.take_move(Position(2, 2), Position(1, 2)).is_some());
        assert_eq!(hints.stage(&game), TutorialStage::Attacking);
        assert!(game.take_move(Position(3, 2), Position(2, 2)).is_some());
        assert_eq!(hints.stage(&game), TutorialStage::Undo);
        assert_eq!(
            hints.stage(&game),
            TutorialStage::Undo,
            "hint stays readable"
        );
        assert!(
            game.take_move(Position(1, 2), Position(0, 2)).is_none(),
            "blocked moves keep the hint"
        );
        assert_eq!(hints.stage(&game), TutorialStage::Undo);
        assert!(game.take_move(Position(1, 2), Position(1, 3)).is_some());
        assert_eq!(hints.stage(&game), TutorialStage::Attacking);
    }

    #[test]
    fn undo_dismisses_the_hint_without_rearming_it_but_rematch_resets_it() {
        let mut level: Level = TUTORIAL_CODE.into();
        level
            .mages
            .iter_mut()
            .find(|m| m.team == Team::Red)
            .unwrap()
            .mana
            .0 = 2;
        let mut game = shared::Game::new(&level, false).unwrap();
        let mut hints = TutorialHints::default();
        assert_eq!(opening_hit(&mut game, &mut hints), TutorialStage::Undo);
        game = game.rewind(2);
        assert_eq!(hints.stage(&game), TutorialStage::Movement);
        assert_eq!(opening_hit(&mut game, &mut hints), TutorialStage::Attacking);
        let mut rematch = game.rewind(game.turns());
        assert_eq!(
            opening_hit(&mut rematch, &mut TutorialHints::default()),
            TutorialStage::Undo
        );
    }

    #[test]
    fn final_blow_and_terminal_results_take_priority_over_low_mana() {
        let mut level: Level = TUTORIAL_CODE.into();
        level
            .mages
            .iter_mut()
            .find(|m| m.team == Team::Red)
            .unwrap()
            .mana
            .0 = 2;
        level
            .mages
            .iter_mut()
            .find(|m| m.team == Team::Blue)
            .unwrap()
            .mana
            .0 = 1;
        let mut game = shared::Game::new(&level, false).unwrap();
        let mut hints = TutorialHints::default();
        assert_eq!(opening_hit(&mut game, &mut hints), TutorialStage::FinalBlow);
        assert!(game.take_move(Position(2, 1), Position(2, 2)).is_some());
        assert_eq!(hints.stage(&game), TutorialStage::Victory);

        level
            .mages
            .iter_mut()
            .find(|m| m.team == Team::Red)
            .unwrap()
            .mana
            .0 = 1;
        let mut game = shared::Game::new(&level, false).unwrap();
        assert!(game.take_move(Position(1, 1), Position(2, 1)).is_some());
        assert!(game.take_move(Position(4, 1), Position(3, 1)).is_some());
        assert_eq!(TutorialHints::default().stage(&game), TutorialStage::Retry);
    }
}
