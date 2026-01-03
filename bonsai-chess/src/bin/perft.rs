use bonsai_chess::prelude::*;

fn perft(game: &BoardFrontend, depth: usize) -> usize {
    // Base case: depth 0 implies we've reached a leaf node
    if depth == 0 {
        return 1;
    }

    let moves = game.get_legal_moves();

    // Optimization: At depth 1, just return the number of moves (count leaves directly)
    if depth == 1 {
        return moves.len();
    }

    let mut number_of_nodes = 0;

    for m in moves {
        // Clone the board to explore this branch
        // (If the API supports make/unmake, that is preferred over cloning)
        let mut next_game = game.clone();
        next_game.make_move(&m);
        number_of_nodes += perft(&next_game, depth - 1);
    }

    number_of_nodes
}

fn main() {
    for depth in 0..=5 {
        let game = BoardFrontend::from_starting_position();

        let total_nodes = perft(&game, depth);
        println!("depth: {depth} nodes: {total_nodes}");
    }
}
