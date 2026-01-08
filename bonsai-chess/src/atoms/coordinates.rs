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

    /// Creates a `Coordinates` instance from a string in standard algebraic notation.
    ///
    /// This function expects a two-character string where the first character represents
    /// the file ('a'-'h') and the second represents the rank ('1'-'8').
    ///
    /// # Arguments
    ///
    /// * `notation` - A string slice representing the square (e.g., "e4", "a1").
    ///
    /// # Returns
    ///
    /// * `Some(Coordinates)` if the string is valid algebraic notation within the board bounds.
    /// * `None` if the string length is not 2, or if the rank/file are invalid characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use bonsai_chess::prelude::Coordinates;
    ///
    /// let coords = Coordinates::from_algebraic_notation("e4").unwrap();
    /// assert_eq!(coords.row(), 4);
    /// assert_eq!(coords.column(), 4);
    ///
    /// assert!(Coordinates::from_algebraic_notation("z9").is_none());
    /// assert!(Coordinates::from_algebraic_notation("a").is_none());
    /// ```
    #[must_use]
    pub fn from_algebraic_notation(notation: &str) -> Option<Self> {
        if notation.len() != 2 {
            return None;
        }

        if let Some(file) = notation.chars().nth(0)
            && let Some(rank) = notation.chars().nth(1)
        {
            let row = match rank {
                '8' => 0,
                '7' => 1,
                '6' => 2,
                '5' => 3,
                '4' => 4,
                '3' => 5,
                '2' => 6,
                '1' => 7,
                _ => return None,
            };

            let column = match file {
                'a' => 0,
                'b' => 1,
                'c' => 2,
                'd' => 3,
                'e' => 4,
                'f' => 5,
                'g' => 6,
                'h' => 7,
                _ => return None,
            };

            return Self::new(row, column);
        }

        None
    }

    /// Converts the coordinates back into a string using standard algebraic notation.
    ///
    /// The resulting string will consist of the file letter ('a'-'h') followed by
    /// the rank number ('1'-'8').
    ///
    /// # Returns
    ///
    /// A `String` representing the square (e.g., "a8", "h1").
    ///
    /// # Panics
    ///
    /// This function panics if the internal `row` or `column` values are outside the
    /// valid 0-7 range.
    ///
    /// *Note: Since `Coordinates` can only be constructed via `new` (which validates bounds),
    /// this panic should be unreachable in safe code.*
    ///
    /// # Examples
    ///
    /// ```
    /// use bonsai_chess::prelude::Coordinates;
    ///
    /// let coords = Coordinates::new(0, 0).unwrap(); // Top-left (A8)
    /// assert_eq!(coords.to_algebraic_notation(), "a8");
    /// ```
    #[must_use]
    pub fn to_algebraic_notation(&self) -> String {
        let rank = match self.row {
            0 => '8',
            1 => '7',
            2 => '6',
            3 => '5',
            4 => '4',
            5 => '3',
            6 => '2',
            7 => '1',
            _ => panic!("Coordinates are hard wired to be in range"),
        };

        let file = match self.column {
            0 => 'a',
            1 => 'b',
            2 => 'c',
            3 => 'd',
            4 => 'e',
            5 => 'f',
            6 => 'g',
            7 => 'h',
            _ => panic!("Coordinates are hard wired to be in range"),
        };

        format!("{file}{rank}")
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

    #[test]
    fn test_from_algebraic_notation_valid() {
        // Test corners
        let a8 = Coordinates::from_algebraic_notation("a8").unwrap();
        assert_eq!(a8.row(), 0);
        assert_eq!(a8.column(), 0);

        let h1 = Coordinates::from_algebraic_notation("h1").unwrap();
        assert_eq!(h1.row(), 7);
        assert_eq!(h1.column(), 7);

        // Test center
        let e4 = Coordinates::from_algebraic_notation("e4").unwrap();
        assert_eq!(e4.row(), 4);
        assert_eq!(e4.column(), 4);
    }

    #[test]
    fn test_from_algebraic_notation_invalid() {
        // Length checks
        assert!(Coordinates::from_algebraic_notation("").is_none());
        assert!(Coordinates::from_algebraic_notation("a").is_none());
        assert!(Coordinates::from_algebraic_notation("a12").is_none());

        // Invalid Files
        assert!(Coordinates::from_algebraic_notation("i1").is_none()); // 'i' is out of bounds
        assert!(Coordinates::from_algebraic_notation("z8").is_none());

        // Invalid Ranks
        assert!(Coordinates::from_algebraic_notation("a0").is_none());
        assert!(Coordinates::from_algebraic_notation("a9").is_none());

        // Case sensitivity (assuming strict lowercase 'a'-'h')
        assert!(Coordinates::from_algebraic_notation("A1").is_none());
    }

    #[test]
    fn test_to_algebraic_notation() {
        let c = Coordinates::new(0, 0).unwrap();
        assert_eq!(c.to_algebraic_notation(), "a8");

        let c = Coordinates::new(7, 7).unwrap();
        assert_eq!(c.to_algebraic_notation(), "h1");

        let c = Coordinates::new(4, 4).unwrap(); // rank 4 (index 4), file e (index 4)
        assert_eq!(c.to_algebraic_notation(), "e4");
    }

    #[test]
    fn test_algebraic_round_trip() {
        // Iterate over every valid square on the board
        for row in BOARD_ROWS_RANGE {
            for column in BOARD_COLUMNS_RANGE {
                let original = Coordinates::new(row, column).unwrap();

                let notation = original.to_algebraic_notation();

                // Ensure we can parse what we output
                let parsed = Coordinates::from_algebraic_notation(&notation)
                    .expect("Generated notation should be valid");

                assert_eq!(
                    original, parsed,
                    "Round trip failed for {notation} (row: {row}, col: {column})"
                );
            }
        }
    }
}
