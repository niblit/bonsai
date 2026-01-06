use crate::{BOARD_COLUMNS_RANGE, BOARD_ROWS_RANGE};

/// Represents a validated coordinate on the chess board.
///
/// `Coordinates` guarantees that the position is within the valid 8x8 grid (0..8).
/// This prevents out-of-bounds errors when accessing the board array.
///
/// # Coordinate System
/// * **Row**: 0-indexed, corresponding to array indices.
///     * Row 0 = Rank 8 (Black's back rank)
///     * Row 7 = Rank 1 (White's back rank)
/// * **Column**: 0-indexed, corresponding to array indices.
///     * Column 0 = File A
///     * Column 7 = File H
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Coordinates {
    row: usize,
    column: usize,
}

impl Coordinates {
    /// Creates a new `Coordinates` instance if the provided row and column are within bounds.
    ///
    /// Returns `Some(Coordinates)` if `row` and `column` are between 0 and 7 (inclusive).
    /// Returns `None` if they are out of bounds.
    ///
    /// # Type Parameters
    /// This method accepts any type that implements `TryInto<usize>`, allowing usage with
    /// various integer types (e.g., `i32`, `u8`, `isize`).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::Coordinates;
    ///
    /// // Valid coordinates (A8)
    /// let a8 = Coordinates::new(0, 0);
    /// assert!(a8.is_some());
    ///
    /// // Invalid coordinates (out of bounds)
    /// let invalid = Coordinates::new(8, 8);
    /// assert!(invalid.is_none());
    ///
    /// // Using different integer types
    /// let c = Coordinates::new(3i32, 3u8);
    /// assert!(c.is_some());
    /// ```
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

    /// Returns the row index (0-7).
    #[must_use]
    pub const fn row(&self) -> usize {
        self.row
    }

    /// Returns the column index (0-7).
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
        assert!(Coordinates::new(0u8, 1u8).is_some());

        // i8 (negative check)
        assert!(Coordinates::new(-1i8, 0i8).is_none());

        // isize and u8
        assert!(Coordinates::new(6isize, 0u8).is_some());
    }
}
