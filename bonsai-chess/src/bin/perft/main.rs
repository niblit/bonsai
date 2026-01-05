use std::time::Instant;

use bonsai_chess::prelude::*;

mod expected;
mod perft;
mod perft_results;

use perft::perft;

use crate::expected::PERFT_EXPECTED;

fn main() {
    for (depth, &expected) in PERFT_EXPECTED.iter().enumerate() {
        let mut game = BoardFrontend::from_starting_position();

        println!("--- Depth: {depth} ---");

        let start = Instant::now();
        let result = perft(&mut game, depth);
        let end = start.elapsed();

        println!("{result:?}");
        println!("Took: {} seconds\n", end.as_secs_f64());

        assert_eq!(result, expected);
    }
}
