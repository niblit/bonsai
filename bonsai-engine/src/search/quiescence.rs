use crate::evaluation::{evaluate_position, score_move};
use bonsai_chess::prelude::*;

// New Quiescence Search Function
pub fn quiescence(state: &mut BoardFrontend, mut alpha: isize, beta: isize) -> isize {
    let stand_pat = evaluate_position(state);

    // Beta cutoff (Standing pat is good enough)
    if stand_pat >= beta {
        return beta;
    }

    // Alpha update
    if stand_pat > alpha {
        alpha = stand_pat;
    }

    let mut moves = state.get_legal_moves();

    // OPTIMIZATION: Only consider capturing moves
    // (Assumes bonsai_chess Ply has piece_captured or similar check)
    moves.retain(|m| m.piece_captured().is_some());

    // Sort captures by MVV-LVA
    moves.sort_by_cached_key(|m| -score_move(m));

    for ply in moves {
        state.make_move(&ply);
        let score = -quiescence(state, -beta, -alpha);
        state.undo_last_move();

        if score >= beta {
            return beta;
        }
        if score > alpha {
            alpha = score;
        }
    }
    alpha
}
