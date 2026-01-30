use bonsai_chess::prelude::BoardFrontend;
use bonsai_engine::best_move;

fn main() {
    let board = BoardFrontend::from_fen("k2q4/pp5p/1b3p2/8/3N1Q2/P1P1pPP1/1P1r3P/1K2R3 b - - 2 29");

    let engine_move = best_move(board, 10_000);

    if let Some(engine_move) = engine_move {
        println!("{engine_move}");
    }
}
