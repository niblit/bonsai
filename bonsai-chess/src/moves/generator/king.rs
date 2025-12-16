use crate::{
    atoms::CastlingRights,
    board::BoardBackend,
    moves::{
        Ply,
        generator::{
            directions::{
                DIAGONALLY_DOWN_LEFT, DIAGONALLY_DOWN_RIGHT, DIAGONALLY_UP_LEFT,
                DIAGONALLY_UP_RIGHT, DOWN, LEFT, RIGHT, UP,
            },
            sliding::slide,
        },
    },
    pieces::LocatedPiece,
};

pub fn pseudo_legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    castling_rights: CastlingRights,
) -> Vec<Ply> {
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
    todo!("add castling");
    slide(what_to_move, 1, &directions, backend)
}
