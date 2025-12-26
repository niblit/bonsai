mod atoms;
mod board;
mod moves;
mod pieces;
mod rules;

pub const BOARD_ROWS: usize = 8;
pub const BOARD_COLUMNS: usize = 8;

pub const BOARD_ROWS_RANGE: std::ops::Range<usize> = 0..BOARD_ROWS;
pub const BOARD_COLUMNS_RANGE: std::ops::Range<usize> = 0..BOARD_COLUMNS;

pub mod prelude {
    pub use crate::atoms::*;
    pub use crate::board::*;
    pub use crate::moves::*;
    pub use crate::pieces::*;
    pub use crate::rules::*;
    pub use crate::{BOARD_COLUMNS, BOARD_COLUMNS_RANGE, BOARD_ROWS, BOARD_ROWS_RANGE};
}
