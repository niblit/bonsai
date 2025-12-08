use crate::{board::BoardGrid, coordinates::Coordinates, located_piece::LocatedPiece, ply::Ply};

pub fn slide(
    start: LocatedPiece,
    distance: usize,
    directions: &[(isize, isize)],
    backend: &BoardGrid,
) -> Vec<Ply> {
    let mut moves = Vec::new();

    'direction_loop: for &(row_direction, column_direction) in directions {
        'distance_loop: for step in 1..=distance {
            let new_row = start.position().row() as isize + (row_direction * (step as isize));
            let new_column =
                start.position().column() as isize + (column_direction * (step as isize));

            if let Some(end) = Coordinates::new(new_row, new_column) {
                let target_square = backend.get(end);
                let potential_move =
                    Ply::new(start.position(), end, start.piece(), target_square, None);
                match target_square {
                    None => moves.push(potential_move),
                    Some(captured_piece) => {
                        if captured_piece.team() != start.piece().team() {
                            moves.push(potential_move);
                        }
                        // A piece blocks further movement
                        break 'distance_loop;
                    }
                }
            } else {
                // Out of bounds
                break 'distance_loop;
            }
        }
    }

    moves
}
