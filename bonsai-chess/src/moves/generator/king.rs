use crate::{
    board::BoardBackend,
    moves::Ply,
    moves::generator::{
        directions::{
            DIAGONALLY_DOWN_LEFT, DIAGONALLY_DOWN_RIGHT, DIAGONALLY_UP_LEFT, DIAGONALLY_UP_RIGHT,
            DOWN, LEFT, RIGHT, UP,
        },
        sliding::slide,
    },
    pieces::LocatedPiece,
};

pub fn pseudo_legal_moves(what_to_move: LocatedPiece, backend: &BoardBackend) -> Vec<Ply> {
    let directions = [
        UP,
        DOWN,
        LEFT,
        RIGHT,
        DIAGONALLY_UP_LEFT,
        DIAGONALLY_UP_RIGHT,
        DIAGONALLY_DOWN_LEFT,
        DIAGONALLY_DOWN_RIGHT,
    ];
    slide(what_to_move, 1, &directions, backend)
}
