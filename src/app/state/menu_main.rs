use shared::{LoadoutMethod, Lobby, LobbySettings, LobbySort, Team};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::{LobbyState, MenuTeleport, State};
use crate::{
    app::{
        Alignment, AppContext, ButtonClass, ButtonElement, ButtonGroupElement, ButtonTrim,
        Interface, StateSort, UIElement, UIEvent,
    },
    draw::{draw_mage, draw_sprite},
    window,
};

pub struct MenuState {
    interface: Interface,
    sentinel_lobby: Lobby,
    lobby_settings: LobbySettings,
}

const BUTTON_LOCAL: usize = 1;
const BUTTON_VS_AI: usize = 2;
const BUTTON_ONLINE: usize = 3;
const BUTTON_DEFAULT: usize = 10;
const BUTTON_RANDOM: usize = 11;
const BUTTON_SYMMETRIC_RANDOM: usize = 12;
// const BUTTON_ROUND_ROBIN: usize = 13;
const BUTTON_BATTLE: usize = 20;
const BUTTON_TELEPORT: usize = 21;

impl MenuState {
    pub fn new() -> MenuState {
        let button_local = ButtonElement::new(
            (0, 0),
            (72, 32),
            BUTTON_LOCAL,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Text("Local".to_string(), Alignment::Center),
        );
        let button_vs_ai = ButtonElement::new(
            (80, 0),
            (72, 32),
            BUTTON_VS_AI,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Text("AI".to_string(), Alignment::Center),
        );
        let button_online = ButtonElement::new(
            (160, 0),
            (72, 32),
            BUTTON_ONLINE,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Text("Online".to_string(), Alignment::Center),
        );

        let group_lobby_type = ButtonGroupElement::new(
            (12, 64),
            vec![button_local, button_vs_ai, button_online],
            BUTTON_LOCAL,
        );

        let button_default = ButtonElement::new(
            (0, 0),
            (80, 18),
            BUTTON_DEFAULT,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Text("Default".to_string(), Alignment::Center),
        );
        let button_random = ButtonElement::new(
            (0, 22 * 1),
            (80, 18),
            BUTTON_SYMMETRIC_RANDOM,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Text("Random".to_string(), Alignment::Center),
        );

        let button_symmetric_random = ButtonElement::new(
            (0, 22 * 2),
            (80, 18),
            BUTTON_RANDOM,
            ButtonTrim::Round,
            ButtonClass::Default,
            crate::app::ContentElement::Text("Chaos".to_string(), Alignment::Center),
        );

        // let button_round_robin = ButtonElement::new(
        //     (0, 22 * 3),
        //     (80, 18),
        //     BUTTON_ROUND_ROBIN,
        //     ButtonTrim::Round,
        //     ButtonClass::Default,
        //     crate::app::ContentElement::Text("Draft".to_string(), Alignment::Center),
        // );

        let group_loadout_type = ButtonGroupElement::new(
            (16, 112),
            vec![
                button_default,
                button_random,
                button_symmetric_random,
                // button_round_robin,
            ],
            BUTTON_DEFAULT,
        );

        let button_battle = ButtonElement::new(
            (16, 188),
            (80, 24),
            BUTTON_BATTLE,
            ButtonTrim::Glorious,
            ButtonClass::Action,
            crate::app::ContentElement::Text("Battle".to_string(), Alignment::Center),
        );

        let button_teleport = ButtonElement::new(
            (156, 192),
            (88, 16),
            BUTTON_TELEPORT,
            ButtonTrim::Glorious,
            ButtonClass::Default,
            crate::app::ContentElement::Text("Teleport".to_string(), Alignment::Center),
        );

        let root_element = Interface::new(vec![
            Box::new(group_lobby_type),
            Box::new(group_loadout_type),
            Box::new(button_battle),
            Box::new(button_teleport),
        ]);

        MenuState {
            interface: root_element,
            sentinel_lobby: Lobby::new(LobbySettings::default()),
            lobby_settings: LobbySettings::default(),
        }
    }

    fn refresh_lobby(&mut self) {
        self.lobby_settings.seed = window().performance().unwrap().now() as u32;
        self.sentinel_lobby = Lobby::new(self.lobby_settings.clone());
    }
}

