//! Defines directional constants for grid navigation.
//!
//! These constants represent `(delta_row, delta_column)` offsets used to calculate
//! target coordinates.
//!
//! # Coordinate System
//! * **Rows**: 0 (Top/Black) to 7 (Bottom/White).
//! * **Columns**: 0 (Left/File A) to 7 (Right/File H).
//!
//! Therefore:
//! * **UP** (Towards Rank 8) = Row - 1
//! * **DOWN** (Towards Rank 1) = Row + 1

// Notation: (delta_row, delta_col)

// --- Orthogonal Directions ---

/// Moves one square towards Rank 8 (Row index decreases).
pub const UP: (isize, isize) = (-1, 0);

/// Moves one square towards Rank 1 (Row index increases).
pub const DOWN: (isize, isize) = (1, 0); // Row +1

/// Moves one square towards File A (Column index decreases).
pub const LEFT: (isize, isize) = (0, -1);

/// Moves one square towards File H (Column index increases).
pub const RIGHT: (isize, isize) = (0, 1);

// --- Diagonal Directions ---

/// North-West movement (towards Rank 8, File A direction).
pub const DIAGONALLY_UP_LEFT: (isize, isize) = (-1, -1);

/// North-East movement (towards Rank 8, File H direction).
pub const DIAGONALLY_UP_RIGHT: (isize, isize) = (-1, 1); // Row -1, Col +1

/// South-West movement (towards Rank 1, File A direction).
pub const DIAGONALLY_DOWN_LEFT: (isize, isize) = (1, -1); // Row +1, Col -1

/// South-East movement (towards Rank 1, File H direction).
pub const DIAGONALLY_DOWN_RIGHT: (isize, isize) = (1, 1); // Row +1, Col +1

// --- Knight Jumps (L-Shapes) ---

// - Upward Bias (Major change in Row) -
/// Jump: Up 2, Left 1
pub const L_UP_LEFT: (isize, isize) = (-2, -1);
/// Jump: Up 2, Right 1
pub const L_UP_RIGHT: (isize, isize) = (-2, 1);

// - Downward Bias (Major change in Row) -
/// Jump: Down 2, Left 1
pub const L_DOWN_LEFT: (isize, isize) = (2, -1);
/// Jump: Down 2, Right 1
pub const L_DOWN_RIGHT: (isize, isize) = (2, 1);

// - Leftward Bias (Major change in Col) -
/// Jump: Up 1, Left 2
pub const L_LEFT_UP: (isize, isize) = (-1, -2);
/// Jump: Down 1, Left 2
pub const L_LEFT_DOWN: (isize, isize) = (1, -2);

// - Rightward Bias (Major change in Col) -
/// Jump: Up 1, Right 2
pub const L_RIGHT_UP: (isize, isize) = (-1, 2);
/// Jump: Down 1, Right 2
pub const L_RIGHT_DOWN: (isize, isize) = (1, 2);
