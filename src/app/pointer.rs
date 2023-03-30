#[derive(Clone, Default)]
pub struct Pointer {
    previous: Option<Box<Pointer>>,
    pub location: (i32, i32),
    pub button: bool,
    pub alt_button: bool,
}

impl Pointer {
    pub fn new() -> Pointer {
        Pointer {
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
}
