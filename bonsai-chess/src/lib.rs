//! # Bonsai Chess
//!
//! `bonsai-chess` is the core chess logic library for the Bonsai workspace.
//! It is designed to be a modular, type-safe foundation for chess engines and interfaces.
//!
//! ## Core Responsibilities
//!
//! * **State Representation**: Storing the board ([`board`]), pieces ([`pieces`]), and specific locations ([`atoms::Coordinates`]).
//! * **Game Rules**: Enforcing the laws of chess, including turn cycles, castling rights, and checkmate detection ([`rules`]).
//! * **Move Generation**: Calculating legal moves for a given position ([`moves`]).
//!
//! ## Usage
//!
//! Most users will want to import the [`prelude`] to get all essential types:
//!
//! ```rust
//! use bonsai_chess::prelude::*;
//!
//! let mut game = BoardFrontend::from_starting_position();
//! let legal_moves = game.get_legal_moves();
//! ```

/// Defines fundamental atomic types (Coordinates, CastlingRights, etc.).
mod atoms;

/// Manages the board state (Grid, BoardFrontend, BoardBackend).
mod board;

/// Handles move definitions and generation logic.
mod moves;

/// Defines chess pieces (King, Queen, Pawn, etc.).
mod pieces;

/// Defines game outcomes (Checkmate, Draw, Win).
mod rules;

/// A collection of the most common types.
///
/// Importing this module allows you to use `BoardFrontend`, `Piece`, `Ply`, and other
/// core items without managing individual sub-module imports.
pub mod prelude {
    pub use crate::atoms::*;
    pub use crate::board::*;
    pub use crate::moves::*;
    pub use crate::pieces::*;
    pub use crate::rules::*;
    pub use crate::{BOARD_COLUMNS, BOARD_COLUMNS_RANGE, BOARD_ROWS, BOARD_ROWS_RANGE};
}

/// The number of rows (ranks) on a standard chess board.
pub const BOARD_ROWS: usize = 8;

/// The number of columns (files) on a standard chess board.
pub const BOARD_COLUMNS: usize = 8;

/// A range representing all valid row indices (0..8).
///
/// Useful for iterating over the board vertically.
pub const BOARD_ROWS_RANGE: std::ops::Range<usize> = 0..BOARD_ROWS;

/// A range representing all valid column indices (0..8).
///
/// Useful for iterating over the board horizontally.
pub const BOARD_COLUMNS_RANGE: std::ops::Range<usize> = 0..BOARD_COLUMNS;
