use js_sys::Math::random;
use shared::{Mage, Position, Team};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::{
    app::{CanvasSettings, Particle, ParticleSort, BOARD_SCALE},
    init_canvas, tuple_as,
};

pub fn rotation_from_position(position: Position) -> i8 {
    let (sx, sy) = (position.0.signum(), position.1.signum());

    match (sx, sy) {
        (1, 0) => 0,
        (1, 1) => 1,
        (0, 1) => 2,
        (-1, 1) => 3,
        (-1, 0) => 4,
        (-1, -1) => 5,
        (0, -1) => 6,
        (1, -1) => 7,
        _ => 0,
    }
}

pub fn draw_sprite(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    sx: f64,
    sy: f64,
    sw: f64,
    sh: f64,
    dx: f64,
    dy: f64,
) -> Result<(), JsValue> {
    context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        atlas, sx, sy, sw, sh, dx, dy, sw, sh,
    )?;

    Ok(())
}

fn kerning(char: char) -> (isize, isize) {
    match char {
        'i' => (-2, -1),
        'l' => (-2, -2),
        't' => (-1, 0),
        'f' => (0, -1),
        _ => (0, 0),
    }
}

pub fn text_length(text: &String) -> isize {
    text.chars()
        .map(|char| {
            let kern = kerning(char);
            (kern.0 + kern.1) + 8
        })
        .sum()
}

pub fn draw_text(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    dx: f64,
    dy: f64,
    text: &String,
) -> Result<(), JsValue> {
    let mut kerning_acc: isize = 0;

    for (i, char) in text.chars().enumerate() {
        let kern = kerning(char);
        kerning_acc += kern.0;

        draw_sprite(
            context,
            atlas,
            ((char as u8 % 32) * 8) as f64,
            216.0 + ((char as u8 / 32) * 8) as f64,
            8.0,
            8.0,
            dx + (i * 8) as f64 + kerning_acc as f64,
            dy + 1.0,
        )
        .unwrap();

        kerning_acc += kern.1;
    }

    Ok(())
}

pub fn draw_crosshair(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    position: &Position,
    offset: (f64, f64),
    frame: u64,
) -> Result<(), JsValue> {
    let board_scale = tuple_as!(BOARD_SCALE, f64);

    draw_sprite(
        context,
        atlas,
        offset.0,
        offset.1,
        8.0,
        8.0,
        position.0 as f64 * board_scale.0 + (frame / 6 % 4) as f64,
        position.1 as f64 * board_scale.1 + (frame / 6 % 4) as f64,
    )?;

    draw_sprite(
        context,
        atlas,
        offset.0 + 8.0,
        offset.1,
        8.0,
        8.0,
        position.0 as f64 * board_scale.0 - (frame / 6 % 4) as f64 + board_scale.0 - 8.0,
        position.1 as f64 * board_scale.1 + (frame / 6 % 4) as f64,
    )?;

    draw_sprite(
        context,
        atlas,
        offset.0,
        offset.1 + 8.0,
        8.0,
        8.0,
        position.0 as f64 * board_scale.0 + (frame / 6 % 4) as f64,
        position.1 as f64 * board_scale.1 - (frame / 6 % 4) as f64 + board_scale.1 - 8.0,
    )?;

    draw_sprite(
        context,
        atlas,
        offset.0 + 8.0,
        offset.1 + 8.0,
        8.0,
        8.0,
        position.0 as f64 * board_scale.0 - (frame / 6 % 4) as f64 + board_scale.0 - 8.0,
        position.1 as f64 * board_scale.1 - (frame / 6 % 4) as f64 + board_scale.1 - 8.0,
    )?;

    Ok(())
}

