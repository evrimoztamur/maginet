#![feature(drain_filter)]

mod app;
mod draw;
mod net;

use std::{cell::RefCell, rc::Rc};

use app::App;
use net::{fetch, request_session};
use wasm_bindgen::{prelude::*, JsCast};
use web_sys::{
    CanvasRenderingContext2d, Document, DomRect, HtmlCanvasElement, HtmlImageElement,
    KeyboardEvent, MouseEvent, TouchEvent, Window,
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

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let canvas = document()
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    let container_element = document().query_selector(&"main").unwrap().unwrap();
    let nav_element = document().query_selector(&"nav").unwrap().unwrap();

    container_element.insert_before(&canvas, Some(&nav_element))?;

    canvas.set_width(ELEMENT_WIDTH);
    canvas.set_height(ELEMENT_HEIGHT);

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<CanvasRenderingContext2d>()?;

    context.set_image_smoothing_enabled(false);

    let atlas = document()
        .create_element("img")
        .unwrap()
        .dyn_into::<HtmlImageElement>()
        .unwrap();

    atlas.set_src(&"/static/png/atlas.png?v=4");

    let app = App::new();

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

        {
            let app = app.borrow();

            if app.session_id().is_none() {
                let _ = fetch(&request_session()).then(&session_closure);
            }
        }

        *g.borrow_mut() = Some(Closure::new(move || {
            let mut app = app.borrow_mut();

            {
                app.tick();
                app.draw(&context, &atlas).unwrap();
            }

            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    session_closure.forget();

    let canvas = Rc::new(canvas);
    let bound: Rc<RefCell<Option<DomRect>>> =
        Rc::new(RefCell::new(Some(canvas.get_bounding_client_rect())));

    {
        let canvas = canvas.clone();
        let bound = bound.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_: JsValue| {
            bound.replace(Some(canvas.get_bounding_client_rect()));
        });
        window().add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            let mut app = app.borrow_mut();
            app.on_mouse_down(event);
        });
        document()
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            let mut app = app.borrow_mut();
            app.on_mouse_up(event);
        });
        document().add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let bound = bound.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            let mut app = app.borrow_mut();
            if let Some(bound) = bound.borrow().as_deref() {
                app.on_mouse_move(bound, event);
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
            let mut app = app.borrow_mut();
            app.on_touch_end(event);
        });
        document()
            .add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
            let mut app = app.borrow_mut();
            app.on_key_down(event);
        });
        document().add_event_listener_with_callback("keydown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            event.prevent_default();
        });
        document()
            .add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
