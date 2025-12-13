use crate::{board::BoardBackend, moves::Ply, pieces::LocatedPiece};

pub fn pseudo_legal_moves(what_to_move: LocatedPiece, backend: &BoardBackend) -> Vec<Ply> {
    Vec::new()
}
