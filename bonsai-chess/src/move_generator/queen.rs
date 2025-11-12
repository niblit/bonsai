use crate::{board::BoardBackend, located_piece::LocatedPiece, ply::Ply};

pub fn pseudo_legal_moves(
    what_to_move: LocatedPiece,
    backend: impl BoardBackend
) -> Vec<Ply> {
    todo!()
}