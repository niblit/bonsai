use crate::{
    board::square::Square, coordinates::Coordinates, special_move::SpecialMove
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Ply {
    starting_square: Coordinates,
    ending_square: Coordinates,

    piece_moved: Square,
    piece_captured: Square,

    special_move: Option<SpecialMove>
}