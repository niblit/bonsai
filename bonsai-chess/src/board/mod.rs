//! The `board` module manages the state of the chess board and the game progression.
//!
//! It is split into two main layers:
//! * **Backend ([`BoardBackend`])**: Handles the raw data structure (the 8x8 grid) and
//!   basic piece placement/removal. It is unaware of high-level rules like turns or checks.
//! * **Frontend ([`BoardFrontend`])**: Manages the complete game state, including the turn
//!   cycle, move history, castling rights, and rule enforcement (e.g., checks, draw conditions).
//!
//! Additionally, it exposes:
//! * [`Grid`]: The underlying 2D array data structure.
//! * [`Square`]: A type alias representing a possibly empty spot on the board.

/// Manages the raw placement and removal of pieces on the 8x8 grid.
mod board_backend;

/// Manages the high-level game state, turn cycles, move history, and move validation.
mod board_frontend;

/// Handles parsing and generating Forsyth-Edwards Notation (FEN) strings.
mod fen;

/// Provides the underlying 2D array representation of the chess board.
mod grid;

/// Contains standard starting positions and notable test configurations.
mod positions;

/// Captures the complete state of the board at a specific point in time (used for threefold repetition tracking).
mod snapshot;

/// Defines a type alias for a potentially unoccupied space on the board.
mod square;

pub use board_backend::BoardBackend;
pub use board_frontend::BoardFrontend;
pub use fen::{FenParsingError, from_fen, to_fen};
pub use grid::Grid;
pub use snapshot::PositionSnapshot;
pub use square::Square;
