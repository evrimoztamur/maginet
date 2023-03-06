#![feature(drain_filter)]

use draw::*;
use futures::TryFutureExt;
use shared::{Game, Team};
use shared::{Mage, Message};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use wasm_bindgen_futures::{future_to_promise, JsFuture};
use web_sys::{CanvasRenderingContext2d, HtmlImageElement, Request, RequestInit, Response};

const BOARD_OFFSET: (i32, i32) = (8, 8);
const BOARD_OFFSET_F64: (f64, f64) = (BOARD_OFFSET.0 as f64, BOARD_OFFSET.1 as f64);
const BOARD_SCALE: (i32, i32) = (30, 30);
const BOARD_SCALE_F64: (f64, f64) = (BOARD_SCALE.0 as f64, BOARD_SCALE.1 as f64);
mod draw;
enum ParticleSort {
    Missile,
    Overdrive,
}
struct Particle(f64, f64, f64, f64, u64, ParticleSort);

impl Particle {
    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        frame: u64,
    ) -> Result<(), JsValue> {
        context.save();
        context.translate(
            ((self.0 + 0.5) * BOARD_SCALE_F64.0).floor(),
            ((self.1 + 0.5) * BOARD_SCALE_F64.1).floor(),
        )?;

        let cycle = frame + (self.0 * 16.0) as u64 + (self.1 * 16.0) as u64;

        context.rotate((cycle / 10) as f64 * std::f64::consts::PI / 2.0)?;
        // context.rotate(frame as f64 * 0.1)?;
        draw_sprite(
            context,
            atlas,
            64.0 + {
                let t = cycle % 20;
                if t > 15 {
                    16.0
                } else if t > 10 {
                    8.0
                } else {
                    0.0
                }
            } + {
                match self.5 {
                    ParticleSort::Missile => 0.0,
                    ParticleSort::Overdrive => 24.0,
                }
            },
            56.0,
            8.0,
            8.0,
            -4.0,
            -4.0,
        )?;
        context.restore();
        self.0 += self.2;
        self.1 += self.3;
        self.2 -= self.2 * 0.1;
        self.3 -= self.3 * 0.1;
        self.4 = self.4.saturating_sub(1);
        Ok(())
    }

    fn is_alive(&self) -> bool {
        self.4 > 1
        // true
    }
}

trait Drawable {
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        index: usize,
        frame: u64,
        team: Team,
    ) -> Result<(), JsValue>;
}

impl Drawable for Mage {
    fn draw(
        &self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        index: usize,
        frame: u64,
        team: Team,
    ) -> Result<(), JsValue> {
        let bounce = (if self.team == team && self.is_alive() {
            2 - ((frame as i64 / 6 + index as i64 / 2) % 4 - 2).abs()
        } else {
            0
        }) as f64;

        let sleeping_offset = if self.is_alive() { 0.0 } else { 64.0 };

        match self.team {
            Team::Red => context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    atlas,
                    32.0 * (index / 2) as f64,
                    64.0 + sleeping_offset,
                    32.0,
                    32.0,
                    0.0,
                    0.0 + bounce,
                    32.0,
                    32.0,
                )?,
            Team::Blue => context
                .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                    atlas,
                    32.0 * (index / 2) as f64,
                    96.0 + sleeping_offset,
                    32.0,
                    32.0,
                    0.0,
                    0.0 + bounce,
                    32.0,
                    32.0,
                )?,
        }

        Ok(())
    }
}

#[derive(Clone)]
struct Pointer {
    previous: Option<Box<Pointer>>,
    location: (i32, i32),
    lmb: bool,
    rmb: bool,
}

impl Pointer {
    fn clicked(&self) -> bool {
        match &self.previous {
            Some(pointer) => self.lmb && !pointer.lmb,
            None => self.lmb,
        }
    }

    fn alt_clicked(&self) -> bool {
        match &self.previous {
            Some(pointer) => self.rmb && !pointer.rmb,
            None => self.rmb,
        }
    }
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

struct App {
    game: Game,
    particles: Vec<Particle>,
    frame: u64,
}

impl App {
    fn new() -> App {
        App {
            game: Game::new(8, 8, 4).unwrap(),
            particles: Vec::new(),
            frame: 0,
        }
    }

