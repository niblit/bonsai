//! The `pieces` module defines the entities that inhabit the chess board.
//!
//! It provides abstractions for:
//! * **[`Kind`]:** The specific type of piece (Pawn, Knight, Bishop, Rook, Queen, King).
//! * **[`ValidPromotions`]:** A subset of **[`Kind`]:** for the valid promotions.
//! * **[`Piece`]:** A combination of a [`Kind`] and a [`crate::atoms::Team`].
//! * **[`LocatedPiece`]:** A [`Piece`] attached to a specific coordinate on the board.

/// Defines the specific type of a chess piece (e.g., Pawn, Knight, King)
/// and valid pawn promotion options.
mod kind;

/// Defines a piece along with its specific coordinate on the chess board.
mod located_piece;

/// Combines a piece kind with a team (color) to represent a concrete piece
/// (e.g., White Pawn, Black Queen).
mod piece;

pub use {
    kind::{Kind, ValidPromotions},
    located_piece::LocatedPiece,
    piece::Piece,
};
