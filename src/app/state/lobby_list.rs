use std::{cell::RefCell, collections::HashMap, rc::Rc};

use shared::{Lobby, LobbySettings, LobbySort, Message, Position};
use wasm_bindgen::{closure::Closure, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{Game, SkirmishMenu, State};
use crate::{
    app::{
        Alignment, AppContext, ButtonElement, Interface, LabelTheme, LabelTrim, StateSort,
        UIElement, UIEvent,
    },
    draw::{draw_label, draw_mage, draw_powerup, draw_text, draw_text_centered},
    net::{fetch, request_lobbies, MessagePool},
};

pub struct LobbyList {
    interface: Interface,
    lobby_list_interface: Interface,
    last_lobby_refresh: u64,
    message_pool: Rc<RefCell<MessagePool>>,
    message_closure: Closure<dyn FnMut(JsValue)>,
    lobbies: HashMap<u16, Lobby>,
    displayed_lobbies: Vec<(usize, (u16, Lobby))>,
    lobby_page: usize,
    lobby_list_dirty: bool,
}

impl LobbyList {}

const BUTTON_PAGE_PREVIOUS: usize = 10;
const BUTTON_PAGE_NEXT: usize = 11;
const BUTTON_ARENA: usize = 20;
const BUTTON_BACK: usize = 21;
const BUTTON_SEARCH: usize = 22;

const LOBBY_PAGE_SIZE: usize = 3;

impl State for LobbyList {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        let frame = app_context.frame;
        let pointer = &app_context.pointer;

        context.save();

        context.translate(-64.0, 0.0)?;

        self.interface
            .draw(interface_context, atlas, pointer, frame)?;
        self.lobby_list_interface
            .draw(interface_context, atlas, pointer, frame)?;

        draw_text_centered(
            context,
            atlas,
            (384.0 - 84.0) / 2.0,
            228.0,
            format!("{}", self.lobby_page + 1).as_str(),
        )?;

        // let a: Vec<f64> = self
        //     .displayed_lobbies
        //     .iter()
        //     .map(|(_, (_, lobby))| lobby.first_heartbeat)
        //     .collect();
        // console::log_1(&format!("{:?}", a).into());

        if self.displayed_lobbies.is_empty() {
            draw_label(
                context,
                atlas,
                ((384 - 160) / 2, (256 - 64) / 2),
                (160, 16),
                "#7f00aa",
                &crate::app::ContentElement::Text(
                    "No lobbies available.".to_string(),
                    Alignment::Center,
                ),
                &pointer,
                frame,
                &LabelTrim::Round,
                false,
            )?;

            // Create a new one to start playing!
            draw_label(
                context,
                atlas,
                ((384 - 240) / 2, (256 - 64) / 2 + 24),
                (240, 24),
                "#7f00aa00",
                &crate::app::ContentElement::Text(
                    "Create a new one to start playing!".to_string(),
                    Alignment::Center,
                ),
                &pointer,
                frame,
                &LabelTrim::Round,
                false,
            )?;
        } else {
            for (i, (lobby_id, lobby)) in &self.displayed_lobbies {
                let ir: usize = i - self.lobby_page * LOBBY_PAGE_SIZE;
                let pointer = pointer.teleport((0, -(12 + ir as i32 * 64)));
                context.save();
                context.translate((384.0 - 256.0) / 2.0, 12.0 + ir as f64 * 64.0)?;
                draw_label(
                    context,
                    atlas,
                    (0, 15),
                    (224, 36),
                    "#553f55",
                    &crate::app::ContentElement::None,
                    &pointer,
                    frame,
                    &LabelTrim::Round,
                    false,
                )?;

                context.save();

                context.translate(16.0, 32.0)?;

                for (i, mage) in lobby.game.iter_mages().enumerate() {
                    if i % 2 == 0 {
                        context.translate(0.0, 4.0)?;
                    } else {
                        context.translate(0.0, -4.0)?;
                    }
                    draw_mage(context, atlas, mage, frame, mage.team, true, None)?;
                    context.translate(20.0, 0.0)?;
                }

                for (i, powerup) in lobby.game.powerups().values().enumerate() {
                    if i % 2 == 0 {
                        context.translate(0.0, 4.0)?;
                    } else {
                        context.translate(0.0, -4.0)?;
                    }
                    draw_powerup(context, atlas, &Position::default(), powerup, frame + i as u64 * 7)?;
                    context.translate(20.0, 0.0)?;
                }

                context.restore();

                if pointer.in_region((-8, 0), (72, 16)) {
                    draw_label(
                        context,
                        atlas,
                        (-8, 0),
                        (72, 16),
                        "#2a9f55",
                        &crate::app::ContentElement::Text(format!("{lobby_id}"), Alignment::Center),
                        &pointer,
                        frame,
                        &LabelTrim::Glorious,
                        false,
                    )?;
                } else {
                    draw_label(
                        context,
                        atlas,
                        (-8, 0),
                        (72, 16),
                        "#2a9f55",
                        &crate::app::ContentElement::Text(
                            format!("Lobby {}", i + 1),
                            Alignment::Center,
                        ),
                        &pointer,
                        frame,
                        &LabelTrim::Glorious,
                        false,
                    )?;
                }

                draw_text(
                    context,
                    atlas,
                    72.0,
                    4.0,
                    &format!("{}", lobby.settings.loadout_method),
                )?;

                context.restore();
            }
        }

        context.restore();

        Ok(())
    }

    fn tick(
        &mut self,
        text_input: &HtmlInputElement,
        app_context: &AppContext,
    ) -> Option<StateSort> {
        let frame = app_context.frame;
        let pointer = &app_context.pointer;

        if text_input.dataset().get("field").is_some() {
            return None;
        }

        if let Some((field, value)) = &app_context.text_input {
            if field == "lobby_code" {
                if let Ok(lobby_code_input) = value.parse::<u16>() {
                    if self
                        .lobbies
                        .keys()
                        .find(|lobby_code| **lobby_code == lobby_code_input)
                        .is_some()
                    {
                        return Some(StateSort::Game(Game::new(LobbySettings {
                            lobby_sort: LobbySort::Online(lobby_code_input as u16),
                            ..Default::default()
                        })));
                    }
                }
            }
        }

        if let Some(UIEvent::ButtonClick(value, clip_id)) = self.interface.tick(pointer) {
            app_context.audio_system.play_clip_option(clip_id);

            if let BUTTON_SEARCH = value {
                text_input.set_value("");
                text_input.set_placeholder("Enter lobby code");
                text_input.dataset().set("field", "lobby_code").unwrap();
                text_input.focus().unwrap();
            } else if let BUTTON_PAGE_PREVIOUS = value {
                self.lobby_page = self.lobby_page.saturating_sub(1);
                self.lobby_list_dirty = true;
            } else if let BUTTON_PAGE_NEXT = value {
                self.lobby_page = self.lobby_page.saturating_add(1);
                self.lobby_list_dirty = true;
            } else if let BUTTON_BACK = value {
                return Some(StateSort::SkirmishMenu(SkirmishMenu::default()));
            }
        }

        if let Some(UIEvent::ButtonClick(value, clip_id)) = self.lobby_list_interface.tick(pointer)
        {
            if let Some(session_id) = &app_context.session_id {
                app_context.audio_system.play_clip_option(clip_id);

                // console::log_1(&format!("{}", value).into());
                return Some(StateSort::Game(Game::new(LobbySettings {
                    lobby_sort: LobbySort::Online(value as u16),
                    ..Default::default()
                })));
            }
        }

        self.lobby_page = self
            .lobby_page
            .min(self.lobbies.len().saturating_sub(1) / LOBBY_PAGE_SIZE);

        if (frame - self.last_lobby_refresh) > 60 {
            self.last_lobby_refresh = frame;
            let _ = fetch(&request_lobbies()).then(&self.message_closure);
        }

        let mut message_pool = self.message_pool.borrow_mut();

        for message in &message_pool.messages {
            match message {
                Message::Lobby(_lobby) => {
                    // self.lobbies.insert(0, *lobby.clone());
                }
                Message::Lobbies(lobbies) => {
                    self.lobbies = lobbies.clone();
                    self.lobby_list_dirty = true;
                }
                _ => (),
            }
        }

        message_pool.clear();

        if self.lobby_list_dirty {
            self.lobby_list_dirty = false;

            let mut displayed_lobbies: Vec<(u16, Lobby)> =
                self.lobbies.clone().into_iter().collect();

            displayed_lobbies.sort_by(|(_, a), (_, b)| {
                a.first_heartbeat()
                    .partial_cmp(&b.first_heartbeat())
                    .expect("unexpected timestamp comparison failure")
            });

            self.displayed_lobbies = displayed_lobbies
                .into_iter()
                .enumerate()
                .skip(self.lobby_page * LOBBY_PAGE_SIZE)
                .take(LOBBY_PAGE_SIZE)
                .collect();

            self.lobby_list_interface = Interface::new(
                self.displayed_lobbies
                    .iter()
                    .map(|(i, (key, _lobby))| {
                        // console::log_1(&format!("INTERP {}", key).into());
                        let ir: usize = i - self.lobby_page * LOBBY_PAGE_SIZE;
                        ButtonElement::new(
                            (256 - 44, 44 + ir as i32 * 64),
                            (24, 24),
                            *key as usize,
                            LabelTrim::Return,
                            LabelTheme::Action,
                            crate::app::ContentElement::Sprite((96, 32), (16, 16)),
                        )
                        .boxed()
                    })
                    .collect(),
            );
        }

        None
    }
}