    fn draw(
        &mut self,
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        pointer: &Pointer,
    ) -> Result<(), JsValue> {
        context.clear_rect(0.0, 0.0, 512.0, 544.0);
        context.save();

        context.scale(2.0, 2.0)?;

        // DRAW background layer (board + UI block)

        // DRAW board

        {
            context.save();

            context.translate(BOARD_OFFSET_F64.0, BOARD_OFFSET_F64.1)?;

            draw_sprite_scaled(context, atlas, 64.0, 0.0, 8.0, 8.0, 0.0, 0.0, 240.0, 240.0)?;

            // DRAW particles

            for particle in self.particles.iter_mut() {
                particle.draw(context, atlas, self.frame)?;
            }

            self.particles.drain_filter(|particle| !particle.is_alive());

            {
                // DRAW mages
                for mage in self.game.iter_mages() {
                    context.save();

                    context.translate(
                        -1.0 + mage.position.0 as f64 * BOARD_SCALE_F64.0,
                        -1.0 + mage.position.1 as f64 * BOARD_SCALE_F64.1,
                    )?;

                    mage.draw(context, atlas, mage.index, self.frame, self.game.turn_for())?;

                    if mage.mana.is_overdriven() && mage.is_alive() {
                        for _ in 0..(self.frame / 2 % 2) {
                            let d = js_sys::Math::random() * -std::f64::consts::PI * 0.9;
                            let v = (js_sys::Math::random() + js_sys::Math::random()) * 0.05;
                            self.particles.push(Particle(
                                mage.position.0 as f64 + d.cos() * 0.4,
                                mage.position.1 as f64 - 0.15 + d.sin() * 0.4,
                                d.cos() * v,
                                d.sin() * v,
                                (js_sys::Math::random() * 50.0) as u64,
                                ParticleSort::Overdrive,
                            ));
                        }
                    }

                    if self.game.is_mage_active(mage) {
                        draw_sprite(
                            context,
                            atlas,
                            72.0,
                            0.0,
                            8.0,
                            5.0,
                            12.0,
                            -1.0 - (self.frame / 6 % 6) as f64,
                        )?;
                    }

                    context.restore();
                }
            }

            {
                // DRAW markers
                context.save();

                if let Some(mage) = self.game.get_active_mage() {
                    let available_moves = mage.available_moves(&self.game);
                    for (position, overdrive) in &available_moves {
                        draw_sprite(
                            context,
                            atlas,
                            if *overdrive { 16.0 } else { 0.0 },
                            32.0,
                            16.0,
                            16.0,
                            position.0 as f64 * BOARD_SCALE_F64.0 + BOARD_OFFSET_F64.0 - 1.0,
                            position.1 as f64 * BOARD_SCALE_F64.1 + BOARD_OFFSET_F64.1 - 1.0,
                        )?;
                    }

                    if let Some(selected_tile) =
                        self.game
                            .location_as_position(pointer.location, BOARD_OFFSET, BOARD_SCALE)
                    {
                        if available_moves
                            .iter()
                            .find(|(position, _)| position == &selected_tile)
                            .is_some()
                        {
                            for (enemy_occupied, position) in
                                &self.game.targets(mage, selected_tile, 0)
                            {
                                if *enemy_occupied {
                                    draw_crosshair(
                                        context,
                                        atlas,
                                        position,
                                        (64.0, 32.0),
                                        self.frame,
                                    )?;
                                } else {
                                    draw_crosshair(context, atlas, position, (48.0, 32.0), 0)?;
                                }
                            }
                        }
                    }
                }

                context.restore();

                if let Some(selected_tile) =
                    self.game
                        .location_as_position(pointer.location, BOARD_OFFSET, BOARD_SCALE)
                {
                    draw_crosshair(context, atlas, &selected_tile, (32.0, 32.0), self.frame)?;
                }
            }

            context.restore();
        }

        // DRAW UI block
        {
            context.save();

            context.translate(8.0, 8.0)?;

            context.translate(0.0, 248.0)?;
            {
                // DRAW active mage
                if let Some(active_mage) = self.game.get_active_mage() {
                    for i in 0..active_mage.mana.max {
                        if i < *active_mage.mana {
                            draw_sprite(context, atlas, 80.0, 0.0, 8.0, 8.0, i as f64 * 10.0, 0.0)?;
                        } else {
                            draw_sprite(context, atlas, 88.0, 0.0, 8.0, 8.0, i as f64 * 10.0, 0.0)?;
                        }
                    }
                }
            }

            context.restore();
        }

        // DRAW cursor
        draw_sprite(
            context,
            atlas,
            64.0,
            8.0,
            16.0,
            16.0,
            pointer.location.0 as f64 - 5.0,
            pointer.location.1 as f64 - 1.0,
        )?;

        if let Some(selected_tile) =
            self.game
                .location_as_position(pointer.location, BOARD_OFFSET, BOARD_SCALE)
        {
            if let Some(occupant) = self.game.live_occupant(&selected_tile) {
                draw_tooltip(
                    context,
                    atlas,
                    (pointer.location.0, pointer.location.1 + 16),
                    24,
                )?;

                draw_digits(
                    context,
                    atlas,
                    (pointer.location.0 + 2, pointer.location.1 + 16),
                    *occupant.mana as usize,
                )?;

                draw_sprite(
                    context,
                    atlas,
                    80.0,
                    12.0,
                    4.0,
                    4.0,
                    pointer.location.0 as f64 + 11.0,
                    pointer.location.1 as f64 + 18.0,
                )?;

                draw_digits(
                    context,
                    atlas,
                    (pointer.location.0 + 17, pointer.location.1 + 16),
                    occupant.mana.max as usize,
                )?;
            }
        }

        context.restore();

        self.frame += 1;

        Ok(())
    }
}

trait Updatable {
    fn update(&mut self, pointer: &Pointer, messages: &Vec<Message>);
}

impl Updatable for Game {
    fn update(&mut self, pointer: &Pointer, messages: &Vec<Message>) {
        if pointer.clicked() {
            if let Some(selected_tile) =
                self.location_as_position(pointer.location, BOARD_OFFSET, BOARD_SCALE)
            {
                if let Some(active_mage) = self.get_active_mage() {
                    let available_moves = active_mage.available_moves(self);
                    let potential_move = available_moves
                        .iter()
                        .find(|(position, _)| *position == selected_tile);

                    if let Some((position, _)) = potential_move {
                        self.get_active_mage_mut().unwrap().position = *position;
                        self.attack();
                        self.end_turn();
                    } else {
                        self.select_mage_at(&selected_tile);
                    }
                } else {
                    self.select_mage_at(&selected_tile);
                }
            }
        } else if pointer.alt_clicked() {
            // if self.attack() > 0 {
            //     self.end_turn();
            // }
        }
    }
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();

    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(512);
    canvas.set_height(544);

