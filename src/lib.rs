mod access;
mod ai;
mod app;
mod draw;
mod net;

use std::{
    cell::{Cell, RefCell},
    pin::Pin,
    rc::Rc,
    task::{Context, Poll},
};

use app::{App, AudioSystem, CanvasLayer, CanvasSettings, Renderer};
use futures::Future;
use net::{fetch, request_session};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    Document, DomRect, FocusEvent, HtmlImageElement, HtmlInputElement, KeyboardEvent, MouseEvent,
    Storage, TouchEvent, Window,
};

fn window() -> Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> Document {
    window()
        .document()
        .expect("should have a document on window")
}

fn storage() -> Option<Storage> {
    window().local_storage().unwrap_or_default()
}

#[cfg(feature = "deploy")]
pub const RESOURCE_BASE_URL: &str = ".";
#[cfg(not(feature = "deploy"))]
pub const RESOURCE_BASE_URL: &str = "";

#[wasm_bindgen(start)]
async fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();
    if web_sys::window().is_none() {
        return Ok(());
    }

    let container_element = document()
        .query_selector("#canvas-container")
        .unwrap()
        .unwrap();

    let mut canvas_settings = CanvasSettings::new(
        384 + 16,
        256 + 16,
        256,
        256,
        window().inner_width().unwrap().as_f64().unwrap()
            < window().inner_height().unwrap().as_f64().unwrap(),
    );

    canvas_settings.update_ios();

    // atlas_img.set_src(&format!("{RESOURCE_BASE_URL}/static/png/atlas.png?v=6"));

    let atlas_future = ImageFuture::new(&format!("{RESOURCE_BASE_URL}/static/png/atlas.png?v=8"));
    // let atlas_img = atlas_future.await.unwrap();
    let atlas_img: Rc<HtmlImageElement> = Rc::new(atlas_future.await.unwrap());

    let mut audio_system = AudioSystem::default();
    audio_system.populate_audio().await;

    audio_system.play_music(app::ClipId::MusicI);

    {
        let atlas_img = atlas_img.clone();

        let renderer = Rc::new(Renderer::new(&canvas_settings)?);
        let canvas = renderer.display.canvas.clone();

        let text_input_element = document()
            .query_selector("#text-input")
            .unwrap()
            .unwrap()
            .dyn_into::<HtmlInputElement>()
            .unwrap();
        let text_input_element = Rc::new(RefCell::new(text_input_element));

        container_element.append_child(&canvas)?;
        renderer.resize()?;

        let atlas_layer = CanvasLayer::new(atlas_img.width(), atlas_img.height())?;
        atlas_layer
            .context
            .draw_image_with_html_image_element(&atlas_img, 0.0, 0.0)?;
        // Preserve the arrow pixels and transparency exactly; only replace their color.
        // This slot is outside the board caches and mage artwork.
        let white_arrows = CanvasLayer::new(32, 16)?;
        white_arrows
            .context
            .draw_image_with_html_canvas_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                &atlas_layer.canvas,
                0.0,
                32.0,
                32.0,
                16.0,
                0.0,
                0.0,
                32.0,
                16.0,
            )?;
        white_arrows
            .context
            .set_global_composite_operation("source-in")?;
        white_arrows.context.set_fill_style(&"#ffffff".into());
        white_arrows.context.fill_rect(0.0, 0.0, 32.0, 16.0);
        atlas_layer.context.draw_image_with_html_canvas_element(
            &white_arrows.canvas,
            160.0,
            144.0,
        )?;
        let atlas = atlas_layer.canvas;

        let app = App::new(&canvas_settings, audio_system.clone());

        let app = Rc::new(RefCell::new(app));

        let session_closure = {
            let app = app.clone();

            Closure::<dyn FnMut(JsValue)>::new(move |value| {
                let mut app = app.borrow_mut();
                app.on_session_response(value);
            })
        };

        let f = Rc::new(RefCell::new(None));
        let g = f.clone();

        {
            let app = app.clone();
            let text_input = text_input_element.clone();
            let renderer = renderer.clone();

            {
                let app = app.borrow();

                if app.session_id().is_none() && !cfg!(feature = "ios") {
                    let _ = fetch(&request_session()).then(&session_closure);
                }
            }

            *g.borrow_mut() = Some(Closure::new(move || {
                let mut app = app.borrow_mut();
                let text_input = text_input.borrow_mut();

                {
                    if crate::access::backgrounded() {
                        request_animation_frame(f.borrow().as_ref().unwrap());
                        return;
                    }
                    app.tick(&text_input);
                    renderer.draw(&mut app, &atlas).unwrap();
                }

                request_animation_frame(f.borrow().as_ref().unwrap());
            }));

            request_animation_frame(g.borrow().as_ref().unwrap());
        }

        session_closure.forget();

        {
            let app = app.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: TouchEvent| {
                app.borrow_mut().on_touch_cancel(event);
            });
            document().add_event_listener_with_callback(
                "touchcancel",
                closure.as_ref().unchecked_ref(),
            )?;
            closure.forget();
        }
        for event_name in ["maginet-background", "visibilitychange"] {
            let app = app.clone();
            let closure = Closure::<dyn FnMut(JsValue)>::new(move |_| {
                app.borrow_mut().cancel_input();
            });
            window()
                .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        for event_name in ["touchstart", "maginet-foreground"] {
            let audio = audio_system.clone();
            let closure = Closure::<dyn FnMut(JsValue)>::new(move |_| {
                audio.resume();
            });
            window()
                .add_event_listener_with_callback(event_name, closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        {
            let app = app.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |_: JsValue| {
                app.borrow_mut().cancel_input();
            });
            window().add_event_listener_with_callback("blur", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }
        let canvas = Rc::new(canvas);
        let bound: Rc<RefCell<Option<DomRect>>> =
            Rc::new(RefCell::new(Some(canvas.get_bounding_client_rect())));

        {
            let canvas = canvas.clone();
            let bound = bound.clone();
            let renderer = renderer.clone();
            let app = app.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |_: JsValue| {
                renderer.resize().unwrap();
                app.borrow_mut().resize_ios();
                bound.replace(Some(canvas.get_bounding_client_rect()));
            });
            window()
                .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let app = app.clone();
            let text_input = text_input_element.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |_: FocusEvent| {
                let mut app = app.borrow_mut();
                let text_input = text_input.borrow();
                app.on_input_submit(text_input.as_ref());
            });

            document()
                .add_event_listener_with_callback("focusout", closure.as_ref().unchecked_ref())?;

            closure.forget();
        }

        {
            let app = app.clone();
            let bound = bound.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
                let mut app = app.borrow_mut();
                if !cfg!(feature = "ios") {
                    if let Some(bound) = bound.borrow().as_deref() {
                        app.on_mouse_down(bound, event);
                    }
                }
            });
            document()
                .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let app = app.clone();
            let bound = bound.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
                let mut app = app.borrow_mut();
                if !cfg!(feature = "ios") {
                    if let Some(bound) = bound.borrow().as_deref() {
                        app.on_mouse_up(bound, event);
                    }
                }
            });
            document()
                .add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let app = app.clone();
            let bound = bound.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
                let mut app = app.borrow_mut();
                if let Some(bound) = bound.borrow().as_deref() {
                    if !cfg!(feature = "ios") {
                        app.on_mouse_move(bound, event);
                    }
                }
            });
            document()
                .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let app = app.clone();
            let bound = bound.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: TouchEvent| {
                let mut app = app.borrow_mut();
                if let Some(bound) = bound.borrow().as_deref() {
                    app.on_touch_move(bound, event);
                }
            });
            document()
                .add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let bound = bound.clone();
            let app = app.clone();

            let closure = Closure::<dyn FnMut(_)>::new(move |event: TouchEvent| {
                if let Some(bound) = bound.borrow().as_deref() {
                    let mut app = app.borrow_mut();
                    app.on_touch_start(bound, event);
                }
            });
            document()
                .add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let app = app.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: TouchEvent| {
                if let Some(bound) = bound.borrow().as_deref() {
                    let mut app = app.borrow_mut();
                    app.on_touch_end(bound, event);
                }
            });
            document()
                .add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let app = app.clone();
            let text_input = text_input_element.clone();
            let closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
                let mut app = app.borrow_mut();

                if event.code() == "Enter" {
                    let text_input = text_input.borrow();
                    app.on_input_submit(text_input.as_ref());
                } else {
                    app.on_key_down(event);
                }
            });
            document()
                .add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
            closure.forget();
        }

        {
            let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
                event.prevent_default();
            });
            document().add_event_listener_with_callback(
                "contextmenu",
                closure.as_ref().unchecked_ref(),
            )?;
            closure.forget();
        }
    }

    Ok(())
}

