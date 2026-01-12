use crate::{atoms::Coordinates, pieces::ValidPromotions};

/// Represents moves that involve mechanics beyond standard displacement or capture.
///
/// Standard moves simply take a piece from A and place it on B (capturing the occupant of B).
/// Special moves require additional logic to update the board state correctly.
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

/// Represents the side of the board where the king castled
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum CastlingSide {
    Short,
    Long,
}
