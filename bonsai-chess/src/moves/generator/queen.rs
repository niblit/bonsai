use crate::{
    board::BoardBackend,
    moves::{
        LegalityContext, Ply,
        generator::{bishop, rook},
    },
    pieces::LocatedPiece,
};

/// Generates pseudo-legal moves for a Queen.
///
/// The Queen combines the power of the Rook and the Bishop, moving any number of
/// squares orthogonally or diagonally.
///
/// # Implementation
/// This function aggregates the results from:
/// * [`rook::pseudo_legal_moves`]
/// * [`bishop::pseudo_legal_moves`]
pub fn legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    context: &LegalityContext,
    buffer: &mut Vec<Ply>,
) {
    rook::legal_moves(what_to_move, backend, context, buffer);
    bishop::legal_moves(what_to_move, backend, context, buffer);
}
