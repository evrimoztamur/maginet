use super::*;
use crate::app::{pointer::GestureEvent, ClipId, Particle, ParticleSort, ParticleSystem};

const CENTER: (f64, f64) = (296.0, 36.0);
const CLICK_WINDOW: u64 = 45;

#[derive(Default)]
struct Spring {
    position: f64,
    velocity: f64,
}
impl Spring {
    fn step(&mut self, target: f64) {
        // Two fixed substeps per logical 60 Hz frame keep an underdamped spring stable.
        self.velocity += ((target - self.position) * 180.0 - self.velocity * 12.0) / 120.0;
        self.position += self.velocity / 120.0;
    }
}

#[derive(Default)]
struct ClickSequence {
    first: Option<u64>,
    count: u8,
    cooldown_until: u64,
}
impl ClickSequence {
    fn expire(&mut self, frame: u64) {
        if self
            .first
            .is_some_and(|first| frame.saturating_sub(first) > CLICK_WINDOW)
        {
            self.first = None;
            self.count = 0;
        }
    }
    fn click(&mut self, frame: u64) -> bool {
        if frame < self.cooldown_until {
            return false;
        }
        self.expire(frame);
        self.first.get_or_insert(frame);
        self.count += 1;
        if self.count == 3 {
            self.count = 0;
            self.first = None;
            self.cooldown_until = frame + 90;
            true
        } else {
            false
        }
    }
}

pub(super) struct ResetStar {
    x: Spring,
    y: Spring,
    clicks: ClickSequence,
    last_frame: Option<u64>,
    exploded_at: Option<u64>,
    failed_at: Option<u64>,
    particles: ParticleSystem,
}
impl Default for ResetStar {
    fn default() -> Self {
        Self {
            x: Spring::default(),
            y: Spring::default(),
            clicks: ClickSequence::default(),
            last_frame: None,
            exploded_at: None,
            failed_at: None,
            particles: ParticleSystem::default(),
        }
    }
}
impl ResetStar {
    fn center(&self) -> (f64, f64) {
        (
            (CENTER.0 + self.x.position).round(),
            (CENTER.1 + self.y.position).round(),
        )
    }
    fn hovered(&self, pointer: &crate::app::Pointer) -> bool {
        let c = self.center();
        (pointer.location.0 as f64 - c.0).abs() < 22.0
            && (pointer.location.1 as f64 - c.1).abs() < 22.0
    }
    pub fn tick(&mut self, app: &AppContext) -> bool {
        let frame = app.frame;
        let elapsed = self
            .last_frame
            .replace(frame)
            .map_or(1, |last| frame.saturating_sub(last))
            .min(8);
        for step in 0..elapsed * 2 {
            let t = frame as f64 - elapsed as f64 + step as f64 / 2.0;
            self.x.step((t * 0.035).sin() * 1.5);
            self.y.step((t * 0.045).sin() * 2.0);
        }
        self.clicks.expire(frame);
        if app
            .pointer
            .gestures
            .iter()
            .any(|e| matches!(e, GestureEvent::Cancel))
        {
            self.clicks.first = None;
            self.clicks.count = 0;
        }
        if !app.pointer.clicked()
            || !self.hovered(&app.pointer)
            || frame < self.clicks.cooldown_until
        {
            return false;
        }
        let direction = if self.clicks.count % 2 == 0 {
            1.0
        } else {
            -1.0
        };
        self.x.velocity += direction * 110.0;
        self.y.velocity -= 125.0;
        app.audio_system.play_clip(ClipId::ClickForward);
        self.clicks.click(frame)
    }
    pub fn complete(&mut self, app: &AppContext, success: bool) {
        if !success {
            self.failed_at = Some(app.frame);
            return;
        }
        self.exploded_at = Some(app.frame);
        self.failed_at = None;
        let c = self.center();
        for _ in 0..96 {
            let angle = js_sys::Math::random() * std::f64::consts::TAU;
            let speed = 0.12 + js_sys::Math::random() * 0.65;
            self.particles.add(Particle::new(
                (c.0 / 32.0 - 0.5, c.1 / 32.0 - 0.5),
                (angle.cos() * speed, angle.sin() * speed),
                35 + (js_sys::Math::random() * 55.0) as u64,
                ParticleSort::Shield,
            ));
        }
        app.audio_system.play_clip(ClipId::StarSparkle);
    }
    pub fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlCanvasElement,
        app: &AppContext,
    ) -> Result<(), JsValue> {
        if !self
            .exploded_at
            .is_some_and(|at| app.frame.saturating_sub(at) < 45)
        {
            let c = self.center();
            context.save();
            context.translate(c.0, c.1)?;
            draw_sprite(context, atlas, 32.0, 320.0, 32.0, 32.0, -16.0, -16.0)?;
            context.restore();
        }
        self.particles.tick_and_draw(context, atlas, app.frame)?;
        let text = if self
            .failed_at
            .is_some_and(|at| app.frame.saturating_sub(at) < 150)
        {
            "Reset failed"
        } else if self
            .exploded_at
            .is_some_and(|at| app.frame.saturating_sub(at) < 150)
        {
            "Campaign reset"
        } else if self.clicks.count == 1 {
            "2 more clicks"
        } else if self.clicks.count == 2 {
            "1 more click"
        } else if self.hovered(&app.pointer) {
            "Reset campaign"
        } else {
            ""
        };
        draw_text(context, atlas, 180.0, 184.0, text)?;
        if self.hovered(&app.pointer) && self.clicks.count == 0 && text == "Reset campaign" {
            draw_text(context, atlas, 180.0, 196.0, "Click star 3x")?;
        }
        Ok(())
    }
}

