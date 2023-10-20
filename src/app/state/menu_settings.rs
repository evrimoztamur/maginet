use shared::{Board, LoadoutMethod, LobbySettings, LobbySort};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{ArenaMenu, Editor, Game, MainMenu, SkirmishMenu, State, Tutorial};
use crate::{
    app::{
        Alignment, App, AppContext, ButtonElement, ConfirmButtonElement, ContentElement, Interface,
        LabelTheme, LabelTrim, Pointer, StateSort, UIElement, UIEvent,
    },
    draw::{draw_label, draw_sprite, draw_text},
    window,
};

pub struct SettingsMenu {
    interface: Interface,
    pub music_volume: i8,
    pub clip_volume: i8,
}

const BUTTON_BACK: usize = 0;
const BUTTON_MUSIC_MINUS: usize = 10;
const BUTTON_MUSIC_PLUS: usize = 11;
const BUTTON_SOUND_MINUS: usize = 12;
const BUTTON_SOUND_PLUS: usize = 13;

impl SettingsMenu {
    fn save_volume(&self) {
        App::kv_set("music_volume", self.music_volume.to_string().as_str());
        App::kv_set("clip_volume", self.clip_volume.to_string().as_str());
    }

    pub fn load_volume() -> (i8, i8) {
        let music_volume = App::kv_get("music_volume").parse::<i8>().unwrap_or(10);
        let clip_volume = App::kv_get("clip_volume").parse::<i8>().unwrap_or(8);

        (music_volume, clip_volume)
    }
}

impl State for SettingsMenu {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue> {
        let frame = app_context.frame;
        let pointer = &app_context.pointer;

        draw_label(
            context,
            atlas,
            (0, 24),
            (136, 24),
            "#7f3faa",
            &ContentElement::Text("Settings".to_string(), Alignment::Center),
            &app_context.pointer,
            app_context.frame,
            &LabelTrim::Glorious,
            false,
        )?;

        draw_text(context, atlas, 0.0, 60.0, "Music Volume")?;

        for i in (0..10).rev() {
            if self.music_volume > i {
                draw_sprite(
                    context,
                    atlas,
                    152.0,
                    0.0,
                    12.0,
                    12.0,
                    32.0 + i as f64 * 10.0,
                    76.0,
                )?;
            } else {
                draw_sprite(
                    context,
                    atlas,
                    164.0,
                    0.0,
                    12.0,
                    12.0,
                    32.0 + i as f64 * 10.0,
                    76.0,
                )?;
            }
        }

        draw_text(context, atlas, 0.0, 100.0, "Sound Volume")?;

        for i in (0..10).rev() {
            if self.clip_volume > i {
                draw_sprite(
                    context,
                    atlas,
                    152.0,
                    0.0,
                    12.0,
                    12.0,
                    32.0 + i as f64 * 10.0,
                    116.0,
                )?;
            } else {
                draw_sprite(
                    context,
                    atlas,
                    164.0,
                    0.0,
                    12.0,
                    12.0,
                    32.0 + i as f64 * 10.0,
                    116.0,
                )?;
            }
        }

        context.save();

        context.translate(180.0, 28.0)?;

        draw_label(
            context,
            atlas,
            (0, 0),
            (96, 16),
            "#7f0055",
            &ContentElement::Text("Credits".to_string(), Alignment::Center),
            &app_context.pointer,
            app_context.frame,
            &LabelTrim::Glorious,
            false,
        )?;

        draw_text(context, atlas, 0.0, 24.0, "Code")?;
        draw_text(context, atlas, 8.0, 24.0 + 12.0, "@evrimzone")?;
        draw_text(context, atlas, 0.0, 24.0 + 32.0, "Graphics")?;
        draw_text(context, atlas, 8.0, 24.0 + 32.0 + 12.0, "@mrmotarius")?;
        draw_text(context, atlas, 0.0, 24.0 + 64.0, "Sounds")?;
        draw_text(context, atlas, 8.0, 24.0 + 64.0 + 12.0, "@effoharkay")?;
        draw_text(context, atlas, 0.0, 24.0 + 96.0, "Music")?;
        draw_text(context, atlas, 8.0, 24.0 + 96.0 + 12.0, "Alex Neri")?;

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
        let frame = app_context.frame;
        let pointer = &app_context.pointer;

        if let Some(UIEvent::ButtonClick(value, clip_id)) = self.interface.tick(pointer) {
            app_context.audio_system.play_clip_option(clip_id);

            match value {
                BUTTON_BACK => {
                    return Some(StateSort::MainMenu(MainMenu::default()));
                }
                BUTTON_MUSIC_MINUS => {
                    self.music_volume = (self.music_volume - 1).max(0).min(10);
                    self.save_volume();
                }
                BUTTON_MUSIC_PLUS => {
                    self.music_volume = (self.music_volume + 1).max(0).min(10);
                    self.save_volume();
                }
                BUTTON_SOUND_MINUS => {
                    self.clip_volume = (self.clip_volume - 1).max(0).min(10);
                    self.save_volume();
                }
                BUTTON_SOUND_PLUS => {
                    self.clip_volume = (self.clip_volume + 1).max(0).min(10);
                    self.save_volume();
                }
                _ => (),
            }
        }

        None
    }
}

impl Default for SettingsMenu {
    fn default() -> Self {
        let button_back = ButtonElement::new(
            (84, 224),
            (88, 16),
            BUTTON_BACK,
            LabelTrim::Return,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Back".to_string(), Alignment::Center),
        );

        let button_music_minus = ButtonElement::new(
            (0, 76),
            (12, 12),
            BUTTON_MUSIC_MINUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((88, 24), (8, 8)),
        );

        let button_music_plus = ButtonElement::new(
            (16, 76),
            (12, 12),
            BUTTON_MUSIC_PLUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((80, 24), (8, 8)),
        );

        let button_sound_minus = ButtonElement::new(
            (0, 116),
            (12, 12),
            BUTTON_SOUND_MINUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((88, 24), (8, 8)),
        );

        let button_sound_plus = ButtonElement::new(
            (16, 116),
            (12, 12),
            BUTTON_SOUND_PLUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((80, 24), (8, 8)),
        );

        let interface = Interface::new(vec![
            button_back.boxed(),
            button_music_minus.boxed(),
            button_music_plus.boxed(),
            button_sound_minus.boxed(),
            button_sound_plus.boxed(),
        ]);

        let (music_volume, clip_volume) = SettingsMenu::load_volume();

        SettingsMenu {
            interface,
            music_volume,
            clip_volume,
        }
    }
}
