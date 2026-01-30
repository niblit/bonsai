use bonsai_chess::prelude::*;

use crate::{config::SCORING_PROMOTING_PAWNS_BONUS, evaluation::get_piece_value};

#[must_use]
pub const fn score_move(ply: &Ply) -> isize {
    let mut score = 0;

    // MVV-LVA (Most Valuable Victim - Least Valuable Aggressor)
    if let Some(captured) = ply.piece_captured() {
        score += 10 * get_piece_value(captured.kind()) - get_piece_value(ply.piece_moved().kind());
    }

    // Bonus for promoting pawns
    if let Some(SpecialMove::Promotion(_)) = ply.special_move() {
        score += SCORING_PROMOTING_PAWNS_BONUS;
    }

    score
}
