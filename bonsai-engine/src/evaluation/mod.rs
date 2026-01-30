mod piece_square_tables;
mod score_move;
mod score_position;

pub use piece_square_tables::*;
pub use score_move::score_move;
pub use score_position::evaluate_position;

use bonsai_chess::prelude::*;

#[must_use]
const fn get_piece_value(kind: Kind) -> isize {
    match kind {
        Kind::Pawn => 100,
        Kind::Knight => 320,
        Kind::Bishop => 330,
        Kind::Rook => 500,
        Kind::Queen => 900,
        Kind::King => 20000,
    }
}
