use std::time::Instant;

use bonsai_chess::prelude::*;

use crate::{
    alpha_beta_prunning::alpha_beta, openings::search_opening_book,
    transposition_table::TranspositionTable,
};

mod alpha_beta_prunning;
mod evaluation;
mod openings;
mod transposition_table;

#[must_use]
pub fn best_move(mut state: BoardFrontend, time_ms: u128) -> Option<Ply> {
    // 1. Check Opening Book first (Placeholder logic)
    if let Some(book_move) = search_opening_book(&state) {
        return Some(book_move);
    }

    // 2. Search using Iterative Deepening
    let mut best_ply = None;
    let mut current_depth = 1;
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
        if start_time.elapsed().as_millis() >= time_ms || current_depth > 100 {
            break;
        }
    }
    best_ply
}
