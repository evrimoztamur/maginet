use shared::{Level, LobbySettings, Mage};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{EditorState, LobbyState, State};
use crate::{
    app::{
        Alignment, App, AppContext, ButtonElement, Interface, LabelTheme, LabelTrim, StateSort,
        UIElement, UIEvent, BOARD_SCALE,
    },
    draw::{draw_board, draw_mage, draw_mana, draw_sprite},
    tuple_as,
};

pub struct PreviewState {
    interface: Interface,
    level: Level,
    board_dirty: bool,
}

const BUTTON_BACK: usize = 0;
const BUTTON_LOCAL: usize = 1;
const BUTTON_VS_AI: usize = 2;
const BUTTON_ONLINE: usize = 3;

impl PreviewState {
    pub fn new(level: Level) -> PreviewState {
        let button_back = ButtonElement::new(
            (-60, 118),
            (20, 20),
            BUTTON_BACK,
            LabelTrim::Round,
            LabelTheme::Bright,
            crate::app::ContentElement::Sprite((160, 32), (16, 16)),
        );

        let button_local = ButtonElement::new(
            (240, 48),
            (72, 32),
            BUTTON_LOCAL,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Local".to_string(), Alignment::Center),
        );
        let button_vs_ai = ButtonElement::new(
            (240, 128),
            (72, 32),
            BUTTON_VS_AI,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Text("AI".to_string(), Alignment::Center),
        );
        let button_online = ButtonElement::new(
            (240, 208),
            (72, 32),
            BUTTON_ONLINE,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Online".to_string(), Alignment::Center),
        );

        let interface = Interface::new(vec![
            button_back.boxed(),
            button_local.boxed(),
            button_vs_ai.boxed(),
            button_online.boxed(),
        ]);

        PreviewState {
            interface,
            level,
            board_dirty: true,
        }
    }

    pub fn board_offset(&self) -> (i32, i32) {
        (
            ((8 - self.level.board.width) as i32 * BOARD_SCALE.0) / 2,
            ((8 - self.level.board.height) as i32 * BOARD_SCALE.1) / 2,
        )
    }
}

impl State for PreviewState {
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
            draw_board(
                atlas,
                256.0,
                0.0,
                self.level.board.width,
                self.level.board.height,
                8,
                8,
                (0, 0),
            )
            .unwrap();
        }

        {
            context.save();

            context.translate(-32.0, 0.0)?;

            {
                context.save();

                draw_sprite(context, atlas, 256.0, 0.0, 256.0, 256.0, 0.0, 0.0)?;

                context.translate(board_offset.0 as f64, board_offset.1 as f64)?;

                let mut mage_heap: Vec<&Mage> = self.level.mages.iter().collect();
                mage_heap.sort_by(|a, b| a.position.1.cmp(&b.position.1));

                // DRAW mages
                for mage in mage_heap {
                    context.save();

                    context.translate(
                        16.0 + mage.position.0 as f64 * board_scale.0,
                        16.0 + mage.position.1 as f64 * board_scale.1,
                    )?;

                    draw_mage(context, atlas, mage, frame, mage.team, true, None)?;
                    draw_mana(context, atlas, mage)?;

                    context.restore();
                }
                context.restore();
            }

            {
                context.translate(276.0, 16.0)?;

                draw_sprite(context, atlas, 256.0, 256.0, 64.0, 32.0, 0.0, 0.0)?;

                draw_sprite(context, atlas, 96.0, 64.0, 32.0, 40.0, 16.0, -2.0)?;
            }

            {
                context.translate(0.0, 80.0)?;

                draw_sprite(context, atlas, 256.0, 256.0, 64.0, 32.0, 0.0, 0.0)?;

                draw_sprite(context, atlas, 64.0, 64.0, 32.0, 40.0, 26.0, -4.0)?;
                draw_sprite(context, atlas, 128.0, 104.0, 32.0, 40.0, 6.0, -2.0)?;
            }

            {
                context.translate(0.0, 80.0)?;

                draw_sprite(context, atlas, 256.0, 256.0, 64.0, 32.0, 0.0, 0.0)?;

                draw_sprite(context, atlas, 32.0, 64.0, 32.0, 40.0, 6.0, -4.0)?;
                draw_sprite(context, atlas, 0.0, 256.0, 32.0, 40.0, 26.0, -2.0)?;
            }

            context.restore();
        }

        self.interface
            .draw(interface_context, atlas, pointer, frame)?;

        Ok(())
    }

    fn tick(
        &mut self,
        _text_input: &HtmlInputElement,
        app_context: &AppContext,
    ) -> Option<StateSort> {
        let _board_offset = self.board_offset();
        let pointer = &app_context.pointer;

        if let Some(UIEvent::ButtonClick(value)) = self.interface.tick(pointer) {
            match value {
                BUTTON_BACK => {
                    return Some(StateSort::Editor(EditorState::new(self.level.clone())));
                }
                BUTTON_LOCAL => {
                    return Some(StateSort::Lobby(LobbyState::new(LobbySettings {
                        lobby_sort: shared::LobbySort::Local,
                        loadout_method: shared::LoadoutMethod::EditorPrefab(self.level.clone()),
                        ..Default::default()
                    })));
                }
                BUTTON_VS_AI => {
                    return Some(StateSort::Lobby(LobbyState::new(LobbySettings {
                        lobby_sort: shared::LobbySort::LocalAI,
                        loadout_method: shared::LoadoutMethod::EditorPrefab(self.level.clone()),
                        ..Default::default()
                    })));
                }
                BUTTON_ONLINE => {
                    return Some(StateSort::Lobby(LobbyState::new(LobbySettings {
                        lobby_sort: shared::LobbySort::Online(0),
                        loadout_method: shared::LoadoutMethod::EditorPrefab(self.level.clone()),
                        ..Default::default()
                    })));
                }
                _ => (),
            }
        }

        None
    }
}

impl Default for PreviewState {
    fn default() -> Self {
        PreviewState::new(App::load_level(0).unwrap_or_default())
    }
}
