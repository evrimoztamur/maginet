use super::CanvasSettings;

#[derive(Clone, Copy, Debug)]
pub enum GestureEvent {
    Press((i32, i32)),
    Move((i32, i32)),
    Release((i32, i32)),
    Cancel,
}

#[derive(Clone, Default)]
pub struct Pointer {
    previous: Option<Box<Pointer>>,
    pub gestures: Vec<GestureEvent>,
    pub active_touch: Option<i32>,
    pub suppress_mouse_until: f64,
    gesture_down: bool,
    touch_cursor: bool,
    cursor_position: Option<(f64, f64)>,
    cursor_updated_at: Option<f64>,
    pub location: (i32, i32),
    pub button: bool,
    pub pending_click: bool,
    pub alt_button: bool,
}

impl Pointer {
    pub fn new(canvas_settings: &CanvasSettings) -> Pointer {
        let midpoint = (
            canvas_settings.element_width() as i32 / 2,
            canvas_settings.element_height() as i32 / 2,
        );

        Pointer {
            location: Pointer::location_from_real(canvas_settings, midpoint),
            ..Default::default()
        }
    }

    pub fn is_touch(&self) -> bool {
        self.touch_cursor
    }

    pub fn accepts_touch(&self, identifier: i32) -> bool {
        self.active_touch == Some(identifier)
    }

    pub fn begin_touch(&mut self, identifier: i32) -> bool {
        if self.active_touch.is_some() {
            return false;
        }
        self.active_touch = Some(identifier);
        self.touch_cursor = true;
        true
    }

    pub fn move_mouse_to(&mut self, location: (i32, i32)) {
        self.touch_cursor = false;
        self.move_to(location);
    }

    /// Animate only the drawn touch cursor; hit testing uses `location` immediately.
    pub fn draw_location(&mut self, now_ms: f64) -> (f64, f64) {
        let target = (self.location.0 as f64, self.location.1 as f64);
        let elapsed = self
            .cursor_updated_at
            .replace(now_ms)
            .map(|last| (now_ms - last).max(0.0));
        let position = match (self.touch_cursor, self.cursor_position, elapsed) {
            (true, Some(position), Some(elapsed)) => {
                // Reach 99% of the target in about 90ms, independent of refresh rate.
                let blend = 1.0 - (-elapsed / 20.0).exp();
                let next = (
                    position.0 + (target.0 - position.0) * blend,
                    position.1 + (target.1 - position.1) * blend,
                );
                if (target.0 - next.0).abs().max((target.1 - next.1).abs()) < 0.5 {
                    target
                } else {
                    next
                }
            }
            _ => target,
        };
        self.cursor_position = Some(position);
        position
    }

    pub fn press(&mut self, location: (i32, i32)) {
        self.location = location;
        self.gesture_down = true;
        self.gestures.push(GestureEvent::Press(location));
    }

    pub fn move_to(&mut self, location: (i32, i32)) {
        self.location = location;
        if self.gesture_down {
            self.gestures.push(GestureEvent::Move(location));
        }
    }

    pub fn release(&mut self, location: (i32, i32)) {
        self.move_to(location);
        if self.gesture_down {
            self.gestures.push(GestureEvent::Release(location));
        }
        self.gesture_down = false;
        self.button = false;
        self.active_touch = None;
    }

    pub fn cancel(&mut self) {
        self.gestures.clear();
        self.gestures.push(GestureEvent::Cancel);
        self.gesture_down = false;
        self.active_touch = None;
        self.alt_button = false;
        self.button = false;
        self.pending_click = false;
    }

    pub fn without_clicks(&self) -> Self {
        let mut pointer = self.clone();
        pointer.button = false;
        pointer.pending_click = false;
        pointer.alt_button = false;
        pointer
    }

    pub fn clicked(&self) -> bool {
        if self.pending_click {
            return true;
        }
        match &self.previous {
            Some(pointer) => self.button && !pointer.button,
            None => self.button,
        }
    }

    pub fn alt_clicked(&self) -> bool {
        match &self.previous {
            Some(pointer) => self.alt_button && !pointer.alt_button,
            None => self.alt_button,
        }
    }

    pub fn swap(&mut self) {
        self.pending_click = false;
        self.gestures.clear();
        self.previous.take(); // Must explicitly drop old Pointer from heap
        self.previous = Some(Box::new(self.clone()));
    }

