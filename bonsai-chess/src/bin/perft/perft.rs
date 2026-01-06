use bonsai_chess::prelude::*;

use crate::perft_results::PerftResults;

/// Executes a Perft run starting from the current board state, using multiple threads.
///
/// This function acts as the parallelized root of the recursion. Instead of processing
/// the entire tree on a single thread, it generates the first ply of moves and spawns
/// a separate thread for each move to calculate the sub-results.
///
/// # Arguments
///
/// * `game`: The board state at the root of the tree.
/// * `depth`: The target depth to traverse.
///
/// # Returns
///
/// The aggregated [`PerftResults`] from all threads.
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

    // Spawn a thread for each legal move available at the root.
    for m in moves {
        let mut board_clone = game.clone();

        let handle = std::thread::spawn(move || {
            board_clone.make_move(&m);
            perft(&mut board_clone, depth - 1)
        });

        handles.push(handle);
    }

    let mut total_results = PerftResults::new();

    // Collect and sum results from all threads.
    for handle in handles {
        if let Ok(result) = handle.join() {
            total_results += result;
        }
    }

    total_results
}

/// Recursively calculates the number of leaf nodes at a specific depth.
///
/// This is the core single-threaded worker function.
///
/// # Bulk Counting Optimization
/// When `depth == 1`, we do not need to `make_move` and recurse further. We simply
/// generate the legal moves and count them. This provides a massive speedup at the
/// leaf nodes of the tree.
pub fn perft(game: &mut BoardFrontend, depth: usize) -> PerftResults {
    let mut results = PerftResults::new();

    // BASE CASE (Should only happen if perft called with depth 0)
    if depth == 0 {
        results.nodes = 1;
        return results;
    }

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
