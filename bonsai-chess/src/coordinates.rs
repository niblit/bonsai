use crate::{BOARD_COLUMNS, BOARD_COLUMNS_RANGE, BOARD_ROWS, BOARD_ROWS_RANGE};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinates {
    row: usize,
    column: usize,
}

impl Coordinates {
    #[must_use]
    pub fn new(row: usize, column: usize) -> Option<Self> {
        if BOARD_ROWS_RANGE.contains(&row) && BOARD_COLUMNS_RANGE.contains(&column) {
            return Some(Self { row, column });
        }
        None
    }

    #[must_use]
    pub fn row(&self) -> usize {
        self.row
    }

    #[must_use]
    pub fn column(&self) -> usize {
        self.column
    }

    #[must_use]
    pub fn up(&self) -> Option<Coordinates> {
        if self.row == 0 {
            return None;
        }

        Coordinates::new(
            self.row - 1,
            self.column
        )
    }

    #[must_use]
    pub fn down(&self) -> Option<Coordinates> {
        if self.row >= (BOARD_ROWS - 1) {
            return None;
        }
        Coordinates::new(
            self.row + 1,
            self.column
        )
    }

    #[must_use]
    pub fn left(&self) -> Option<Coordinates> {
        if self.column == 0 {
            return None;
        }
        Coordinates::new(
            self.row,
            self.column - 1
        )
    }

    #[must_use]
    pub fn right(&self) -> Option<Coordinates> {
        if self.column >= (BOARD_COLUMNS - 1) {
            return None;
        }
        Coordinates::new(
            self.row,
            self.column + 1
        )
    }
}
