use bonsai_chess::prelude::*;

use crate::perft_results::PerftResults;

pub fn perft(game: &mut BoardFrontend, depth: usize) -> PerftResults {
    let mut results = PerftResults::new();

    // BULK COUNTING OPTIMIZATION:
    // If we are at depth 1, the number of nodes is simply the number of legal moves.
    // We can count other features (captures, etc.) by inspecting the move objects
    // without actually executing them on the board.
    if depth == 1 {
        let moves = game.get_legal_moves();
        results.nodes = moves.len();

        for m in moves {
            if m.piece_captured().is_some() {
                results.captures += 1;
            }

            if let Some(sp) = m.special_move() {
                match sp {
                    SpecialMove::Castle => results.castles += 1,
                    SpecialMove::EnPassant(_) => results.en_passant += 1,
                    SpecialMove::Promotion(_) => results.promotions += 1,
                }
            }
        }

        return results;
    }

    // BASE CASE (Should only happen if perft called with depth 0)
    if depth == 0 {
        results.nodes = 1;
        return results;
    }

    // We must make moves to traverse deeper into the tree.
    let moves = game.get_legal_moves();
    for m in moves {
        game.make_move(&m);

        // Accumulate results from the leaves
        results += perft(game, depth - 1);

        game.undo_last_move();
    }

    results
}
