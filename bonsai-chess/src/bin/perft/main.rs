//! The entry point for the Perft (Performance Test) binary.
//!
//! This program verifies the correctness and performance of the move generator
//! by walking the move tree of the starting position up to a certain depth.
//! It compares the results against known correct values defined in [`expected::PERFT_EXPECTED`].

use std::time::Instant;

use bonsai_chess::prelude::*;

mod expected;
mod perft;
mod perft_results;

use crate::{expected::PERFT_EXPECTED, perft::root_level_perft};

fn main() {
    // Iterate through each depth level defined in our expected results.
    for (depth, &expected) in PERFT_EXPECTED.iter().enumerate() {
        let mut game = BoardFrontend::from_starting_position();

        println!("--- Depth: {depth} ---");

        let start = Instant::now();
        // Execute the parallelized perft search
        let result = root_level_perft(&mut game, depth);
        let end = start.elapsed();

        println!("{result:?}");
        println!("Took: {} seconds\n", end.as_secs_f64());

        // Validate the result
        assert_eq!(result, expected);
    }
}
