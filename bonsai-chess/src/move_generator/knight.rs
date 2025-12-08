use crate::{
    board::BoardGrid,
    located_piece::LocatedPiece,
    move_generator::{
        directions::{
            L_DOWN_LEFT, L_DOWN_RIGHT, L_LEFT_DOWN, L_LEFT_UP, L_RIGHT_DOWN, L_RIGHT_UP, L_UP_LEFT,
            L_UP_RIGHT,
        },
        sliding::slide,
    },
    ply::Ply,
};

pub fn pseudo_legal_moves(what_to_move: LocatedPiece, backend: &BoardGrid) -> Vec<Ply> {
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
