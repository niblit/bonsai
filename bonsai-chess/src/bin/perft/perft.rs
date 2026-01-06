use bonsai_chess::prelude::*;

use crate::perft_results::PerftResults;

pub fn root_level_perft(game: &mut BoardFrontend, depth: usize) -> PerftResults {
    // 1. Handle Depth 0 (Base Case)
    // Perft(0) is just the current board state itself (1 node).
    if depth == 0 {
        let mut res = PerftResults::new();
        res.nodes = 1;
        return res;
    }

    // 2. Optimization for Depth 1
    // Threading is slower than serial execution for shallow depths.
    if depth == 1 {
        return perft(game, 1);
    }

    let moves = game.get_legal_moves();
    let mut handles = Vec::new();

    for m in moves {
        let mut board_clone = game.clone();

        let handle = std::thread::spawn(move || {
            board_clone.make_move(&m);
            // This is now safe because we ensured depth > 0 above
            perft(&mut board_clone, depth - 1)
        });

        handles.push(handle);
    }

    let mut total_results = PerftResults::new();

    for handle in handles {
        if let Ok(result) = handle.join() {
            total_results += result;
        }
    }

    total_results
}

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