pub struct ImageFuture {
    image: Option<HtmlImageElement>,
    load_failed: Rc<Cell<bool>>,
}

impl ImageFuture {
    pub fn new(path: &str) -> Self {
        let image = HtmlImageElement::new().unwrap();
        image.set_src(path);
        ImageFuture {
            image: Some(image),
            load_failed: Rc::new(Cell::new(false)),
        }
    }
}

impl Future for ImageFuture {
    type Output = Result<HtmlImageElement, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match &self.image {
            Some(image) if image.complete() => {
                let image = self.image.take().unwrap();
                let failed = self.load_failed.get();

                if failed {
                    Poll::Ready(Err(()))
                } else {
                    Poll::Ready(Ok(image))
                }
            }
            Some(image) => {
                let waker = cx.waker().clone();
                let on_load_closure = Closure::wrap(Box::new(move || {
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);
                image.set_onload(Some(on_load_closure.as_ref().unchecked_ref()));
                on_load_closure.forget();

                let waker = cx.waker().clone();
                let failed_flag = self.load_failed.clone();
                let on_error_closure = Closure::wrap(Box::new(move || {
                    failed_flag.set(true);
                    waker.wake_by_ref();
                }) as Box<dyn FnMut()>);
                image.set_onerror(Some(on_error_closure.as_ref().unchecked_ref()));
                on_error_closure.forget();

                Poll::Pending
            }
            _ => Poll::Ready(Err(())),
        }
    }
}

#[macro_export]
macro_rules! tuple_as {
    ($t: expr, $ty: ident) => {{
        let (a, b) = $t;
        (a as $ty, b as $ty)
    }};
}
