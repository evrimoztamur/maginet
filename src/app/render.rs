//! Rasterize at native resolution, then enlarge complete pixels for display.
use std::cell::RefCell;

use wasm_bindgen::{JsCast, JsValue};
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use super::{App, CanvasSettings};
use crate::{document, window};

pub struct CanvasLayer {
    pub canvas: HtmlCanvasElement,
    pub context: CanvasRenderingContext2d,
}

impl CanvasLayer {
    pub fn new(width: u32, height: u32) -> Result<Self, JsValue> {
        let canvas = document()
            .create_element("canvas")?
            .dyn_into::<HtmlCanvasElement>()?;
        canvas.set_width(width);
        canvas.set_height(height);
        let context = canvas
            .get_context("2d")?
            .unwrap()
            .dyn_into::<CanvasRenderingContext2d>()?;
        context.set_image_smoothing_enabled(false);
        Ok(Self { canvas, context })
    }
}

pub struct Renderer {
    pub display: CanvasLayer,
    game: CanvasLayer,
    interface: CanvasLayer,
    settings: CanvasSettings,
    outgoing: CanvasLayer,
    composite: CanvasLayer,
    transition: RefCell<Option<(f64, i8)>>,
}

impl Renderer {
    pub fn new(settings: &CanvasSettings) -> Result<Self, JsValue> {
        let (width, height) = (settings.element_width(), settings.element_height());
        let display = CanvasLayer::new(width, height)?;
        display.canvas.set_id("game-canvas");
        Ok(Self {
            display,
            game: CanvasLayer::new(width, height)?,
            interface: CanvasLayer::new(width, height)?,
            settings: settings.clone(),
            outgoing: CanvasLayer::new(width, height)?,
            composite: CanvasLayer::new(width, height)?,
            transition: RefCell::new(None),
        })
    }

    pub fn resize(&self) -> Result<(), JsValue> {
        self.transition.replace(None);
        if cfg!(feature = "ios") {
            let w = window().inner_width()?.as_f64().unwrap();
            let h = window().inner_height()?.as_f64().unwrap();
            let dpr = window().device_pixel_ratio();
            // Sprite transforms must stay on the logical pixel grid. Fractional
            // scaling belongs only to the final, already-composited image.
            let logical_width = (w * 272.0 / h).ceil() as u32;
            for layer in [&self.game, &self.interface, &self.outgoing, &self.composite] {
                layer.canvas.set_width(logical_width);
                layer.canvas.set_height(272);
                layer.context.set_image_smoothing_enabled(false);
            }
            self.display.canvas.set_width((w * dpr).round() as u32);
            self.display.canvas.set_height((h * dpr).round() as u32);
            self.display.context.set_image_smoothing_enabled(false);
            self.display
                .canvas
                .style()
                .set_property("width", &format!("{w}px"))?;
            self.display
                .canvas
                .style()
                .set_property("height", &format!("{h}px"))?;
            return Ok(());
        }
        let window = window();
        let dpr = window.device_pixel_ratio().max(0.1);
        let nav_height = document()
            .query_selector("nav")?
            .map_or(0.0, |nav| nav.get_bounding_client_rect().height());
        let available = (
            window.inner_width()?.as_f64().unwrap(),
            (window.inner_height()?.as_f64().unwrap() - nav_height - 16.0).max(1.0),
        );
        let scale = self.settings.display_scale(available, dpr);
        let width = self.settings.element_width() * scale;
        let height = self.settings.element_height() * scale;
        self.display.canvas.set_width(width);
        self.display.canvas.set_height(height);
        // Resizing resets context state, including image smoothing.
        self.display.context.set_image_smoothing_enabled(false);
        self.display
            .canvas
            .style()
            .set_property("width", &format!("{}px", width as f64 / dpr))?;
        self.display
            .canvas
            .style()
            .set_property("height", &format!("{}px", height as f64 / dpr))?;
        // Centered flex layout can otherwise place the entire canvas on half a device pixel.
        self.display
            .canvas
            .style()
            .set_property("transform", "none")?;
        let bounds = self.display.canvas.get_bounding_client_rect();
        let dx = (bounds.left() * dpr).round() / dpr - bounds.left();
        let dy = (bounds.top() * dpr).round() / dpr - bounds.top();
        self.display
            .canvas
            .style()
            .set_property("transform", &format!("translate({dx}px, {dy}px)"))?;
        Ok(())
    }

    pub fn draw(&self, app: &mut App, atlas: &HtmlCanvasElement) -> Result<(), JsValue> {
        let now = window().performance().unwrap().now();
        let native_width = self.game.canvas.width() as f64;
        let native_height = self.game.canvas.height() as f64;
        if let Some(direction) = app.take_navigation() {
            self.outgoing
                .context
                .clear_rect(0.0, 0.0, native_width, native_height);
            self.outgoing.context.draw_image_with_html_canvas_element(
                &self.game.canvas,
                0.0,
                0.0,
            )?;
            self.transition.replace(Some((now, direction)));
        }
        app.draw(&self.game.context, &self.interface.context, atlas)?;
        let width = self.display.canvas.width() as f64;
        let height = self.display.canvas.height() as f64;
        self.display.context.clear_rect(0.0, 0.0, width, height);
        // Composite at source resolution, then sample that complete image once.
        self.game
            .context
            .draw_image_with_html_canvas_element(&self.interface.canvas, 0.0, 0.0)?;
        self.composite
            .context
            .clear_rect(0.0, 0.0, native_width, native_height);
        let mut incoming_x = 0.0;
        if let Some((started, direction)) = *self.transition.borrow() {
            let progress = ((now - started) / 250.0).clamp(0.0, 1.0);
            let eased = 1.0 - (1.0 - progress).powi(3);
            incoming_x = (direction as f64 * native_width * (1.0 - eased)).round();
            let outgoing_x = incoming_x - direction as f64 * native_width;
            self.composite.context.draw_image_with_html_canvas_element(
                &self.outgoing.canvas,
                outgoing_x,
                0.0,
            )?;
        }
        self.composite.context.draw_image_with_html_canvas_element(
            &self.game.canvas,
            incoming_x,
            0.0,
        )?;
        if self
            .transition
            .borrow()
            .is_some_and(|(started, _)| now - started >= 250.0)
        {
            self.transition.replace(None);
        }
        app.draw_cursor(&self.composite.context, atlas)?;
        let drawn_width = if cfg!(feature = "ios") {
            // The logical width is rounded up to cover the viewport. Clip the
            // spare fraction at the right edge instead of stretching the pixels.
            self.game.canvas.width() as f64 * height / self.game.canvas.height() as f64
        } else {
            width
        };
        self.display
            .context
            .draw_image_with_html_canvas_element_and_dw_and_dh(
                &self.composite.canvas,
                0.0,
                0.0,
                drawn_width,
                height,
            )?;
        Ok(())
    }
}
