mod board;
mod castling_rights;
mod coordinates;
mod kind;
mod located_piece;
mod move_generator;
mod outcome;
mod piece;
mod ply;
mod special_move;
mod team;

pub const BOARD_ROWS: usize = 8;
pub const BOARD_COLUMNS: usize = 8;

pub const BOARD_ROWS_RANGE: std::ops::Range<usize> = 0..BOARD_ROWS;
pub const BOARD_COLUMNS_RANGE: std::ops::Range<usize> = 0..BOARD_COLUMNS;

pub mod prelude {
    pub use crate::board::Board;
    pub use crate::board::BoardGrid;
}
