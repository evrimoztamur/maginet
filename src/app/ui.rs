use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use super::Pointer;
use crate::draw::{draw_sprite, draw_text};

pub enum UIEvent {
    ButtonClick(usize),
}

pub trait UIElement {
    fn tick(&mut self, pointer: &Pointer) -> Option<UIEvent> {
        None
    }
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        pointer: &Pointer,
        frame: u64,
    ) -> Result<(), JsValue>;
}

pub enum Alignment {
    Start,
    Center,
    End,
}

pub enum ContentElement {
    Text(String, Alignment),
    Sprite((i32, i32), (i32, i32)),
    List(Vec<ContentElement>),
}

impl UIElement for ContentElement {
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        pointer: &Pointer,
        frame: u64,
    ) -> Result<(), JsValue> {
        context.save();

        match self {
            ContentElement::Text(text, _) => {
                draw_text(context, atlas, text.len() as f64 * -4.0, -4.0, text)
            }
            ContentElement::Sprite(_, _) => todo!(),
            ContentElement::List(_) => todo!(),
        }?;

        context.restore();

        Ok(())
    }
}

#[derive(PartialEq)]
pub enum ButtonTrim {
    Round,
    Glorious,
}

#[derive(PartialEq)]
pub enum ButtonClass {
    Default,
    Action,
}

pub struct ButtonElement {
    position: (i32, i32),
    size: (i32, i32),
    value: usize,
    trim: ButtonTrim,
    class: ButtonClass,
    content: ContentElement,
    selected: bool,
}

impl ButtonElement {
    pub fn new(
        position: (i32, i32),
        size: (i32, i32),
        value: usize,
        trim: ButtonTrim,
        class: ButtonClass,
        content: ContentElement,
    ) -> ButtonElement {
        ButtonElement {
            position,
            size,
            value,
            trim,
            class,
            content,
            selected: false,
        }
    }

    fn hovered(&self, pointer: &Pointer) -> bool {
        let pointer_location = pointer.location();

        pointer_location.0 >= self.position.0
            && pointer_location.0 < self.position.0 + self.size.0
            && pointer_location.1 >= self.position.1
            && pointer_location.1 < self.position.1 + self.size.1
    }

    fn clicked(&self, pointer: &Pointer) -> bool {
        self.hovered(pointer) && pointer.clicked()
    }
}

impl UIElement for ButtonElement {
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        pointer: &Pointer,
        frame: u64,
    ) -> Result<(), JsValue> {
        context.save();

        context.translate(self.position.0 as f64, self.position.1 as f64)?;

        let fs = context.fill_style();

        match self.class {
            ButtonClass::Default => {
                if self.selected {
                    context.set_fill_style(&"#007faa".into());
                } else if self.hovered(pointer) {
                    context.set_fill_style(&"#008080".into());
                } else {
                    context.set_fill_style(&"#006080".into());
                }
            }
            ButtonClass::Action => {
                if self.selected {
                    context.set_fill_style(&"#007faa".into());
                } else if self.hovered(pointer) {
                    context.set_fill_style(&"#aa5f00".into());
                } else {
                    context.set_fill_style(&"#7f1f00".into());
                }
            }
        }

        context.fill_rect(0.0, 0.0, self.size.0 as f64, self.size.1 as f64);
        context.set_fill_style(&fs);

        context.translate(self.size.0 as f64 / 2.0, self.size.1 as f64 / 2.0)?;

        self.content.draw(context, atlas, pointer, frame)?;

        context.set_global_composite_operation("destination-out")?;

        let trim_position = match self.trim {
            ButtonTrim::Round => (80.0, 0.0),
            ButtonTrim::Glorious => (88.0, 0.0),
        };

        draw_sprite(
            context,
            atlas,
            trim_position.0,
            trim_position.1,
            4.0,
            4.0,
            -self.size.0 as f64 / 2.0,
            -self.size.1 as f64 / 2.0,
        )?;
        draw_sprite(
            context,
            atlas,
            trim_position.0 + 4.0,
            trim_position.1,
            4.0,
            4.0,
            self.size.0 as f64 / 2.0 - 4.0,
            -self.size.1 as f64 / 2.0,
        )?;
        draw_sprite(
            context,
            atlas,
            trim_position.0,
            trim_position.1 + 4.0,
            4.0,
            4.0,
            -self.size.0 as f64 / 2.0,
            self.size.1 as f64 / 2.0 - 4.0,
        )?;
        draw_sprite(
            context,
            atlas,
            trim_position.0 + 4.0,
            trim_position.1 + 4.0,
            4.0,
            4.0,
            self.size.0 as f64 / 2.0 - 4.0,
            self.size.1 as f64 / 2.0 - 4.0,
        )?;

        if self.trim == ButtonTrim::Glorious {
            context.fill_rect(
                -self.size.0 as f64 / 2.0,
                -self.size.1 as f64 / 2.0 + 2.0,
                2.0,
                self.size.1 as f64 - 4.0,
            );
            context.fill_rect(
                self.size.0 as f64 / 2.0 - 2.0,
                -self.size.1 as f64 / 2.0 + 2.0,
                2.0,
                self.size.1 as f64 - 4.0,
            );
        }

        context.restore();

        Ok(())
    }

    fn tick(&mut self, pointer: &Pointer) -> Option<UIEvent> {
        if self.clicked(&pointer) {
            Some(UIEvent::ButtonClick(self.value))
        } else {
            None
        }
    }
}

pub struct ButtonGroupElement {
    position: (i32, i32),
    buttons: Vec<ButtonElement>,
    value: usize,
}

impl ButtonGroupElement {
    pub fn new(
        position: (i32, i32),
        buttons: Vec<ButtonElement>,
        value: usize,
    ) -> ButtonGroupElement {
        ButtonGroupElement {
            position,
            buttons,
            value,
        }
    }
}

impl UIElement for ButtonGroupElement {
    fn tick(&mut self, pointer: &Pointer) -> Option<UIEvent> {
        let pointer = pointer.teleport((-self.position.0, -self.position.1));
        let mut event = None;

        for button in self.buttons.iter_mut() {
            if let Some(child_event) = button.tick(&pointer) {
                self.value = button.value;
                event = Some(child_event);
            }

            button.selected = self.value == button.value;
        }

        event
    }

    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        pointer: &Pointer,
        frame: u64,
    ) -> Result<(), JsValue> {
        context.save();

        context.translate(self.position.0 as f64, self.position.1 as f64)?;

        let pointer = pointer.teleport((-self.position.0, -self.position.1));

        for button in &self.buttons {
            button.draw(context, atlas, &pointer, frame)?;
        }

        context.restore();

        Ok(())
    }
}

pub struct Interface {
    children: Vec<Box<dyn UIElement>>,
}

impl Interface {
    pub fn new(children: Vec<Box<dyn UIElement>>) -> Interface {
        Interface { children }
    }
}

impl UIElement for Interface {
    fn tick(&mut self, pointer: &Pointer) -> Option<UIEvent> {
        let mut event = None;

        for child in &mut self.children {
            if let Some(child_event) = child.tick(pointer) {
                event = Some(child_event);
            }
        }

        event
    }
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        pointer: &Pointer,
        frame: u64,
    ) -> Result<(), JsValue> {
        for child in &self.children {
            child.draw(context, atlas, pointer, frame)?;
        }
        Ok(())
    }
}