    let context = canvas
        .get_context("2d")?
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    context.set_image_smoothing_enabled(false);

    let atlas = document
        .create_element("img")
        .unwrap()
        .dyn_into::<web_sys::HtmlImageElement>()
        .unwrap();

    atlas.set_src(&"img/atlas.png");

    let pointer = Rc::new(RefCell::new(Pointer {
        previous: None,
        location: (0, 0),
        lmb: false,
        rmb: false,
    }));

    let request = {
        let mut opts = RequestInit::new();
        opts.method("GET");

        let url = format!("test");

        Request::new_with_str_and_init(&url, &opts).unwrap()
    };

    let message_pool: Rc<RefCell<MessagePool>> = Rc::new(RefCell::new(MessagePool::new()));

    let message_closure = {
        let message_pool = message_pool.clone();

        Closure::<dyn FnMut(JsValue)>::new(move |value| {
            let mut message_pool = message_pool.borrow_mut();
            let message: Message = serde_wasm_bindgen::from_value(value).unwrap();
            message_pool.messages.push(message);

            message_pool.polling = false;
        })
    };

    let app = App::new();
    let app = Rc::new(RefCell::new(app));

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    {
        let pointer = pointer.clone();
        let app = app.clone();

        *g.borrow_mut() = Some(Closure::new(move || {
            let message_pool = message_pool.clone();
            let mut app = app.borrow_mut();

            {
                let pointer = pointer.borrow();
                let message_pool = message_pool.borrow();

                app.game.update(&pointer, &message_pool.messages);
                app.draw(&context, &atlas, &pointer).unwrap();
            }

            message_pool.borrow_mut().messages.clear();

            {
                let mut pointer = pointer.borrow_mut();
                pointer.previous.take();
                pointer.previous = Some(Box::new(pointer.clone()));
            }

            if !message_pool.borrow().polling {
            //     let resp_value =
            //         JsFuture::from(web_sys::window().unwrap().fetch_with_request(&request))
            //             .and_then(|v| {
            //                 assert!(v.is_instance_of::<Response>());
            //                 let resp: Response = v.dyn_into().unwrap();
            //                 JsFuture::from(resp.json().unwrap())
            //             });

            //     let promise = future_to_promise(resp_value);
            //     promise.then(&message_closure);
            //     // promise.catch(&Closure::<dyn FnMut(_)>::new(|value: JsValue| {
            //     //     web_sys::console::log_1(&value);
            //     // }));

                message_pool.borrow_mut().polling = true;
            }

            request_animation_frame(f.borrow().as_ref().unwrap());
        }));

        request_animation_frame(g.borrow().as_ref().unwrap());
    }

