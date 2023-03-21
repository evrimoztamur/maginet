use shared::{Mage, Position, Prop, Team};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::app::BOARD_SCALE_F64;

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

pub fn draw_sprite_scaled(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    sx: f64,
    sy: f64,
    sw: f64,
    sh: f64,
    dx: f64,
    dy: f64,
    dw: f64,
    dh: f64,
) -> Result<(), JsValue> {
    context.draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
        atlas, sx, sy, sw, sh, dx, dy, dw, dh,
    )?;

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

pub fn draw_digits(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    location: (i32, i32),
    num: usize,
) -> Result<(), JsValue> {
    let digits = num.max(1).ilog10();

    for i in (0..=digits).rev() {
        let digit = num / 10usize.pow(i) % 10;

        draw_sprite(
            context,
            atlas,
            digit as f64 * 8.0,
            48.0,
            8.0,
            8.0,
            location.0 as f64 + (digits - i) as f64 * 8.0,
            location.1 as f64,
        )?;
    }

    Ok(())
}

pub fn draw_tooltip(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    location: (i32, i32),
    width: usize,
) -> Result<(), JsValue> {
    draw_sprite(
        context,
        atlas,
        96.0,
        0.0,
        1.0,
        12.0,
        location.0 as f64,
        location.1 as f64 - 2.0,
    )?;

    draw_sprite(
        context,
        atlas,
        96.0,
        0.0,
        1.0,
        12.0,
        (location.0 + width as i32 + 1) as f64,
        location.1 as f64 - 2.0,
    )?;

    draw_sprite_scaled(
        context,
        atlas,
        97.0,
        0.0,
        1.0,
        12.0,
        (location.0 + 1) as f64,
        location.1 as f64 - 2.0,
        width as f64,
        12.0,
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
    }

    match mage.team {
        Team::Red => context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                atlas,
                32.0 * (mage.index / 2) as f64,
                64.0 + sleeping_offset,
                32.0,
                32.0,
                -16.0,
                -16.0 + bounce,
                32.0,
                32.0,
            )?,
        Team::Blue => context
            .draw_image_with_html_image_element_and_sw_and_sh_and_dx_and_dy_and_dw_and_dh(
                atlas,
                32.0 * (mage.index / 2) as f64,
                96.0 + sleeping_offset,
                32.0,
                32.0,
                -16.0,
                -16.0 + bounce,
                32.0,
                32.0,
            )?,
    }

    context.restore();

    Ok(())
}

pub fn draw_prop(
    context: &CanvasRenderingContext2d,
    atlas: &HtmlImageElement,
    prop: &Prop,
    frame: u64,
    game_started: bool,
    game_finished: bool,
) -> Result<(), JsValue> {
    context.save();

    match prop {
        Prop::DoubleDamage => {
            context.translate(
                0.0,
                ((((frame as i64 % 24) - 12).abs()) as f64 / -3.0).round() - 6.0,
            )?;

            let flip_timer = (frame as i64 % 48 - 24).abs();

            context.scale(if flip_timer > 12 { 1.0 } else { -1.0 }, 1.0)?;
            draw_sprite(context, atlas, 0.0, 192.0, 32.0, 32.0, -16.0, -16.0)?
        }
    }

    context.restore();

    Ok(())
}
