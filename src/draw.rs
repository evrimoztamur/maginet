use shared::Position;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlImageElement};

use crate::BOARD_SCALE_F64;

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
