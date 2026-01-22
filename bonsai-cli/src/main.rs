use bonsai_chess::prelude::*;
use std::io::{self, Write};

fn main() {
    let mut game = BoardFrontend::from_starting_position();

    loop {
        // 1. Display the board state
        println!("{}", game.backend().grid());
        println!("\n{:?} to move\n", game.turn());

        // 2. Check for game termination
        if let Some(outcome) = game.outcome() {
            println!("\nGame Over: {outcome:?}");
            break;
        }

        // 3. Generate legal moves for the current turn
        let legal_moves = game.get_legal_moves();

        // Edge case: No moves but no outcome implies an error.
        if legal_moves.is_empty() {
            eprintln!("No legal moves available.");
            break;
        }

        // 4. List possible moves in columns
        // We want 10 moves per vertical column to save vertical space.
        let rows_per_column = 10;
        let total_moves = legal_moves.len();

        // Calculate how many actual rows we need to print (max 10, or less if few moves)
        let display_rows = std::cmp::min(total_moves, rows_per_column);

        // Calculate total columns needed
        let num_columns = total_moves.div_ceil(rows_per_column);

        for row in 0..display_rows {
            for column in 0..num_columns {
                let index = row + (column * rows_per_column); // Flatten 2D columns display to a legal_moves 1D

                if index < total_moves {
                    let ply = &legal_moves[index];
                    let coordinates_info = format!(
                        "{}: {} -> {}",
                        index,
                        ply.starting_square().to_algebraic_notation(),
                        ply.ending_square().to_algebraic_notation()
                    );
                    let extra_info =
                        ply.special_move()
                            .map_or_else(String::new, |special| match special {
                                SpecialMove::Castle(castling_side) => match castling_side {
                                    CastlingSide::Short => String::from("0-0"),
                                    CastlingSide::Long => String::from("0-0-0"),
                                },
                                SpecialMove::EnPassant(_) => String::new(),
                                SpecialMove::Promotion(valid_promotions) => format!(
                                    "={}",
                                    Piece::new(
                                        Team::White,
                                        Kind::from_valid_promotions(valid_promotions)
                                    )
                                ),
                            });
                    let move_str = format!("{coordinates_info} {extra_info}");

                    // Print with padding (width 20) to align columns
                    print!("{move_str:<20}");
                }
            }
            println!(); // Newline at the end of the row
        }

        // 5. Prompt for selection
        print!("\nEnter move number (0-{}): ", legal_moves.len() - 1);
        io::stdout().flush().unwrap();

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                match input.trim().parse::<usize>() {
                    Ok(idx) if idx < legal_moves.len() => {
                        // 6. Execute the selected move
                        let selected_move = &legal_moves[idx];
                        println!(
                            "Playing: {} -> {}",
                            selected_move.starting_square().to_algebraic_notation(),
                            selected_move.ending_square().to_algebraic_notation()
                        );
                        game.make_move(selected_move);
                    }
                    _ => {
                        println!(
                            "Invalid selection. Please enter a number between 0 and {}.",
                            legal_moves.len() - 1
                        );
                    }
                }
            }
            Err(error) => {
                eprintln!("Error reading input: {error}");
                break;
            }
        }
        println!();
    }
}
