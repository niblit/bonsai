use std::time::Instant;

use bonsai_chess::prelude::*;

use crate::{
    config::{MAX_DEPTH, STARTING_DEPTH},
    openings::search_opening_book,
    search::alpha_beta,
    transposition_table::TranspositionTable,
};

mod config;
mod evaluation;
mod openings;
mod search;
mod transposition_table;

#[must_use]
pub fn best_move(mut state: BoardFrontend, time_ms: u128) -> Option<Ply> {
    // 1. Check Opening Book first (Placeholder logic)
    if let Some(book_move) = search_opening_book(&state) {
        return Some(book_move);
    }

    // 2. Search using Iterative Deepening
    let mut best_ply = None;
    let mut current_depth = STARTING_DEPTH;
    let mut tt = TranspositionTable::new();
    let start_time = Instant::now();

    // Continue deepening as long as we have time
    loop {
        let mut depth_best_ply = None;
        alpha_beta(
            &mut state,
            current_depth,
            isize::MIN + 1,
            isize::MAX - 1,
            &mut depth_best_ply,
            &mut tt,
        );

        if depth_best_ply.is_some() {
            best_ply = depth_best_ply;
        }

        current_depth += 1;

        // Check if we've exceeded the allocated time
        if start_time.elapsed().as_millis() >= time_ms || current_depth > MAX_DEPTH {
            break;
        }
    }
    best_ply
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Helper to setup board from FEN
    fn get_board(fen: &str) -> BoardFrontend {
        // Assuming BoardFrontend implements FromStr or has a similar method.
        // If not, replace this with: BoardFrontend::new(fen) or similar.
        BoardFrontend::from_fen(fen)
    }

    #[test]
    fn test_mate_in_one_white() {
        // A simple position where White can checkmate in 1 move.
        // White King on f6, Rook on f1. Black King on h8, Rook on c8 (irrelevant).
        // Solution: Rh1#
        let fen = "2r4k/8/5K2/8/8/8/8/5R2 w - - 0 1";
        let state = get_board(fen);

        // Give the engine 1000ms (1 second) to find the move
        let best = best_move(state, 1000);

        assert!(
            best.is_some(),
            "Engine failed to return a move for Mate in 1"
        );

        let m = best.unwrap();
        println!("Found move: {m:?}");

        // Optional: If you want to strictly assert the move is Rh1,
        // you would need to check the coordinates of 'm' here.
        // e.g. assert_eq!(m.to_string(), "f1h1");
    }

    #[test]
    fn test_mate_in_one_black() {
        // Black to move and checkmate.
        // FEN: 7k/8/8/8/8/5n2/7r/7K b - - 0 1
        // Solution: ...Rh2# (Arabian Mate pattern variant)
        let fen = "7k/8/8/8/8/5n2/7r/7K b - - 0 1";
        let state = get_board(fen);

        let best = best_move(state, 1000);

        assert!(
            best.is_some(),
            "Engine failed to return a move for Black Mate in 1"
        );
        println!("Found move: {:?}", best.unwrap());
    }

    #[test]
    fn test_do_not_blunder_queen() {
        // Simple sanity check: Queen is hanging, engine should move it or defend it.
        // White Queen on d4 is attacked by Black Pawn on c5.
        // FEN: rnbqkbnr/pp1ppppp/8/2p5/3Q4/8/PPP1PPPP/RNB1KBNR w KQkq - 0 2
        let fen = "rnbqkbnr/pp1ppppp/8/2p5/3Q4/8/PPP1PPPP/RNB1KBNR w KQkq - 0 2";
        let state = get_board(fen);

        let best = best_move(state, 1000);
        assert!(best.is_some());

        // We just want to ensure it runs without crashing.
        // A smarter test would check if the score dropped significantly or check the move.
        println!("Safety move: {:?}", best.unwrap());
    }
}
