use crate::{
    board::BoardBackend,
    moves::Ply,
    moves::generator::{
        directions::{
            L_DOWN_LEFT, L_DOWN_RIGHT, L_LEFT_DOWN, L_LEFT_UP, L_RIGHT_DOWN, L_RIGHT_UP, L_UP_LEFT,
            L_UP_RIGHT,
        },
        sliding::slide,
    },
    pieces::LocatedPiece,
};

pub fn pseudo_legal_moves(what_to_move: LocatedPiece, backend: &BoardBackend) -> Vec<Ply> {
    let directions = [
        L_UP_LEFT,
        L_UP_RIGHT,
        L_DOWN_LEFT,
        L_DOWN_RIGHT,
        L_LEFT_UP,
        L_LEFT_DOWN,
        L_RIGHT_UP,
        L_RIGHT_DOWN,
    ];
    slide(what_to_move, 1, &directions, backend)
}
