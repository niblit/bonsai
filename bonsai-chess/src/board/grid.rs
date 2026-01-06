use crate::{BOARD_COLUMNS, BOARD_ROWS, board::Square};

/// A type alias representing the internal 8x8 chess board state.
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
pub type Grid = [[Square; BOARD_COLUMNS]; BOARD_ROWS];
