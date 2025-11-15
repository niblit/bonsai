mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;
mod sliding;

use crate::{
    board::BoardBackend,
    kind::Kind,
    located_piece::{self, LocatedPiece},
    piece::Piece,
    ply::Ply,
};

pub fn generate_pseudo_legal_moves(
    what_to_move: LocatedPiece,
    backend: impl BoardBackend,
) -> Vec<Ply> {
    assert_eq!(
        backend.get(what_to_move.position()),
        Some(what_to_move.piece())
    );
    match what_to_move.piece().kind() {
        Kind::King => king::pseudo_legal_moves(what_to_move, backend),
        Kind::Queen => queen::pseudo_legal_moves(what_to_move, backend),
        Kind::Rook => rook::pseudo_legal_moves(what_to_move, backend),
        Kind::Bishop => bishop::pseudo_legal_moves(what_to_move, backend),
        Kind::Knight => knight::pseudo_legal_moves(what_to_move, backend),
        Kind::Pawn => pawn::pseudo_legal_moves(what_to_move, backend),
    }
}
