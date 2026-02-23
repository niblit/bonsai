mod bishop;
pub mod directions;
mod king;
mod knight;
mod legality_context;
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

pub use legality_context::LegalityContext;

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
pub fn generate_legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    en_passant_target: Option<Coordinates>,
    castling_rights: CastlingRights,
    context: &LegalityContext,
    buffer: &mut Vec<Ply>,
) {
    if context.in_double_check() && what_to_move.piece().kind() != Kind::King {
        // Double check: Only the king can move.
        return;
    }

    match what_to_move.piece().kind() {
        Kind::King => king::legal_moves(what_to_move, backend, castling_rights, context, buffer),
        Kind::Queen => queen::legal_moves(what_to_move, backend, context, buffer),
        Kind::Rook => rook::legal_moves(what_to_move, backend, context, buffer),
        Kind::Bishop => bishop::legal_moves(what_to_move, backend, context, buffer),
        Kind::Knight => knight::legal_moves(what_to_move, backend, context, buffer),
        Kind::Pawn => pawn::legal_moves(what_to_move, backend, en_passant_target, context, buffer),
    }
}
