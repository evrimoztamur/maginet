#![feature(drain_filter)]

mod app;
mod callbacks;
mod draw;
mod net;

use std::{cell::RefCell, rc::Rc};

use app::App;
use callbacks::*;
use net::{fetch, request_session, request_state, request_turns_since, MessagePool};
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
    let canvas = document()
        .create_element("canvas")?
        .dyn_into::<HtmlCanvasElement>()?;

    let container_element = document().query_selector(&"main").unwrap().unwrap();
    let nav_element = document().query_selector(&"nav").unwrap().unwrap();

    container_element.insert_before(&canvas, Some(&nav_element))?;

    canvas.set_width(512);
    canvas.set_height(512);

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

    atlas.set_src(&"/static/png/atlas.png?v=3");

    let app = App::new();

    let app = Rc::new(RefCell::new(app));

    let message_pool: Rc<RefCell<MessagePool>> = Rc::new(RefCell::new(MessagePool::new()));

    let state_closure = {
        let app = app.clone();

        Closure::<dyn FnMut(JsValue)>::new(move |value| {
            on_state_response(&app, value);
        })
    };

    let message_closure = {
        let message_pool = message_pool.clone();

        Closure::<dyn FnMut(JsValue)>::new(move |value| {
            on_message_response(&message_pool, value);
        })
    };

    let session_closure = {
        let app = app.clone();

        Closure::<dyn FnMut(JsValue)>::new(move |value| {
            on_session_response(&app, value);
        })
    };

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        let app = app.clone();
        let message_pool = message_pool.clone();

        {
            let app = app.borrow();

            if !app.lobby.is_local() && app.session_id.is_none() {
                let _ = fetch(&request_session()).then(&session_closure);
            }
        }

        *g.borrow_mut() = Some(Closure::new(move || {
            let mut app = app.borrow_mut();

            {
                let mut message_pool = message_pool.borrow_mut();

                app.preprocess(&mut message_pool.messages);
                app.update(&message_pool.messages);
                app.draw(&context, &atlas).unwrap();
                app.pointer.swap();

                message_pool.clear();
            }

            if !app.lobby.is_local() && message_pool.borrow().available(app.frame) {
                let mut message_pool = message_pool.borrow_mut();

                if app.lobby.all_ready() {
                    let _ = fetch(&request_turns_since(app.lobby.game.turns())).then(&message_closure);
                } else {
                    let _ = fetch(&request_state()).then(&state_closure);
                }

                message_pool.block(app.frame);
            }

            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    let canvas = Rc::new(canvas);
    let bound: Rc<RefCell<Option<DomRect>>> =
        Rc::new(RefCell::new(Some(canvas.get_bounding_client_rect())));

    {
        let canvas = canvas.clone();
        let bound = bound.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_: JsValue| {
            on_resize(&canvas, &bound);
        });
        window().add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            on_mouse_down(&app, event);
        });
        document()
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            on_mouse_up(&app, event);
        });
        document().add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let bound = bound.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: MouseEvent| {
            on_mouse_move(&app, &bound, event);
        });
        document()
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let bound = bound.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: TouchEvent| {
            on_touch_move(&app, &bound, event);
        });
        document()
            .add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let bound = bound.clone();
        let app = app.clone();

        let closure = Closure::<dyn FnMut(_)>::new(move |event: TouchEvent| {
            on_touch_start(&app, &bound, event);
        });
        document()
            .add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: TouchEvent| {
            on_touch_end(&app, event);
        });
        document()
            .add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let message_pool = message_pool.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: KeyboardEvent| {
            on_key_down(&app, &message_pool, event);
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

    session_closure.forget();

    Ok(())
}
