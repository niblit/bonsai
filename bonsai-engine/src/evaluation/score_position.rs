use bonsai_chess::prelude::*;

use crate::{
    config::{CHECKMATE_SCORE, DRAW_SCORE},
    evaluation::{KNIGHT_TABLE, PAWN_TABLE, flip_square, get_piece_value},
};

#[must_use]
pub fn evaluate_position(state: &BoardFrontend) -> isize {
    if let Some(outcome) = state.outcome() {
        return match outcome {
            Outcome::Win { winner, .. } => {
                if winner == state.turn() {
                    CHECKMATE_SCORE
                } else {
                    -CHECKMATE_SCORE
                }
            }
            Outcome::Draw { .. } => DRAW_SCORE,
        };
    }

    let mut score = 0;
    let pieces = state.backend().get_all_pieces();

    for lp in pieces {
        let piece = lp.piece();
        let kind = piece.kind();
        let team = piece.team();

        // 1. Material Score
        let material = get_piece_value(kind);

        // 2. Positional Score (PST)
        // You need to extract the square index (0-63) from `lp`
        // Assuming `lp.square().index()` exists and returns usize 0-63
        let sq_index = lp.position().row() * BOARD_COLUMNS + lp.position().column();

        let position_bonus = match kind {
            Kind::Pawn => {
                if team == Team::White {
                    PAWN_TABLE[sq_index]
                } else {
                    PAWN_TABLE[flip_square(sq_index)]
                }
            }
            Kind::Knight => {
                if team == Team::White {
                    KNIGHT_TABLE[sq_index]
                } else {
                    KNIGHT_TABLE[flip_square(sq_index)]
                }
            }
            // Add tables for other pieces...
            _ => 0,
        };

        let total_val = material + position_bonus;

        if team == state.turn() {
            score += total_val;
        } else {
            score -= total_val;
        }
    }
    score
}
