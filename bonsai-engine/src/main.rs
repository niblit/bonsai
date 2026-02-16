use bonsai_chess::prelude::BoardFrontend;
use bonsai_engine::best_move;

fn main() {
    // from lichess.org mate in 3 puzzles
    // solution is 26... Nf3+ 27. Kh1 Rg1+ 28. Rxg1 Rxg1#
    let fen = "2k3r1/ppp1n1r1/3p1p1b/1P2p1nQ/2PPP2P/P1N5/5P2/R1B2RK1 b - - 0 26";
    let board = BoardFrontend::from_fen(fen);

    let engine_move = best_move(board, 6);

    if let Some(engine_move) = engine_move {
        println!("{engine_move}");
    }
}
