use bonsai_chess::prelude::*;

fn main() {
    let mut game = Board::<BoardGrid>::from_starting_position();
    let moves = game.get_legal_moves();
    println!("Total pseudo legal moves is: {}", moves.len());
}
