// Notation: (delta_row, delta_col)

// Change ROW (1st val) to move vertically
pub const UP: (isize, isize) = (-1, 0); // Row -1
pub const DOWN: (isize, isize) = (1, 0); // Row +1

// Change COL (2nd val) to move horizontally
pub const LEFT: (isize, isize) = (0, -1); // Col -1
pub const RIGHT: (isize, isize) = (0, 1); // Col +1

pub const DIAGONALLY_UP_LEFT: (isize, isize) = (-1, -1); // Row -1, Col -1
pub const DIAGONALLY_UP_RIGHT: (isize, isize) = (-1, 1); // Row -1, Col +1
pub const DIAGONALLY_DOWN_LEFT: (isize, isize) = (1, -1); // Row +1, Col -1
pub const DIAGONALLY_DOWN_RIGHT: (isize, isize) = (1, 1); // Row +1, Col +1

// Notation: (delta_row, delta_col)

// --- Upward Bias (Major change in Row) ---
pub const L_UP_LEFT: (isize, isize) = (-2, -1); // Up 2 rows, Left 1 col
pub const L_UP_RIGHT: (isize, isize) = (-2, 1); // Up 2 rows, Right 1 col

// --- Downward Bias (Major change in Row) ---
pub const L_DOWN_LEFT: (isize, isize) = (2, -1); // Down 2 rows, Left 1 col
pub const L_DOWN_RIGHT: (isize, isize) = (2, 1); // Down 2 rows, Right 1 col

// --- Leftward Bias (Major change in Col) ---
pub const L_LEFT_UP: (isize, isize) = (-1, -2); // Up 1 row, Left 2 cols
pub const L_LEFT_DOWN: (isize, isize) = (1, -2); // Down 1 row, Left 2 cols

// --- Rightward Bias (Major change in Col) ---
pub const L_RIGHT_UP: (isize, isize) = (-1, 2); // Up 1 row, Right 2 cols
pub const L_RIGHT_DOWN: (isize, isize) = (1, 2); // Down 1 row, Right 2 cols