pub fn draw_mage(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    mage: &Mage,
    frame: u64,
    team: Team,
    game_started: bool,
    game_finished: bool,
    show_mana: bool,
) -> Result<(), JsValue> {
    let bounce = (if mage.is_alive() && (mage.team == team && game_started || game_finished) {
        -((frame as i64 / 6 + mage.index as i64 / 2) % 4 - 2).abs()
    } else {
        0
    }) as f64;

    let sleeping_offset = if mage.is_alive() && game_started {
        0.0
    } else {
        64.0
    };

    context.save();

    if mage.is_alive() {
        draw_sprite(context, atlas, 0.0, 192.0, 32.0, 32.0, -16.0, -20.0)?;

        if game_finished {
            context.translate(
                0.0,
                ((frame as i64 % 80 - 40).max(0) - 20).abs() as f64 - 20.0,
            )?;
            context.rotate(
                ((frame as i64 % 80 - 35).max(0) / 5) as f64 * std::f64::consts::PI / 2.0,
            )?;
        }
    } else {
        context.translate(0.0, 4.0)?;

        draw_sprite(context, atlas, 32.0, 192.0, 32.0, 32.0, -16.0, -20.0)?;
    }

    let sprite_x = match mage.sort {
        shared::MageSort::Diamond => 0.0,
        shared::MageSort::Spike => 32.0,
        shared::MageSort::Knight => 64.0,
        shared::MageSort::Cross => 96.0,
        shared::MageSort::Plus => 0.0,
    };

    match mage.team {
        Team::Red => context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                atlas,
                sprite_x,
                64.0 + sleeping_offset,
                32.0,
                32.0,
                -16.0,
                -20.0 + bounce,
                32.0,
                32.0,
            )?,
        Team::Blue => {
            context.scale(-1.0, 1.0)?;
            context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                atlas,
                sprite_x,
                96.0 + sleeping_offset,
                32.0,
                32.0,
                -16.0,
                -20.0 + bounce,
                32.0,
                32.0,
            )?
        }
    }

    context.restore();

    if show_mana {
        draw_mana(context, atlas, mage)?;
    }

    Ok(())
}

pub fn draw_mana(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    mage: &Mage,
) -> Result<(), JsValue> {
    for i in 0..mage.mana.0 {
        draw_sprite(
            context,
            atlas,
            80.0,
            12.0,
            4.0,
            4.0,
            i as f64 * 6.0 - mage.mana.0 as f64 * 3.0 + 1.0,
            10.0,
        )?;
    }

    Ok(())
}

pub fn draw_spell_pattern(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    mage: &Mage,
) -> Result<(), JsValue> {
    for x in 0..5 {
        for y in 0..5 {
            if x == 2 && y == 2 {
                draw_sprite(
                    context,
                    atlas,
                    104.0,
                    16.0,
                    8.0,
                    8.0,
                    x as f64 * 8.0,
                    y as f64 * 8.0,
                )?;
            } else {
                draw_sprite(
                    context,
                    atlas,
                    96.0,
                    16.0,
                    8.0,
                    8.0,
                    x as f64 * 8.0,
                    y as f64 * 8.0,
                )?;
            }
        }
    }

    for position in &mage.spell.pattern {
        draw_sprite(
            context,
            atlas,
            96.0,
            24.0,
            8.0,
            8.0,
            position.0 as f64 * 8.0 + 16.0,
            position.1 as f64 * 8.0 + 16.0,
        )?;
    }

    Ok(())
}

pub fn draw_particle(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    particle: &Particle,
    frame: u64,
) -> Result<(), JsValue> {
    let board_scale = tuple_as!(BOARD_SCALE, f64);

    context.save();
    context.translate(
        ((particle.position.0 + 0.5) * board_scale.0).floor(),
        ((particle.position.1 + 0.5) * board_scale.1).floor(),
    )?;

    let spin = particle.lifetime;
    let cycle =
        frame + (particle.position.0 * 16.0) as u64 + (particle.position.1 * 16.0) as u64 + spin;

    context.rotate((spin / 5) as f64 * std::f64::consts::PI / 2.0)?;
    // context.rotate(frame as f64 * 0.1)?;
    draw_sprite(
        context,
        atlas,
        {
            let t = cycle % 24;
            if t > 16 {
                16.0
            } else if t > 8 {
                8.0
            } else {
                0.0
            }
        } + {
            match particle.sort {
                ParticleSort::Missile => 0.0,
                ParticleSort::Diagonals => 24.0,
            }
        },
        56.0,
        8.0,
        8.0,
        -4.0,
        -4.0,
    )?;
    context.restore();

    Ok(())
}

fn quadrant_to_xy(corner: u8) -> (u8, u8) {
    match corner {
        0 => (0, 0),
        1 => (1, 0),
        2 => (1, 1),
        _ => (0, 1),
    }
}

