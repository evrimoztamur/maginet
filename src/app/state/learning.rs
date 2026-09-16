use shared::LoadoutMethod;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::{
    app::{Alignment, AppContext, ContentElement, LabelTrim, BOARD_SCALE},
    draw::{draw_label, draw_text_centered, text_length},
};

#[derive(Clone, Copy)]
pub(super) struct BoardHint {
    pub title: &'static str,
    pub lines: &'static [&'static str],
}

impl BoardHint {
    pub fn for_campaign(loadout: &LoadoutMethod) -> Option<Self> {
        let LoadoutMethod::Arena(_, position) = loadout else {
            return None;
        };
        let entries = shared::campaign_catalogue(false);
        let entry = entries.iter().find(|entry| entry.position == *position)?;
        Some(match entry.id.as_str() {
            "diagonals-i" => Self {
                title: "Diagon Rune",
                lines: &["This rune allows you to move", "diagonally, too."],
            },
            "shields-i" => Self {
                title: "Shield Rune",
                lines: &["This rune reflects attacks", "to the enemy."],
            },
            "beams-i" => Self {
                title: "Beam Crystal",
                lines: &[
                    "This crystal discharges a",
                    "strong cardinal beam, hurting",
                    "all in its way.",
                ],
            },
            _ => return None,
        })
    }

    pub fn draw(
        self,
        context: &CanvasRenderingContext2d,
        interface_context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app: &AppContext,
        offset: (i32, i32),
        size: (usize, usize),
    ) -> Result<(), JsValue> {
        let center_x = offset.0 + size.0 as i32 * BOARD_SCALE.0 / 2;
        let title_height = 24;
        let margin = 16;
        draw_label(
            context,
            atlas,
            (center_x - 48, offset.1 - margin - title_height),
            (96, title_height),
            "#557F55",
            &ContentElement::Text(self.title.into(), Alignment::Center),
            &app.pointer,
            app.frame,
            &LabelTrim::Glorious,
            false,
        )?;
        let text_top = (offset.1 + size.1 as i32 * BOARD_SCALE.1 + margin) as f64;
        let text_width = self
            .lines
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
            self.lines.len() as f64 * 14.0,
        );
        for (i, line) in self.lines.iter().enumerate() {
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn only_the_three_campaign_introductions_have_opening_hints() {
        let entries = shared::campaign_catalogue(false);
        for entry in entries {
            let loadout = LoadoutMethod::Arena(entry.level(), entry.position);
            assert_eq!(
                BoardHint::for_campaign(&loadout).is_some(),
                matches!(entry.id.as_str(), "diagonals-i" | "beams-i" | "shields-i"),
                "{}",
                entry.id,
            );
            assert!(BoardHint::for_campaign(&LoadoutMethod::Prefab(entry.level())).is_none());
        }
    }
}
