mod reset_star;
use reset_star::{reset_campaign_progress, ResetStar};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, HtmlInputElement};

use super::{MainMenu, State};
use crate::{
    app::{
        Alignment, App, AppContext, ButtonElement, ButtonGroupElement, ContentElement, Interface,
        LabelTheme, LabelTrim, StateSort, UIElement, UIEvent,
    },
    draw::{draw_label, draw_sprite, draw_text},
};

pub struct SettingsMenu {
    interface: Interface,
    pub music_volume: i8,
    pub clip_volume: i8,
    difficulty: shared::Difficulty,
    reset_star: ResetStar,
}

const BUTTON_BACK: usize = 0;
const BUTTON_MUSIC_MINUS: usize = 10;
const BUTTON_MUSIC_PLUS: usize = 11;
const BUTTON_SOUND_MINUS: usize = 12;
const BUTTON_SOUND_PLUS: usize = 13;
const BUTTON_CONTROLS_ON: usize = 20;
const BUTTON_CONTROLS_OFF: usize = 21;
const BUTTON_DIFFICULTY_EASY: usize = 30;
const BUTTON_DIFFICULTY_NORMAL: usize = 31;
const BUTTON_DIFFICULTY_HARD: usize = 32;

impl SettingsMenu {
    pub(crate) fn onscreen_controls_enabled() -> bool {
        App::kv_get("onscreen_controls") != "off"
    }