    {
        let pointer = pointer.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let mut pointer = pointer.borrow_mut();

            match event.button() {
                0 => pointer.lmb = true,
                2 => pointer.rmb = true,
                _ => (),
            }
        });
        document.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let pointer = pointer.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let mut pointer = pointer.borrow_mut();

            match event.button() {
                0 => pointer.lmb = false,
                2 => pointer.rmb = false,
                _ => (),
            }
        });
        document.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
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
        web_sys::window()
            .unwrap()
            .add_event_listener_with_callback("resize", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let pointer = pointer.clone();
        let bound = bound.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            let bound = bound.borrow();

            if let Some(bound) = bound.as_ref() {
                let mut pointer = pointer.borrow_mut();

                let x = event.client_x() - bound.left() as i32;
                let y = event.client_y() - bound.top() as i32;
                let x = (x as f64 * (512.0 / bound.width())) as i32;
                let y = (y as f64 * (512.0 / bound.width())) as i32;

                pointer.location = (x / 2, y / 2);
            }

            event.prevent_default();
        });
        document.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let pointer = pointer.clone();
        let bound = bound.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::TouchEvent| {
            let mut pointer = pointer.borrow_mut();
            let bound = bound.borrow();

            if let Some(bound) = bound.as_ref() {
                if let Some(touch) = event.target_touches().item(0) {
                    let x = touch.client_x() - bound.left() as i32;
                    let y = touch.client_y() - bound.top() as i32;

                    let x = (x as f64 * (512.0 / bound.width())) as i32;
                    let y = (y as f64 * (512.0 / bound.width())) as i32;

                    pointer.location = (x as i32 / 2, y as i32 / 2);
                }
            }
        });
        document.add_event_listener_with_callback("touchmove", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let pointer = pointer.clone();
        let bound = bound.clone();
        let app = app.clone();

        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::TouchEvent| {
            let bound = bound.borrow();

            if let Some(bound) = bound.as_ref() {
                if let Some(touch) = event.target_touches().item(0) {
                    let mut pointer = pointer.borrow_mut();
                    let app = app.borrow();

                    let x = touch.client_x() - bound.left() as i32;
                    let y = touch.client_y() - bound.top() as i32;

                    let x = (x as f64 * (512.0 / bound.width())) as i32;
                    let y = (y as f64 * (512.0 / bound.width())) as i32;

                    let pointer_location = (x as i32 / 2, y as i32 / 2);

                    if (pointer_location.0 - pointer.location.0).abs() < 16
                        && (pointer_location.1 - pointer.location.1).abs() < 16
                    {
                        pointer.lmb = true;
                    } else if let Some(selected_tile) =
                        app.game
                            .location_as_position(pointer_location, BOARD_OFFSET, BOARD_SCALE)
                    {
                        if let Some(_) = app.game.live_occupant(&selected_tile) {
                            pointer.lmb = true;
                        }
                    }

                    pointer.location = (x as i32 / 2, y as i32 / 2);
                }
            }
        });
        document
            .add_event_listener_with_callback("touchstart", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    {
        let pointer = pointer.clone();
        let closure = Closure::<dyn FnMut(_)>::new(move |_: web_sys::TouchEvent| {
            let mut pointer = pointer.borrow_mut();

            pointer.lmb = false;
        });
        document.add_event_listener_with_callback("touchend", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }
    {
        let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
            event.prevent_default();
        });
        document
            .add_event_listener_with_callback("contextmenu", closure.as_ref().unchecked_ref())?;
        closure.forget();
    }

    Ok(())
}

struct MessagePool {
    messages: Vec<Message>,
    polling: bool,
}

impl MessagePool {
    fn new() -> MessagePool {
        MessagePool {
            messages: Vec::new(),
            polling: false,
        }
    }
}
