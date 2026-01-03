use crate::{BOARD_COLUMNS_RANGE, BOARD_ROWS_RANGE};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinates {
    row: usize,
    column: usize,
}

impl Coordinates {
    #[must_use]
    pub fn new<IntegerA, IntegerB>(row: IntegerA, column: IntegerB) -> Option<Self>
    where
        IntegerA: TryInto<usize>,
        IntegerB: TryInto<usize>,
    {
        let row_usize = row.try_into().ok()?;
        let column_usize = column.try_into().ok()?;

        if BOARD_ROWS_RANGE.contains(&row_usize) && BOARD_COLUMNS_RANGE.contains(&column_usize) {
            Some(Self {
                row: row_usize,
                column: column_usize,
            })
        } else {
            None
        }
    }

    #[must_use]
    pub const fn row(&self) -> usize {
        self.row
    }

    #[must_use]
    pub const fn column(&self) -> usize {
        self.column
    }
}

#[cfg(test)]
mod tests {
    use crate::{BOARD_COLUMNS, BOARD_ROWS};

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
        assert!(Coordinates::new(BOARD_ROWS, BOARD_COLUMNS).is_none());
        assert!(Coordinates::new(BOARD_ROWS, 0).is_none());
        assert!(Coordinates::new(0, BOARD_COLUMNS).is_none());
    }

    #[test]
    fn test_mixed_integer_types() {
        // usize (standard)
        assert!(Coordinates::new(0usize, 0usize).is_some());

        // i32
        assert!(Coordinates::new(0i32, 0i32).is_some());

        // u8
        assert!(Coordinates::new(0u8, 108).is_some());

        // i8 (negative check)
        assert!(Coordinates::new(-1i8, 0i8).is_none());
    }
}
