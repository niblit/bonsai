mod coordinates;
mod grid;
mod kind;
mod located_piece;
mod piece;
mod team;

pub const BOARD_ROWS: usize = 8;
pub const BOARD_COLUMNS: usize = 8;

pub const BOARD_ROWS_RANGE: std::ops::Range<usize> = 0..BOARD_ROWS;
pub const BOARD_COLUMNS_RANGE: std::ops::Range<usize> = 0..BOARD_COLUMNS;
