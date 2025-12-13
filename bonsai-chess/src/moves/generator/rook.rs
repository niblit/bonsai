use crate::{
    board::BoardBackend,
    moves::Ply,
    moves::generator::{
        directions::{DOWN, LEFT, RIGHT, UP},
        sliding::slide,
    },
    pieces::LocatedPiece,
};

pub fn pseudo_legal_moves(what_to_move: LocatedPiece, backend: &BoardBackend) -> Vec<Ply> {
    let directions = [UP, DOWN, LEFT, RIGHT];
    slide(what_to_move, 7, &directions, backend)
}
