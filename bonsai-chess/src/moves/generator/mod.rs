mod bishop;
pub mod directions;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;
mod sliding;

use crate::{
    atoms::{CastlingRights, Coordinates},
    board::BoardBackend,
    moves::Ply,
    pieces::{Kind, LocatedPiece},
};

/// Calculates all mechanically valid moves for a specific piece.
///
/// "Pseudo-legal" means the moves satisfy the movement rules for the piece type
/// (e.g., sliding diagonally, jumping in an L-shape) and board occupancy rules
/// (capturing enemies, blocked by friends).
///
/// **Important:** This function does **not** filter out moves that leave the King in check.
/// That validation step is handled by [`crate::board::BoardFrontend::get_legal_moves`].
///
/// # Arguments
///
/// * `what_to_move`: The piece and its current location.
/// * `backend`: The current state of the board grid.
/// * `en_passant_target`: The coordinate of a pawn that can be captured en passant (if any).
/// * `castling_rights`: The current castling permissions.
#[must_use]
pub fn generate_pseudo_legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    en_passant_target: Option<Coordinates>,
    castling_rights: CastlingRights,
) -> Vec<Ply> {
    match what_to_move.piece().kind() {
        Kind::King => king::pseudo_legal_moves(what_to_move, backend, castling_rights),
        Kind::Queen => queen::pseudo_legal_moves(what_to_move, backend),
        Kind::Rook => rook::pseudo_legal_moves(what_to_move, backend),
        Kind::Bishop => bishop::pseudo_legal_moves(what_to_move, backend),
        Kind::Knight => knight::pseudo_legal_moves(what_to_move, backend),
        Kind::Pawn => pawn::pseudo_legal_moves(what_to_move, backend, en_passant_target),
    }
}
