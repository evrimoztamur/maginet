mod app;
mod particle;
mod pointer;
mod state;

pub use app::*;
pub use particle::*;
pub use pointer::*;
pub use state::*;

pub const BOARD_OFFSET: (i32, i32) = (0, 0);
pub const BOARD_OFFSET_F64: (f64, f64) = (BOARD_OFFSET.0 as f64, BOARD_OFFSET.1 as f64);
pub const BOARD_SCALE: (i32, i32) = (32, 32);
pub const BOARD_SCALE_F64: (f64, f64) = (BOARD_SCALE.0 as f64, BOARD_SCALE.1 as f64);
