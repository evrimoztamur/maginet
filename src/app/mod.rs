mod app;
mod particle;
mod pointer;

pub use app::*;
pub use particle::*;
pub use pointer::*;

pub const BOARD_OFFSET: (i32, i32) = (8, 8);
pub const BOARD_OFFSET_F64: (f64, f64) = (BOARD_OFFSET.0 as f64, BOARD_OFFSET.1 as f64);
pub const BOARD_SCALE: (i32, i32) = (30, 30);
pub const BOARD_SCALE_F64: (f64, f64) = (BOARD_SCALE.0 as f64, BOARD_SCALE.1 as f64);
