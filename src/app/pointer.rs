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
    pub real: (i32, i32),
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
            real: midpoint,
            location: Pointer::location_from_real(canvas_settings, midpoint),
            ..Default::default()
        }
    }

    pub fn accepts_touch(&self, identifier: i32) -> bool {
        self.active_touch == Some(identifier)
    }

    pub fn begin_touch(&mut self, identifier: i32) -> bool {
        if self.active_touch.is_some() {
            return false;
        }
        self.active_touch = Some(identifier);
        true
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
