use crate::evaluation::{CHECKMATE_SCORE, DRAW_SCORE, evaluate_position, score_move};
use crate::transposition_table::{Entry, NodeType, TranspositionTable};
use bonsai_chess::prelude::*;

pub fn alpha_beta(
    state: &mut BoardFrontend,
    depth: usize,
    mut alpha: isize,
    mut beta: isize,
    best_move_found: &mut Option<Ply>,
    tt: &mut TranspositionTable, // Added parameter
) -> isize {
    let snapshot = state.create_snapshot();

    // 1. Transposition Table Lookup
    if let Some(entry) = tt.get(&snapshot)
        && entry.depth >= depth
    {
        match entry.node_type {
            NodeType::Exact => return entry.score,
            NodeType::Lower => alpha = alpha.max(entry.score),
            NodeType::Upper => beta = beta.min(entry.score),
        }
        if alpha >= beta {
            return entry.score;
        }
    }

    // Handle terminal states and depth exhaustion
    if let Some(outcome) = state.outcome() {
        return match outcome {
            Outcome::Win { winner, .. } => {
                let score = CHECKMATE_SCORE + depth as isize;
                if winner == state.turn() {
                    score
                } else {
                    -score
                }
            }
            Outcome::Draw { .. } => DRAW_SCORE,
        };
    }

    if depth == 0 {
        return evaluate_position(state);
    }

    let mut moves = state.get_legal_moves();
    if moves.is_empty() {
        return evaluate_position(state);
    }

    // Move Ordering
    moves.sort_by_cached_key(|m| -score_move(m));

    let old_alpha = alpha;
    let mut best_move = None;
    let mut best_score = isize::MIN;

    for ply in moves {
        state.make_move(&ply);
        let score = -alpha_beta(state, depth - 1, -beta, -alpha, &mut None, tt);
        state.undo_last_move();

        if score > best_score {
            best_score = score;
            best_move = Some(ply);
        }

        alpha = alpha.max(score);
        if alpha >= beta {
            break; // Beta-cutoff
        }
    }

    // 2. Transposition Table Store
    let node_type = if best_score <= old_alpha {
        NodeType::Upper
    } else if best_score >= beta {
        NodeType::Lower
    } else {
        NodeType::Exact
    };

    tt.insert(
        snapshot,
        Entry {
            score: best_score,
            depth,
            node_type,
            best_move,
        },
    );

    *best_move_found = best_move;
    best_score
}
