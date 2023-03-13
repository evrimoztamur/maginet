#![feature(drain_filter)]

mod app;
mod draw;
mod net;

use std::{cell::RefCell, rc::Rc};

use app::{App, BOARD_OFFSET, BOARD_SCALE};
use net::{fetch, request_ready, request_state, request_turns_since, MessagePool};
use shared::Message;
use wasm_bindgen::{prelude::*, JsCast};

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let canvas = document()
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    let container_element = document().query_selector(&"main").unwrap().unwrap();
    let nav_element = document().query_selector(&"nav").unwrap().unwrap();
    container_element.insert_before(&canvas, Some(&nav_element))?;

    canvas.set_width(512);
    canvas.set_height(544);

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    context.set_image_smoothing_enabled(false);

    let atlas = document()
        .create_element("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();

    atlas.set_src(&"/img/atlas.png");

    let app = App::new();
    let app = Rc::new(RefCell::new(app));

    let state_request = request_state();
    let ready_request = request_ready();

    let state_closure = {
        let app = app.clone();

        Closure::<dyn FnMut(JsValue)>::new(move |value| {
            let mut app = app.borrow_mut();
            let message: Message = serde_wasm_bindgen::from_value(value).unwrap();

            match message {
                Message::Lobby(lobby) => {
                    app.lobby = lobby;
                }
                _ => (),
            }

            if !app.in_lobby() {
                fetch(&ready_request);
            }
        })
    };

    let message_pool: Rc<RefCell<MessagePool>> = Rc::new(RefCell::new(MessagePool::new()));

    let message_closure = {
        let message_pool = message_pool.clone();

        Closure::<dyn FnMut(JsValue)>::new(move |value| {
            let mut message_pool = message_pool.borrow_mut();
            let messages: Vec<Message> = serde_wasm_bindgen::from_value(value).unwrap();
            message_pool.append(messages);
        })
    };

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        let app = app.clone();
        let message_pool = message_pool.clone();

        *g.borrow_mut() = Some(Closure::new(move || {
            let mut app = app.borrow_mut();

            {
                let message_pool = message_pool.borrow();

                app.update(&message_pool.messages);
                app.draw(&context, &atlas).unwrap();
                app.pointer.swap();
            }

            if message_pool.borrow().available(app.frame) {
                let mut message_pool = message_pool.borrow_mut();

                if app.lobby.all_ready() {
                    fetch(&request_turns_since(app.lobby.game.turns())).then(&message_closure);
                } else {
                    fetch(&state_request).then(&state_closure);
                }

                message_pool.clear();
                message_pool.block(app.frame);
            }

            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    {
        let app = app.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let mut app = app.borrow_mut();

            match event.button() {
                0 | 2 => app.pointer.button = true,
                _ => (),
            }
        });
        document()
            .add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let mut app = app.borrow_mut();

            match event.button() {
                0 | 2 => app.pointer.button = false,
                _ => (),
            }
        });
        document().add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    let canvas = Rc::new(canvas);
    let bound: Rc<RefCell<Option<web_sys::DomRect>>> =
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
        let bound = bound.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let bound = bound.borrow();

            if let Some(bound) = bound.as_ref() {
                let mut app = app.borrow_mut();

                let x = event.client_x() - bound.left() as i32;
                let y = event.client_y() - bound.top() as i32;
                let x = (x as f64 * (512.0 / bound.width())) as i32;
                let y = (y as f64 * (512.0 / bound.width())) as i32;

                app.pointer.location = (x / 2, y / 2);
            }

            event.prevent_default();
        });
        document()
            .add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let bound = bound.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::TouchEvent| {
            let mut app = app.borrow_mut();
            let bound = bound.borrow();

            if let Some(bound) = bound.as_ref() {
                if let Some(touch) = event.target_touches().item(0) {
                    let x = touch.client_x() - bound.left() as i32;
                    let y = touch.client_y() - bound.top() as i32;

                    let x = (x as f64 * (512.0 / bound.width())) as i32;
                    let y = (y as f64 * (512.0 / bound.width())) as i32;

                    app.pointer.location = (x as i32 / 2, y as i32 / 2);
                }
            }
            event.prevent_default();
        });
        document()
            .add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let bound = bound.clone();
        let app = app.clone();

        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::TouchEvent| {
            let bound = bound.borrow();

            if let Some(bound) = bound.as_ref() {
                if let Some(touch) = event.target_touches().item(0) {
                    let mut app = app.borrow_mut();

                    let x = touch.client_x() - bound.left() as i32;
                    let y = touch.client_y() - bound.top() as i32;

                    let x = (x as f64 * (512.0 / bound.width())) as i32;
                    let y = (y as f64 * (512.0 / bound.width())) as i32;

                    let pointer_location = (x as i32 / 2, y as i32 / 2);

                    if (pointer_location.0 - app.pointer.location.0).abs() < 16
                        && (pointer_location.1 - app.pointer.location.1).abs() < 16
                    {
                        app.pointer.button = true;
                    } else if let Some(selected_tile) = app.lobby.game.location_as_position(
                        pointer_location,
                        BOARD_OFFSET,
                        BOARD_SCALE,
                    ) {
                        if let Some(_) = app.lobby.game.live_occupant(&selected_tile) {
                            app.pointer.button = true;
                        }
                    }

                    app.pointer.location = (x as i32 / 2, y as i32 / 2);
                }
            }
            event.prevent_default();
        });
        document()
            .add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let app = app.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::TouchEvent| {
            let mut app = app.borrow_mut();

            app.pointer.button = false;
            event.prevent_default();
        });
        document()
            .add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            event.prevent_default();
        });
        document()
            .add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}
