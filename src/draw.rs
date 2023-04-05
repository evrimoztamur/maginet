use shared::{Mage, Position, Team};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::app::{Particle, ParticleSort, BOARD_SCALE_F64};

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
    draw_sprite(
        context,
        atlas,
        offset.0,
        offset.1,
        8.0,
        8.0,
        position.0 as f64 * BOARD_SCALE_F64.0 + (frame / 6 % 4) as f64,
        position.1 as f64 * BOARD_SCALE_F64.1 + (frame / 6 % 4) as f64,
    )?;

    draw_sprite(
        context,
        atlas,
        offset.0 + 8.0,
        offset.1,
        8.0,
        8.0,
        position.0 as f64 * BOARD_SCALE_F64.0 - (frame / 6 % 4) as f64 + BOARD_SCALE_F64.0 - 8.0,
        position.1 as f64 * BOARD_SCALE_F64.1 + (frame / 6 % 4) as f64,
    )?;

    draw_sprite(
        context,
        atlas,
        offset.0,
        offset.1 + 8.0,
        8.0,
        8.0,
        position.0 as f64 * BOARD_SCALE_F64.0 + (frame / 6 % 4) as f64,
        position.1 as f64 * BOARD_SCALE_F64.1 - (frame / 6 % 4) as f64 + BOARD_SCALE_F64.1 - 8.0,
    )?;

    draw_sprite(
        context,
        atlas,
        offset.0 + 8.0,
        offset.1 + 8.0,
        8.0,
        8.0,
        position.0 as f64 * BOARD_SCALE_F64.0 - (frame / 6 % 4) as f64 + BOARD_SCALE_F64.0 - 8.0,
        position.1 as f64 * BOARD_SCALE_F64.1 - (frame / 6 % 4) as f64 + BOARD_SCALE_F64.1 - 8.0,
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
) -> Result<(), JsValue> {
    let bounce = (if mage.is_alive() && (mage.team == team && game_started || game_finished) {
        2 - ((frame as i64 / 6 + mage.index as i64 / 2) % 4 - 2).abs()
    } else {
        0
    }) as f64;

    let sleeping_offset = if mage.is_alive() && game_started {
        0.0
    } else {
        64.0
    };

    context.save();

    if game_finished && mage.is_alive() {
        context.translate(
            0.0,
            ((frame as i64 % 80 - 40).max(0) - 20).abs() as f64 - 20.0,
        )?;
        context
            .rotate(((frame as i64 % 80 - 35).max(0) / 5) as f64 * std::f64::consts::PI / 2.0)?;
    } else if !mage.is_alive() {
        context.translate(0.0, 4.0)?;
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
                -15.0,
                -20.0 + bounce,
                32.0,
                32.0,
            )?,
        Team::Blue => context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                atlas,
                sprite_x,
                96.0 + sleeping_offset,
                32.0,
                32.0,
                -15.0,
                -20.0 + bounce,
                32.0,
                32.0,
            )?,
    }

    context.restore();

    for i in 0..mage.mana.0 {
        draw_sprite(
            context,
            atlas,
            80.0,
            12.0,
            4.0,
            4.0,
            i as f64 * 6.0 - mage.mana.0 as f64 * 3.0 + 2.0,
            10.0,
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
    context.save();
    context.translate(
        ((particle.position.0 + 0.5) * BOARD_SCALE_F64.0).floor(),
        ((particle.position.1 + 0.5) * BOARD_SCALE_F64.1).floor(),
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
