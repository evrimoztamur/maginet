use super::CanvasSettings;

#[derive(Clone, Default)]
pub struct Pointer {
    previous: Option<Box<Pointer>>,
    location: (i32, i32),
    padding: (i32, i32),
    pub button: bool,
    pub alt_button: bool,
    orientation: bool,
}

impl Pointer {
    pub fn new(canvas_settings: &CanvasSettings) -> Pointer {
        Pointer {
            orientation: canvas_settings.orientation(),
            location: (
                canvas_settings.canvas_width() as i32 / 2,
                canvas_settings.canvas_height() as i32 / 2,
            ),
            padding: canvas_settings.padding(),
            ..Default::default()
        }
    }

    pub fn clicked(&self) -> bool {
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
        self.previous.take(); // Must explicitly drop old Pointer from heap
        self.previous = Some(Box::new(self.clone()));
    }

    pub fn teleport(&self, location: (i32, i32)) -> Pointer {
        let mut returned = self.clone();

        if self.orientation {
            returned.location.0 -= location.1;
            returned.location.1 += location.0;
        } else {
            returned.location.0 += location.0;
            returned.location.1 += location.1;
        }

        returned
    }

    pub fn location(&self) -> (i32, i32) {
        Pointer::location_from_real(self.real(), self.padding, self.orientation)
    }

    pub fn location_from_real(
        real: (i32, i32),
        padding: (i32, i32),
        orientation: bool,
    ) -> (i32, i32) {
        if orientation {
            (real.1 - padding.1, real.0 - padding.0)
        } else {
            (real.0 - padding.0, real.1 - padding.1)
        }
    }

    pub fn real(&self) -> (i32, i32) {
        self.location
    }

    pub fn set_real(&mut self, location: (i32, i32)) {
        self.location = location;
    }
}