    pub fn teleport(&self, location: (i32, i32)) -> Pointer {
        let mut returned = self.clone();

        returned.location.0 += location.0;
        returned.location.1 += location.1;
        returned
    }

    pub fn location_from_real(canvas_settings: &CanvasSettings, real: (i32, i32)) -> (i32, i32) {
        let flip = (
            canvas_settings.interface_width as i32,
            canvas_settings.interface_height as i32,
        );

        let padding = canvas_settings.padding();

        if canvas_settings.orientation {
            (real.1 - padding.0, flip.0 - 1 - (real.0 - padding.1))
        } else {
            (real.0 - padding.0, real.1 - padding.1)
        }
    }

    pub fn in_region(&self, position: (i32, i32), size: (i32, i32)) -> bool {
        self.location.0 >= position.0
            && self.location.0 < position.0 + size.0
            && self.location.1 >= position.1
            && self.location.1 < position.1 + size.1
    }
}

#[cfg(test)]
mod tests {
    use super::Pointer;

    #[test]
    fn touch_cursor_eases_after_release_without_delaying_clicks() {
        let mut pointer = Pointer::default();
        assert_eq!(pointer.draw_location(0.0), (0.0, 0.0));
        pointer.begin_touch(1);
        pointer.press((200, 100));
        pointer.release((200, 100));
        pointer.pending_click = true;
        let cursor = pointer.draw_location(16.0);
        assert!(cursor.0 > 0.0 && cursor.0 < 200.0);
        assert_eq!(pointer.location, (200, 100));
        assert!(pointer.clicked());
        pointer.swap();
        assert_eq!(pointer.draw_location(160.0), (200.0, 100.0));
        assert!(!pointer.clicked());
    }

    #[test]
    fn mouse_movement_immediately_interrupts_touch_animation() {
        let mut pointer = Pointer::default();
        pointer.draw_location(0.0);
        pointer.begin_touch(1);
        pointer.release((200, 100));
        pointer.draw_location(16.0);
        pointer.move_mouse_to((40, 60));
        assert_eq!(pointer.draw_location(17.0), (40.0, 60.0));
    }

    #[test]
    fn touch_cursor_motion_is_independent_of_refresh_rate() {
        let mut pointer = Pointer::default();
        pointer.draw_location(0.0);
        pointer.begin_touch(1);
        pointer.press((200, 100));
        let mut faster = pointer.clone();
        for frame in 1..=3 {
            pointer.draw_location(frame as f64 * 1000.0 / 60.0);
        }
        for frame in 1..=6 {
            faster.draw_location(frame as f64 * 1000.0 / 120.0);
        }
        let a = pointer.cursor_position.unwrap();
        let b = faster.cursor_position.unwrap();
        assert!((a.0 - b.0).abs() < 1e-9);
        assert!((a.1 - b.1).abs() < 1e-9);
    }

    #[test]
    fn short_gestures_keep_press_motion_and_matching_release_until_swap() {
        let mut p = Pointer::default();
        assert!(p.begin_touch(7));
        p.press((10, 20));
        assert!(!p.begin_touch(8));
        assert!(!p.accepts_touch(8));
        assert!(p.accepts_touch(7));
        p.move_to((42, 20));
        p.release((44, 22));
        assert!(matches!(
            p.gestures.first(),
            Some(super::GestureEvent::Press((10, 20)))
        ));
        assert!(matches!(
            p.gestures.last(),
            Some(super::GestureEvent::Release((44, 22)))
        ));
        assert_eq!(p.location, (44, 22));
        assert_eq!(p.active_touch, None);
        p.swap();
        assert!(p.gestures.is_empty());
    }

    #[test]
    fn cancellation_discards_pending_release_and_click() {
        let mut p = Pointer::default();
        p.press((0, 0));
        p.release((32, 0));
        p.pending_click = true;
        p.cancel();
        p.release((64, 0));
        assert_eq!(p.gestures.len(), 1);
        assert!(matches!(p.gestures[0], super::GestureEvent::Cancel));
        assert!(!p.clicked());
    }

    #[test]
    fn released_touch_is_consumed_once() {
        let mut pointer = Pointer {
            pending_click: true,
            button: false,
            ..Pointer::default()
        };
        assert!(pointer.clicked());
        assert!(pointer.teleport((10, 10)).clicked());
        pointer.swap();
        assert!(!pointer.clicked());
    }
}
