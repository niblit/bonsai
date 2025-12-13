use crate::{
    board::BoardBackend,
    moves::{
        Ply,
        generator::{
            directions::{
                DIAGONALLY_DOWN_LEFT, DIAGONALLY_DOWN_RIGHT, DIAGONALLY_UP_LEFT,
                DIAGONALLY_UP_RIGHT,
            },
            sliding::slide,
        },
    },
    pieces::LocatedPiece,
};

pub fn pseudo_legal_moves(what_to_move: LocatedPiece, backend: &BoardBackend) -> Vec<Ply> {
    let directions = [
        DIAGONALLY_UP_LEFT,
        DIAGONALLY_UP_RIGHT,
        DIAGONALLY_DOWN_LEFT,
        DIAGONALLY_DOWN_RIGHT,
    ];
    slide(what_to_move, 7, &directions, backend)
}
