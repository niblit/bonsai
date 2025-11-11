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
        Some(Self { row, column })
    } else {
        None
    }
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
        let new_row = self.row.checked_sub(1)?;
        Coordinates::new(new_row, self.column)
    }

    #[must_use]
    pub fn down(&self) -> Option<Coordinates> {
        let new_row = self.row.checked_add(1)?;
        Coordinates::new(new_row, self.column)
    }

    #[must_use]
    pub fn left(&self) -> Option<Coordinates> {
        let new_column = self.column.checked_sub(1)?;
        Coordinates::new(self.row, new_column)
    }

    #[must_use]
    pub fn right(&self) -> Option<Coordinates> {
        let new_column = self.column.checked_add(1)?;
        Coordinates::new(self.row, new_column)
    }

    #[must_use]
    pub fn diagonal_up_left(&self) -> Option<Coordinates> {
        self.up()?.left()
    }

    #[must_use]
    pub fn diagonal_up_right(&self) -> Option<Coordinates> {
        self.up()?.right()
    }

    #[must_use]
    pub fn diagonal_down_right(&self) -> Option<Coordinates> {
        self.down()?.right()
    }

    #[must_use]
    pub fn diagonal_down_left(&self) -> Option<Coordinates> {
        self.down()?.left()
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_valid() {
        for row in BOARD_ROWS_RANGE {
            for column in BOARD_COLUMNS_RANGE {
                let c = Coordinates::new(row, column);
                assert!(c.is_some());
            }
        }
    }

    #[test]
    fn test_new_invalid() {
        assert!(
            Coordinates::new(BOARD_ROWS, BOARD_COLUMNS).is_none()
        );
        assert!(
            Coordinates::new(BOARD_ROWS, 0).is_none()
        );
        assert!(
            Coordinates::new(0, BOARD_COLUMNS).is_none()
        );
    }
}