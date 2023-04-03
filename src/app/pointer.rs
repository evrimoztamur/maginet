use crate::{CANVAS_VERTICAL, PADDING_X, PADDING_Y};

#[derive(Clone, Default)]
pub struct Pointer {
    previous: Option<Box<Pointer>>,
    location: (i32, i32),
    pub button: bool,
    pub alt_button: bool,
    vertical: bool,
}

impl Pointer {
    pub fn new() -> Pointer {
        Pointer {
            vertical: CANVAS_VERTICAL,
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

        if self.vertical {
            returned.location.0 -= location.1;
            returned.location.1 += location.0;
        } else {
            returned.location.0 += location.0;
            returned.location.1 += location.1;
        }

        returned
    }

    pub fn location(&self) -> (i32, i32) {
        if self.vertical {
            (self.location.1 - 64, 256 - self.location.0)
        } else {
            (self.location.0 - PADDING_X as i32, self.location.1 - PADDING_Y as i32)
        }
    }

    pub fn real(&self) -> &(i32, i32) {
        &self.location
    }

    pub fn set_real(&mut self, location: (i32, i32)) {
        self.location = location;
    }
}
