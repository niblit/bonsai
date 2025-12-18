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

pub fn generate_pseudo_legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    en_passant_target: Option<Coordinates>,
    castling_rights: CastlingRights,
) -> Vec<Ply> {
    assert_eq!(
        backend.get(what_to_move.position()),
        Some(what_to_move.piece())
    );
    match what_to_move.piece().kind() {
        Kind::King => king::pseudo_legal_moves(what_to_move, backend, castling_rights),
        Kind::Queen => queen::pseudo_legal_moves(what_to_move, backend),
        Kind::Rook => rook::pseudo_legal_moves(what_to_move, backend),
        Kind::Bishop => bishop::pseudo_legal_moves(what_to_move, backend),
        Kind::Knight => knight::pseudo_legal_moves(what_to_move, backend),
        Kind::Pawn => pawn::pseudo_legal_moves(what_to_move, backend, en_passant_target),
    }
}
