use crate::{
    board::BoardBackend,
    located_piece::LocatedPiece,
    move_generator::{
        directions::{
            DIAGONALLY_DOWN_LEFT, DIAGONALLY_DOWN_RIGHT, DIAGONALLY_UP_LEFT, DIAGONALLY_UP_RIGHT,
        },
        sliding::slide,
    },
    ply::Ply,
};

pub fn pseudo_legal_moves(what_to_move: LocatedPiece, backend: &impl BoardBackend) -> Vec<Ply> {
    let directions = [
        DIAGONALLY_UP_LEFT,
        DIAGONALLY_UP_RIGHT,
        DIAGONALLY_DOWN_LEFT,
        DIAGONALLY_DOWN_RIGHT,
    ];
    slide(what_to_move, 7, &directions, backend)
}
