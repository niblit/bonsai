//! # Board Grid
//!
//! This module defines the [`Grid`] struct, a strongly-typed wrapper around
//! the raw 2D array that represents the chess board's spatial layout. It
//! provides the foundational data structure used by the board backends, along
//! with formatting traits for easy visualization and dereferencing traits for
//! ergonomic array access.

use crate::{BOARD_COLUMNS, BOARD_ROWS, board::Square};

/// A wrapper struct representing the internal 8x8 chess board state.
/// This creates a local type so we can implement traits like Display.
///
/// The `Grid` is a fixed-size 2D array of [`Square`] items, representing the board in a
/// **row-major** layout (`grid[row][column]`).
///
/// # Coordinate Mapping
///
/// Based on the engine's starting position configuration:
/// * **Row 0**: Corresponds to **Rank 8** (Black's back rank).
/// * **Row 7**: Corresponds to **Rank 1** (White's back rank).
/// * **Column 0**: Corresponds to **File A**.
/// * **Column 7**: Corresponds to **File H**.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Grid(pub [[Square; BOARD_COLUMNS]; BOARD_ROWS]);

impl Grid {
    /// Creates a new `Grid` from an existing 2D array of squares.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::{Grid, BOARD_ROWS, BOARD_COLUMNS};
    ///
    /// // Create a completely empty grid
    /// let empty_array = [[None; BOARD_COLUMNS]; BOARD_ROWS];
    /// let grid = Grid::new(empty_array);
    /// ```
    #[must_use]
    pub const fn new(grid: [[Square; BOARD_COLUMNS]; BOARD_ROWS]) -> Self {
        Self(grid)
    }
}

/// Formats the grid into a human-readable ASCII chess board.
///
/// This is extremely useful for debugging board states visually in the terminal.
impl std::fmt::Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        const FILES: &str = "    a   b   c   d   e   f   g   h";
        const BORDER: &str = "  +---+---+---+---+---+---+---+---+";

        let mut board_string = FILES.to_string();

        for (row_idx, row) in self.iter().enumerate() {
            // Row 0 is Rank 8, Row 7 is Rank 1
            let rank = 8 - row_idx;

            // Top border for this row
            board_string = format!("{board_string}\n{BORDER}");

            // At a rank hint at the left
            board_string = format!("{board_string}\n{rank}");

            // row content
            for square in row {
                let symbol = square
                    .as_ref()
                    .map_or_else(|| String::from(" "), std::string::ToString::to_string);
                board_string = format!("{board_string} | {symbol}");
            }

            // right-hand border and rank number
            board_string = format!("{board_string} | {rank}");
        }

        // final bottom border
        board_string = format!("{board_string}\n{BORDER}");

        // file letters
        board_string = format!("{board_string}\n{FILES}");

        write!(f, "{board_string}")
    }
}

/// Implementing Deref allows you to use `grid[0][0]` directly on your struct without typing `grid.0[0][0]`.
impl std::ops::Deref for Grid {
    type Target = [[Square; BOARD_COLUMNS]; BOARD_ROWS];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Same with Deref, but mutably.
impl std::ops::DerefMut for Grid {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
