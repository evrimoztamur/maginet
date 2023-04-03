use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::app::{AppContext, StateSort};

pub trait State {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        app_context: &AppContext,
    ) -> Result<(), JsValue>;

    fn tick(&mut self, app_context: &AppContext) -> Option<StateSort>;
}