impl State for MenuState {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        let frame = app_context.frame;
        let pointer = &app_context.pointer;

        context.save();
        context.translate(16.0, 32.0)?;
        draw_sprite(context, atlas, 256.0, 0.0, 32.0, 32.0, 0.0, 0.0)?;
        draw_sprite(context, atlas, 480.0, 0.0, 32.0, 32.0, 32.0, 0.0)?;

        draw_sprite(context, atlas, 96.0, 64.0, 32.0, 32.0, 16.0, 4.0)?;
        context.translate(80.0, 0.0)?;
        draw_sprite(context, atlas, 256.0, 0.0, 32.0, 32.0, 0.0, 0.0)?;
        draw_sprite(context, atlas, 480.0, 0.0, 32.0, 32.0, 32.0, 0.0)?;

        draw_sprite(context, atlas, 64.0, 64.0, 32.0, 32.0, 26.0, 4.0)?;
        draw_sprite(context, atlas, 128.0, 64.0, 32.0, 32.0, 6.0, 6.0)?;
        context.translate(80.0, 0.0)?;
        draw_sprite(context, atlas, 256.0, 0.0, 32.0, 32.0, 0.0, 0.0)?;
        draw_sprite(context, atlas, 480.0, 0.0, 32.0, 32.0, 32.0, 0.0)?;

        draw_sprite(context, atlas, 32.0, 64.0, 32.0, 32.0, 6.0, 4.0)?;
        draw_sprite(context, atlas, 96.0, 96.0, 32.0, 32.0, 26.0, 6.0)?;
        context.restore();

        self.interface.draw(context, atlas, pointer, frame)?;

        context.save();
        context.translate(108.0, 112.0)?;

        draw_sprite(context, atlas, 256.0, 0.0, 64.0, 32.0, 0.0, 0.0)?;
        draw_sprite(context, atlas, 448.0, 0.0, 64.0, 32.0, 64.0, 0.0)?;
        draw_sprite(context, atlas, 256.0, 224.0, 64.0, 32.0, 0.0, 32.0)?;
        draw_sprite(context, atlas, 448.0, 224.0, 64.0, 32.0, 64.0, 32.0)?;

        for mage in self.sentinel_lobby.game.iter_mages() {
            context.save();
            context.translate(
                -49.0 + mage.position.0 as f64 * 32.0,
                15.0 + if mage.team == Team::Red { 0.0 } else { 1.0 } as f64 * 32.0,
            )?;
            draw_mage(context, atlas, mage, frame, mage.team, true, false)?;
            context.restore();
        }
        context.restore();

        Ok(())
    }

    fn tick(&mut self, app_context: &AppContext) -> Option<StateSort> {
        let pointer = &app_context.pointer;

        if let Some(UIEvent::ButtonClick(value)) = self.interface.tick(pointer) {
            match value {
                BUTTON_LOCAL => {
                    self.lobby_settings.lobby_sort = LobbySort::Local;
                }
                BUTTON_VS_AI => {
                    self.lobby_settings.lobby_sort = LobbySort::LocalAI;
                }
                BUTTON_ONLINE => {
                    self.lobby_settings.lobby_sort = LobbySort::Online(0);
                }
                BUTTON_DEFAULT => {
                    self.lobby_settings.loadout_method = LoadoutMethod::Default;
                    self.refresh_lobby();
                }
                BUTTON_RANDOM => {
                    self.lobby_settings.loadout_method = LoadoutMethod::Random { symmetric: false };
                    self.refresh_lobby();
                }
                BUTTON_SYMMETRIC_RANDOM => {
                    self.lobby_settings.loadout_method = LoadoutMethod::Random { symmetric: true };
                    self.refresh_lobby();
                }
                BUTTON_BATTLE => {
                    return Some(StateSort::Lobby(LobbyState::new(
                        self.lobby_settings.clone(),
                    )));
                }
                BUTTON_TELEPORT => {
                    return Some(StateSort::MenuTeleport(MenuTeleport::new()));
                }
                _ => (),
            }
        }

        None
    }
}