impl Default for LobbyList {
    fn default() -> Self {
        let button_search = ButtonElement::new(
            ((256 - 160) / 2 - 32, 220 - 2),
            (28, 20),
            BUTTON_SEARCH,
            LabelTrim::Return,
            LabelTheme::Action,
            crate::app::ContentElement::Sprite((176, 32), (16, 16)),
        );

        let button_settings: ButtonElement = ButtonElement::new(
            (128, 220),
            (88, 16),
            BUTTON_BACK,
            LabelTrim::Return,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Back".to_string(), Alignment::Center),
        );

        let button_page_previous: ButtonElement = ButtonElement::new(
            ((256 - 160) / 2 + 6, 220),
            (20, 16),
            BUTTON_PAGE_PREVIOUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((128, 48), (8, 8)),
        );

        let button_page_next: ButtonElement = ButtonElement::new(
            ((256 - 160) / 2 + 50, 220),
            (20, 16),
            BUTTON_PAGE_NEXT,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((136, 48), (8, 8)),
        );

        let interface = Interface::new(vec![
            button_search.boxed(),
            button_settings.boxed(),
            button_page_previous.boxed(),
            button_page_next.boxed(),
        ]);

        let message_pool = Rc::new(RefCell::new(MessagePool::new()));

        let message_closure = {
            let message_pool = message_pool.clone();

            Closure::<dyn FnMut(JsValue)>::new(move |value| {
                let mut message_pool = message_pool.borrow_mut();
                let message: Message = serde_wasm_bindgen::from_value(value).unwrap();
                message_pool.push(message);
            })
        };

        let lobbies = HashMap::new();

        LobbyList {
            interface,
            lobby_list_interface: Interface::new(Vec::default()),
            last_lobby_refresh: 0,
            lobby_page: 0,
            lobby_list_dirty: false,
            displayed_lobbies: Vec::new(),
            message_closure,
            message_pool,
            lobbies,
        }
    }
}
