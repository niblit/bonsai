//! The `pieces` module defines the entities that inhabit the chess board.
//!
//! It provides abstractions for:
//! * **[`Kind`]:** The specific type of piece (Pawn, Knight, Bishop, Rook, Queen, King).
//! * **[`ValidPromotions`]:** A subset of **[`Kind`]:** for the valid promotions.
//! * **[`Piece`]:** A combination of a [`Kind`] and a [`crate::atoms::Team`].
//! * **[`LocatedPiece`]:** A [`Piece`] attached to a specific coordinate on the board.

mod kind;
mod located_piece;
mod piece;

pub use {
    kind::{Kind, ValidPromotions},
    located_piece::LocatedPiece,
    piece::Piece,
};