    fn purchase_button() -> ButtonElement {
        let owned = !crate::access::demo();
        ButtonElement::new(
            (176, 156),
            (144, 22),
            98,
            LabelTrim::Round,
            if owned {
                LabelTheme::Disabled
            } else {
                LabelTheme::Action
            },
            ContentElement::Text(
                if owned {
                    "Full Version!"
                } else {
                    "Unlock Full Game"
                }
                .into(),
                Alignment::Center,
            ),
        )
    }

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
            (-4, 16),
            (136, 24),
            "#7f3faa",
            &ContentElement::Text("Settings".to_string(), Alignment::Center),
            &app_context.pointer,
            app_context.frame,
            &LabelTrim::Glorious,
            false,
        )?;

        draw_text(context, atlas, -16.0, 52.0, "Music Volume")?;

        for i in (0..10).rev() {
            if self.music_volume > i {
                draw_sprite(
                    context,
                    atlas,
                    152.0,
                    0.0,
                    12.0,
                    12.0,
                    12.0 + i as f64 * 10.0,
                    72.0,
                )?;
            } else {
                draw_sprite(
                    context,
                    atlas,
                    164.0,
                    0.0,
                    12.0,
                    12.0,
                    12.0 + i as f64 * 10.0,
                    72.0,
                )?;
            }
        }

        draw_text(context, atlas, -16.0, 98.0, "Sound Volume")?;

        for i in (0..10).rev() {
            if self.clip_volume > i {
                draw_sprite(
                    context,
                    atlas,
                    152.0,
                    0.0,
                    12.0,
                    12.0,
                    12.0 + i as f64 * 10.0,
                    118.0,
                )?;
            } else {
                draw_sprite(
                    context,
                    atlas,
                    164.0,
                    0.0,
                    12.0,
                    12.0,
                    12.0 + i as f64 * 10.0,
                    118.0,
                )?;
            }
        }

        draw_text(context, atlas, -16.0, 144.0, "AI Difficulty")?;

        draw_text(context, atlas, -16.0, 190.0, "On-screen controls")?;

        context.save();

        context.translate(176.0, 16.0)?;

        draw_label(
            context,
            atlas,
            (0, 0),
            (96, 24),
            "#7f0055",
            &ContentElement::Text("Credits".to_string(), Alignment::Center),
            &app_context.pointer,
            app_context.frame,
            &LabelTrim::Glorious,
            false,
        )?;

        for (index, (role, author)) in [
            ("Code", "@evrimzone"),
            ("Graphics", "@mrmotarius"),
            ("Sounds", "@effoharkay"),
            ("Music", "Alex Neri"),
        ]
        .iter()
        .enumerate()
        {
            let y = 36.0 + index as f64 * 24.0;
            draw_text(context, atlas, 0.0, y, role)?;
            draw_text(context, atlas, 8.0, y + 10.0, author)?;
        }

        context.restore();

        self.interface
            .draw(interface_context, atlas, pointer, frame)?;
        if cfg!(feature = "mobile") {
            Self::purchase_button().draw(interface_context, atlas, pointer, frame)?;
        }

        self.reset_star
            .draw(interface_context, atlas, app_context)?;
        Ok(())
    }

    fn tick(
        &mut self,
        _text_input: &HtmlInputElement,
        app_context: &AppContext,
    ) -> Option<StateSort> {
        if self.reset_star.tick(app_context) {
            self.reset_star
                .complete(app_context, reset_campaign_progress().is_ok());
        }
        // let frame = app_context.frame;
        let pointer = &app_context.pointer;

        let event = self.interface.tick(pointer).or_else(|| {
            if cfg!(feature = "mobile") {
                Self::purchase_button().tick(pointer)
            } else {
                None
            }
        });
        if let Some(UIEvent::ButtonClick(value, clip_id)) = event {
            app_context.audio_system.play_clip_option(clip_id);

            match value {
                BUTTON_CONTROLS_ON | BUTTON_CONTROLS_OFF => {
                    App::kv_set(
                        "onscreen_controls",
                        if value == BUTTON_CONTROLS_ON {
                            "on"
                        } else {
                            "off"
                        },
                    );
                }
                98 => crate::access::purchase("purchase"),
                99 => crate::access::purchase("restore"),
                100 => crate::access::purchase("review"),
                BUTTON_DIFFICULTY_EASY | BUTTON_DIFFICULTY_NORMAL | BUTTON_DIFFICULTY_HARD => {
                    self.difficulty = match value {
                        BUTTON_DIFFICULTY_EASY => shared::Difficulty::Easy,
                        BUTTON_DIFFICULTY_HARD => shared::Difficulty::Hard,
                        _ => shared::Difficulty::Normal,
                    };
                    App::kv_set("difficulty", self.difficulty.label());
                }
                BUTTON_BACK => {
                    return Some(StateSort::MainMenu(MainMenu::default()));
                }
                BUTTON_MUSIC_MINUS => {
                    self.music_volume = (self.music_volume - 1).clamp(0, 10);
                    self.save_volume();
                }
                BUTTON_MUSIC_PLUS => {
                    self.music_volume = (self.music_volume + 1).clamp(0, 10);
                    self.save_volume();
                }
                BUTTON_SOUND_MINUS => {
                    self.clip_volume = (self.clip_volume - 1).clamp(0, 10);
                    self.save_volume();
                }
                BUTTON_SOUND_PLUS => {
                    self.clip_volume = (self.clip_volume + 1).clamp(0, 10);
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
            (16, 236),
            (96, 20),
            BUTTON_BACK,
            LabelTrim::Return,
            LabelTheme::Default,
            crate::app::ContentElement::Text("Back".to_string(), Alignment::Center),
        );

        let button_music_minus = ButtonElement::new(
            (-16, 68),
            (20, 20),
            BUTTON_MUSIC_MINUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((88, 24), (8, 8)),
        );

        let button_music_plus = ButtonElement::new(
            (124, 68),
            (20, 20),
            BUTTON_MUSIC_PLUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((80, 24), (8, 8)),
        );

        let button_sound_minus = ButtonElement::new(
            (-16, 114),
            (20, 20),
            BUTTON_SOUND_MINUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((88, 24), (8, 8)),
        );

        let button_sound_plus = ButtonElement::new(
            (124, 114),
            (20, 20),
            BUTTON_SOUND_PLUS,
            LabelTrim::Round,
            LabelTheme::Default,
            crate::app::ContentElement::Sprite((80, 24), (8, 8)),
        );

        let controls_value = if Self::onscreen_controls_enabled() {
            BUTTON_CONTROLS_ON
        } else {
            BUTTON_CONTROLS_OFF
        };
        let mut controls = ButtonGroupElement::new(
            (-16, 206),
            vec![
                ButtonElement::new(
                    (0, 0),
                    (76, 22),
                    BUTTON_CONTROLS_ON,
                    LabelTrim::Round,
                    LabelTheme::Default,
                    ContentElement::Text("On".into(), Alignment::Center),
                ),
                ButtonElement::new(
                    (84, 0),
                    (76, 22),
                    BUTTON_CONTROLS_OFF,
                    LabelTrim::Round,
                    LabelTheme::Default,
                    ContentElement::Text("Off".into(), Alignment::Center),
                ),
            ],
            controls_value,
        );
        controls.select_group_value(controls_value);

        let difficulty = shared::Difficulty::from_preference(&App::kv_get("difficulty"));
        let difficulty_value = match difficulty {
            shared::Difficulty::Easy => BUTTON_DIFFICULTY_EASY,
            shared::Difficulty::Normal => BUTTON_DIFFICULTY_NORMAL,
            shared::Difficulty::Hard => BUTTON_DIFFICULTY_HARD,
        };
        let mut difficulty_buttons = ButtonGroupElement::new(
            (-16, 160),
            [
                (0, 44, "Easy", BUTTON_DIFFICULTY_EASY),
                (52, 56, "Normal", BUTTON_DIFFICULTY_NORMAL),
                (116, 44, "Hard", BUTTON_DIFFICULTY_HARD),
            ]
            .into_iter()
            .map(|(x, width, label, value)| {
                ButtonElement::new(
                    (x, 0),
                    (width, 22),
                    value,
                    LabelTrim::Round,
                    LabelTheme::Default,
                    ContentElement::Text(label.into(), Alignment::Center),
                )
            })
            .collect(),
            difficulty_value,
        );
        difficulty_buttons.select_group_value(difficulty_value);
        let mut interface = Interface::new(vec![
            controls.boxed(),
            difficulty_buttons.boxed(),
            button_back.boxed(),
            button_music_minus.boxed(),
            button_music_plus.boxed(),
            button_sound_minus.boxed(),
            button_sound_plus.boxed(),
        ]);

        if cfg!(feature = "mobile") {
            interface = Interface::new(vec![
                interface.boxed(),
                ButtonElement::new(
                    (176, 188),
                    (144, 22),
                    99,
                    LabelTrim::Round,
                    LabelTheme::Default,
                    ContentElement::Text("Restore Purchases".into(), Alignment::Center),
                )
                .boxed(),
                ButtonElement::new(
                    (176, 220),
                    (144, 22),
                    100,
                    LabelTrim::Round,
                    LabelTheme::Default,
                    ContentElement::Text("Reviewer Access".into(), Alignment::Center),
                )
                .boxed(),
            ]);
        }
        let (music_volume, clip_volume) = SettingsMenu::load_volume();

        SettingsMenu {
            interface,
            reset_star: ResetStar::default(),
            difficulty: shared::Difficulty::from_preference(&App::kv_get("difficulty")),
            music_volume,
            clip_volume,
        }
    }
}