/// Both historical raw codes and the styled canonical keys are campaign-only records.
fn progress_keys() -> Vec<String> {
    let mut keys = Vec::new();
    for entry in shared::campaign_catalogue(false) {
        keys.push(entry.level().as_code());
        keys.push(entry.code);
    }
    keys.push(shared::Level::from(shared::TUTORIAL_CODE).as_code());
    for key in keys.clone() {
        keys.extend(
            shared::campaign_progress_aliases(&key)
                .iter()
                .map(|alias| (*alias).to_owned()),
        );
    }
    keys.sort();
    keys.dedup();
    keys
}
pub(super) fn reset_campaign_progress() -> Result<(), JsValue> {
    let storage = crate::storage().ok_or_else(|| JsValue::from_str("Save storage unavailable"))?;
    for key in progress_keys() {
        storage.remove_item(&key)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn triple_click_is_timed_and_fires_once() {
        let mut clicks = ClickSequence::default();
        assert!(!clicks.click(10));
        assert!(!clicks.click(20));
        assert!(!clicks.click(60)); // Earlier clicks expired.
        assert!(!clicks.click(65));
        assert!(clicks.click(70));
        assert!(!clicks.click(71));
        assert!(!clicks.click(72));
        assert!(!clicks.click(73));
        assert_eq!(clicks.count, 0);
    }
    #[test]
    fn spring_overshoots_and_settles_without_drifting() {
        let mut spring = Spring {
            position: 0.0,
            velocity: 110.0,
        };
        let mut peak: f64 = 0.0;
        let mut crossed = false;
        for _ in 0..600 {
            spring.step(0.0);
            peak = peak.max(spring.position);
            crossed |= spring.position < 0.0;
        }
        assert!(peak > 3.0 && crossed);
        assert!(spring.position.abs() < 0.001 && spring.velocity.abs() < 0.001);
    }
    #[test]
    fn reset_keys_cover_full_campaign_and_exclude_other_saves() {
        let keys = progress_keys();
        assert!(shared::campaign_catalogue(false)
            .iter()
            .all(|e| keys.contains(&e.level().as_code())));
        for key in [
            "levels",
            "music_volume",
            "clip_volume",
            "difficulty",
            "onscreen_controls",
        ] {
            assert!(!keys.iter().any(|k| k == key));
        }
    }
}
