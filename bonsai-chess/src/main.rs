use bonsai_chess::prelude::*;

fn perft(game: &mut BoardFrontend, depth: u32) -> u64 {
    // Base case: depth 0 implies we've reached a leaf node
    if depth == 0 {
        return 1;
    }

    let moves = game.get_legal_moves();

    // Optimization: At depth 1, just return the number of moves (count leaves directly)
    if depth == 1 {
        return moves.len() as u64;
    }

    let mut nodes = 0;

    for m in moves {
        // Clone the board to explore this branch
        // (If the API supports make/unmake, that is preferred over cloning)
        let mut next_game = game.clone();
        next_game.make_move(m);
        nodes += perft(&mut next_game, depth - 1);
    }

    nodes
}

fn main() {
    for depth in 0..=6 {
        let mut game = BoardFrontend::from_starting_position();

        let total_nodes = perft(&mut game, depth);
        println!("Perft ({depth}) result: {total_nodes}");
    }
}
