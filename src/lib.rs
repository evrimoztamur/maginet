use std::cell::{Cell, RefCell};
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

struct Position(i8, i8);

enum Team {
    Red,
    Blue,
}

struct Mana {
    value: u8,
    max: u8,
}

impl Mana {
    fn with_max(max_mana: u8) -> Mana {
        Mana {
            value: max_mana,
            max: max_mana,
        }
    }
}

enum SpellSort {
    Damage(u8),
}

struct Spell {
    cost: u8,
    sort: SpellSort,
    pattern: Vec<Position>,
}

impl Spell {
    fn new(cost: u8, sort: SpellSort, pattern: Vec<Position>) -> Spell {
        Spell {
            cost,
            sort,
            pattern,
        }
    }
    fn default_missile() -> Spell {
        Self::new(
            1,
            SpellSort::Damage(1),
            vec![
                Position(-2, 0),
                Position(-1, 0),
                Position(1, 0),
                Position(2, 0),
                Position(0, -2),
                Position(0, -1),
                Position(0, 1),
                Position(0, 2),
            ],
        )
    }
}

struct Mage {
    position: Position,
    mana: Mana,
    team: Team,
    spells: Vec<Spell>,
}

impl Mage {
    fn new(position: Position, max_mana: u8, team: Team, spells: Vec<Spell>) -> Mage {
        Mage {
            position,
            team,
            mana: Mana::with_max(max_mana),
            spells,
        }
    }
}

struct Board {
    width: usize,
    height: usize,
}

impl Board {
    fn new(width: usize, height: usize) -> Result<Board, &'static str> {
        match (width, height) {
            (width, height) if width >= 4 && width <= 8 && height >= 4 && height <= 8 => {
                Ok(Board { width, height })
            }
            _ => Err("board size does not conform to limits"),
        }
    }
}

struct Game {
    board: Board,
    mages: Vec<Mage>,
    turn: u16,
}

impl Game {
    fn new(
        board_width: usize,
        board_height: usize,
        mage_count: usize,
    ) -> Result<Game, &'static str> {
        if mage_count >= (board_width - 1) * (board_height - 1) {
            Err("game contains too many mages for board")
        } else {
            let board = Board::new(board_width, board_height)?;
            let mut mages = Vec::with_capacity(mage_count * 2);

            let mut x_offset = ((board_width - mage_count) / 2) as i8;

            for i in 0..mage_count {
                mages.push(Mage::new(
                    Position(x_offset + i as i8, 0),
                    8,
                    Team::Blue,
                    vec![Spell::default_missile()],
                ));

                mages.push(Mage::new(
                    Position(x_offset + i as i8, board_height as i8 - 1),
                    8,
                    Team::Red,
                    vec![Spell::default_missile()],
                ));
            }

            let turn = 0;

            Ok(Game { board, mages, turn })
        }
    }
}

struct Pointer {
    previous: Option<Box<Pointer>>,
    location: (i32, i32),
    lmb: bool,
    rmb: bool,
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    web_sys::window()
        .expect("no global `window` exists")
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

#[wasm_bindgen(start)]
fn start() -> Result<(), JsValue> {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document
        .create_element("canvas")?
        .dyn_into::<web_sys::HtmlCanvasElement>()?;

    document.body().unwrap().append_child(&canvas)?;
    canvas.set_width(512);
    canvas.set_height(640);

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

    atlas.set_src(&"atlas.png");

    let game = Game::new(8, 8, 4).unwrap();

    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    fn draw(
        context: &CanvasRenderingContext2d,
        atlas: &HtmlImageElement,
        game: &Game,
    ) -> Result<(), JsValue> {
        context.save();

        context.scale(2.0, 2.0)?;

        // DRAW background layer (board + UI block)
        context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
            atlas, 64.0, 0.0, 8.0, 10.0, 0.0, 0.0, 256.0, 320.0,
        )?;

        // DRAW mages
        for mage in &game.mages {
            match mage.team {
                Team::Red => context
                    .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        atlas,
                        0.0,
                        0.0,
                        32.0,
                        32.0,
                        mage.position.0 as f64 * 32.0,
                        mage.position.1 as f64 * 32.0,
                        32.0,
                        32.0,
                    )?,
                Team::Blue => context
                    .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                        atlas,
                        32.0,
                        0.0,
                        32.0,
                        32.0,
                        mage.position.0 as f64 * 32.0,
                        mage.position.1 as f64 * 32.0,
                        32.0,
                        32.0,
                    )?,
            }
        }

        context.restore();

        Ok(())
    }

    *g.borrow_mut() = Some(Closure::new(move || {
        draw(&context, &atlas, &game).unwrap();

        request_animation_frame(f.borrow().as_ref().unwrap());
    }));

    request_animation_frame(g.borrow().as_ref().unwrap());

    // let context = Rc::new(context);
    // let pressed = Rc::new(Cell::new(false));

    // {
    //     let context = context.clone();
    //     let pressed = pressed.clone();
    //     let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
    //         context.begin_path();
    //         context.move_to(event.offset_x() as f64 + 0.5, event.offset_y() as f64 + 0.5);
    //         pressed.set(true);
    //     });
    //     canvas.add_event_listener_with_callback("mousedown", closure.as_ref().unchecked_ref())?;
    //     closure.forget();
    // }

    // {
    //     let context = context.clone();
    //     let pressed = pressed.clone();
    //     let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
    //         if pressed.get() {
    //             context.line_to(event.offset_x() as f64 + 0.5, event.offset_y() as f64 + 0.5);
    //             context.stroke();
    //             context.begin_path();
    //             context.move_to(event.offset_x() as f64 + 0.5, event.offset_y() as f64 + 0.5);
    //         }
    //     });
    //     canvas.add_event_listener_with_callback("mousemove", closure.as_ref().unchecked_ref())?;
    //     closure.forget();
    // }

    // {
    //     let context = context.clone();
    //     let pressed = pressed.clone();
    //     let closure = Closure::<dyn FnMut(_)>::new(move |event: web_sys::MouseEvent| {
    //         pressed.set(false);
    //         context.line_to(event.offset_x() as f64 + 0.5, event.offset_y() as f64 + 0.5);
    //         context.stroke();
    //     });
    //     canvas.add_event_listener_with_callback("mouseup", closure.as_ref().unchecked_ref())?;
    //     closure.forget();
    // }

    Ok(())
}
