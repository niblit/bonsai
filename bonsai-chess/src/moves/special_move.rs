//! # Special Moves
//!
//! This module defines the [`SpecialMove`] and [`CastlingSide`] enums, which
//! represent chess maneuvers that break the standard rules of single-piece
//! displacement and direct capture. These moves require complex, multi-step
//! updates to the board state.

use crate::{atoms::Coordinates, pieces::ValidPromotions};

/// Represents moves that involve mechanics beyond standard displacement or capture.
///
/// Standard moves simply take a piece from square A and place it on square B
/// (capturing the occupant of B, if any). Special moves require additional,
/// specialized logic to update the board state correctly (e.g., moving a second
/// piece, removing a piece on a different square, or morphing a piece's type).
///
/// # Examples
///
/// ```rust
/// use bonsai_chess::prelude::{Coordinates, ValidPromotions, SpecialMove, CastlingSide};
///
/// // A Kingside castle
/// let castle = SpecialMove::Castle(CastlingSide::Short);
///
/// // A pawn promoting to a Queen
/// let promotion = SpecialMove::Promotion(ValidPromotions::Queen);
///
/// // An en passant capture, where the captured pawn is on d5
/// let d5 = Coordinates::from_algebraic_notation("d5").unwrap();
/// let en_passant = SpecialMove::EnPassant(d5);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SpecialMove {
    /// Castling (King-side or Queen-side).
    ///
    /// The move is represented as the King's movement. The backend logic must
    /// identify the corresponding Rook and move it to the correct adjacent square.
    Castle(CastlingSide),

    /// En Passant capture.
    ///
    /// This occurs when a pawn captures an opposing pawn that has just moved two squares.
    ///
    /// # Payload
    /// * `Coordinates`: The location of the **captured pawn**.
    ///   Note: This is *not* the destination square of the moving pawn (which is empty).
    ///   The backend needs this specific coordinate to remove the captured pawn.
    EnPassant(Coordinates),

    /// Pawn Promotion.
    ///
    /// Occurs when a pawn advances to the final rank.
    ///
    /// # Payload
    /// * [`ValidPromotions`]: The specific piece type selected by the player
    ///   (Queen, Rook, Bishop, or Knight).
    Promotion(ValidPromotions),
}

/// Represents the side of the board where the king castled.
///
/// Castling involves the King moving two squares towards a Rook, and the Rook
/// jumping over the King to the adjacent square.
///
/// # Examples
///
/// ```rust
/// use bonsai_chess::prelude::{CastlingSide, SpecialMove};
///
/// let side = CastlingSide::Short;
/// let move_type = SpecialMove::Castle(side);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CastlingSide {
    /// Kingside castling (O-O).
    ///
    /// The King moves two squares towards the h-file Rook.
    Short,

    /// Queenside castling (O-O-O).
    ///
    /// The King moves two squares towards the a-file Rook.
    Long,
}