pub fn draw_tile(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    position: &Position,
) -> Result<(), JsValue> {
    let board_scale = tuple_as!(BOARD_SCALE, f64);

    let offset = if (position.0 + position.1) % 2 == 0 {
        (224.0, 0.0)
    } else {
        (224.0, 32.0)
    };

    for corner in 0..4 {
        let (x, y) = quadrant_to_xy(corner);

        if random() < 0.75 {
            draw_sprite(
                context,
                atlas,
                offset.0 + 8.0,
                offset.1 + 8.0,
                16.0,
                16.0,
                (position.0 as f64 + x as f64 / 2.0) * board_scale.0,
                (position.1 as f64 + y as f64 / 2.0) as f64 * board_scale.1,
            )?;
        } else {
            context.save();

            draw_sprite(
                context,
                atlas,
                offset.0 + x as f64 * board_scale.0 / 2.0,
                offset.1 + y as f64 * board_scale.1 / 2.0,
                16.0,
                16.0,
                (position.0 as f64 + x as f64 / 2.0) * board_scale.0,
                (position.1 as f64 + y as f64 / 2.0) as f64 * board_scale.1,
            )?;

            context.restore();
        }
    }
    Ok(())
}

pub fn draw_board(
    atlas: &HtmlImageElement,
    dx: f64,
    dy: f64,
    width: usize,
    height: usize,
    clear_width: usize,
    clear_height: usize,
) -> Result<(), JsValue> {
    let board_scale = tuple_as!(BOARD_SCALE, f64);
    let (atlas_canvas, atlas_context) = init_canvas(&CanvasSettings {
        canvas_width: atlas.width(),
        canvas_height: atlas.height(),
        canvas_scale: 1,
        ..Default::default()
    })?;

    atlas_context.draw_image_with_html_image_element(atlas, 0.0, 0.0)?;
    atlas_context.clear_rect(
        dx,
        dy,
        clear_width as f64 * board_scale.0,
        clear_height as f64 * board_scale.1,
    );

    atlas_context.translate(
        dx + ((clear_width - width) as f64 * board_scale.0) / 2.0,
        dy + ((clear_height - height) as f64 * board_scale.1) / 2.0,
    )?;

    for x in 0..width {
        for y in 0..height {
            draw_tile(&atlas_context, atlas, &Position(x as i8, y as i8))?;
        }
    }

    atlas_context.set_global_composite_operation(&"xor")?;

    for x in 0..width {
        for y in 0..height {
            let (edge_l, edge_r, edge_t, edge_b) =
                (x == 0, x == width - 1, y == 0, y == height - 1);

            let (dx, dy) = (x as f64 * board_scale.0, y as f64 * board_scale.1);

            atlas_context.save();
            atlas_context.translate(dx + 16.0, dy + 16.0)?;
            match (edge_l, edge_r, edge_t, edge_b) {
                (true, false, true, false) => {
                    draw_sprite(&atlas_context, atlas, 192.0, 0.0, 32.0, 32.0, -16.0, -16.0)?;
                } // TL corner
                (false, true, true, false) => {
                    atlas_context.rotate(std::f64::consts::PI * 0.5)?;
                    draw_sprite(&atlas_context, atlas, 192.0, 0.0, 32.0, 32.0, -16.0, -16.0)?;
                } // TR corner
                (true, false, false, true) => {
                    atlas_context.rotate(std::f64::consts::PI * 1.5)?;
                    draw_sprite(&atlas_context, atlas, 192.0, 0.0, 32.0, 32.0, -16.0, -16.0)?;
                } // BL corner
                (false, true, false, true) => {
                    atlas_context.rotate(std::f64::consts::PI)?;
                    draw_sprite(&atlas_context, atlas, 192.0, 0.0, 32.0, 32.0, -16.0, -16.0)?;
                } // BR corner
                (true, false, false, false) => {
                    draw_sprite(&atlas_context, atlas, 192.0, 32.0, 32.0, 32.0, -16.0, -16.0)?;
                } // L edge
                (false, true, false, false) => {
                    atlas_context.rotate(std::f64::consts::PI)?;
                    draw_sprite(&atlas_context, atlas, 192.0, 32.0, 32.0, 32.0, -16.0, -16.0)?;
                } // R edge
                (false, false, true, false) => {
                    atlas_context.rotate(std::f64::consts::PI * 0.5)?;
                    draw_sprite(&atlas_context, atlas, 192.0, 32.0, 32.0, 32.0, -16.0, -16.0)?;
                } // T edge
                (false, false, false, true) => {
                    atlas_context.rotate(std::f64::consts::PI * 1.5)?;
                    draw_sprite(&atlas_context, atlas, 192.0, 32.0, 32.0, 32.0, -16.0, -16.0)?;
                } // B edge
                _ => (),
            }
            atlas_context.restore();
        }
    }

    atlas.set_src(atlas_canvas.to_data_url()?.as_str());

    Ok(())
}
