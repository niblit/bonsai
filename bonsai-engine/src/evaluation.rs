use bonsai_chess::prelude::*;

pub const CHECKMATE_SCORE: isize = 1_000_000;
pub const DRAW_SCORE: isize = 0;

#[must_use]
pub fn evaluate_position(state: &BoardFrontend) -> isize {
    // Check for game-ending conditions first
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
        let val = get_piece_value(lp.piece().kind());

        if lp.piece().team() == state.turn() {
            score += val;
        } else {
            score -= val;
        }
    }
    score
}

#[must_use]
pub const fn score_move(ply: &Ply) -> isize {
    let mut score = 0;

    // MVV-LVA (Most Valuable Victim - Least Valuable Aggressor)
    if let Some(captured) = ply.piece_captured() {
        score += 10 * get_piece_value(captured.kind()) - get_piece_value(ply.piece_moved().kind());
    }

    // Bonus for promoting pawns
    if let Some(SpecialMove::Promotion(_)) = ply.special_move() {
        score += 800;
    }

    score
}

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
