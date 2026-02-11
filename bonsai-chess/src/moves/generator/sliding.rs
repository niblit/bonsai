use crate::{atoms::Coordinates, board::BoardBackend, moves::Ply, pieces::LocatedPiece};

/// Generates moves for pieces that move in straight lines (sliding pieces).
///
/// This helper function is shared by the Rook, Bishop, and Queen (and the King and Knight
/// with a limited distance of 1). It iterates outward from the starting square in the
/// specified directions until it hits the board edge or another piece.
///
/// # Logic
/// * **Empty Square**: Adds the move and continues sliding further.
/// * **Enemy Piece**: Adds the capture move and *stops* (cannot jump over).
/// * **Friendly Piece**: *Stops* immediately (cannot capture or jump over).
/// * **Out of Bounds**: *Stops* immediately.
///
/// # Arguments
///
/// * `what_to_slide`: The piece moving and its location.
/// * `distance`: The maximum number of squares to slide (usually 7 for sliders).
/// * `directions`: A list of `(row_delta, col_delta)` tuples defining the lines of movement.
/// * `backend`: The board state to check for occupancy.
pub fn slide(
    what_to_slide: LocatedPiece,
    distance: usize,
    directions: &[(isize, isize)],
    backend: &BoardBackend,
    buffer: &mut Vec<Ply>,
) {
    #[allow(unused_labels)]
    'direction_loop: for &(row_direction, column_direction) in directions {
        'distance_loop: for step in 1..=distance {
            let step_isize: isize = step.try_into().unwrap();

            let start_row_isize: isize = what_to_slide.position().row().try_into().unwrap();
            let new_row = start_row_isize + (row_direction * step_isize);

            let start_column_isize: isize = what_to_slide.position().column().try_into().unwrap();
            let new_column = start_column_isize + (column_direction * step_isize);

            if let Some(end) = Coordinates::new(new_row, new_column) {
                let target_square = backend.get(end);
                let potential_move = Ply::new(
                    what_to_slide.position(),
                    end,
                    what_to_slide.piece(),
                    target_square,
                    None,
                );
                match target_square {
                    None => buffer.push(potential_move),
                    Some(captured_piece) => {
                        if captured_piece.team() != what_to_slide.piece().team() {
                            buffer.push(potential_move);
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
}
