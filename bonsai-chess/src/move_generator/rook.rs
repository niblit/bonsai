use crate::{board::BoardBackend, located_piece::LocatedPiece, move_generator::{directions::{DOWN, LEFT, RIGHT, UP}, sliding::slide}, ply::Ply};

pub fn pseudo_legal_moves(what_to_move: LocatedPiece, backend: &impl BoardBackend) -> Vec<Ply> {
    let directions = [
        UP,
        DOWN,
        LEFT,
        RIGHT
    ];
    slide(what_to_move, 7, &directions, backend)
}
