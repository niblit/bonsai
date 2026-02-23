//! # Sliding Move Generator
//!
//! This module provides the shared [`slide`] function, which is the core engine
//! for generating moves for sliding pieces (Queens, Rooks, Bishops). It evaluates
//! directional rays outward from a piece's origin, halting when it encounters
//! other pieces or the edge of the board, while actively respecting pins and checks.

use crate::{
    atoms::{Coordinates, Team},
    board::BoardBackend,
    moves::{LegalityContext, Ply},
    pieces::{Kind, LocatedPiece},
};

/// Generates moves for pieces that move in straight lines (sliding pieces).
///
/// This helper function is shared by the Rook, Bishop, and Queen (and can be used
/// by the King and Knight with a limited distance of 1). It iterates outward
/// from the starting square in the specified directions until it hits the board edge
/// or another piece.
///
/// # Logic
/// * **Empty Square**: Adds the move and continues sliding further.
/// * **Enemy Piece**: Adds the capture move and *stops* (cannot jump over).
/// * **Friendly Piece**: *Stops* immediately (cannot capture or jump over).
/// * **Out of Bounds**: *Stops* immediately.
///
/// # Arguments
///
/// * `what_to_slide` - The piece moving and its starting location.
/// * `distance` - The maximum number of squares to slide (usually 7 for sliding pieces).
/// * `directions` - A list of `(row_delta, col_delta)` tuples defining the lines of movement.
/// * `backend` - The board state used to check for occupancy and captures.
/// * `context` - The pre-calculated legality constraints (pins, checks, and danger squares).
/// * `buffer` - A mutable vector where the generated, strictly legal [`Ply`] instances will be appended.
pub fn slide(
    what_to_slide: LocatedPiece,
    distance: usize,
    directions: &[(isize, isize)],
    backend: &BoardBackend,
    context: &LegalityContext,
    buffer: &mut Vec<Ply>,
) {
    let is_king = what_to_slide.piece().kind() == Kind::King;
    let start = what_to_slide.position();
    let king_position = if what_to_slide.piece().team() == Team::White {
        backend.get_white_king()
    } else {
        backend.get_black_king()
    };

    #[allow(unused_labels)]
    'direction_loop: for &(row_direction, column_direction) in directions {
        // [PIN CHECK] - If pinned, skip directions that don't match the pin ray
        if !is_king && !context.is_direction_allowed_for_pin(start, row_direction, column_direction)
        {
            continue 'direction_loop;
        }

        'distance_loop: for step in 1..=distance {
            let step_isize: isize = step.try_into().unwrap();

            let start_row_isize: isize = what_to_slide.position().row().try_into().unwrap();
            let new_row = start_row_isize + (row_direction * step_isize);

            let start_column_isize: isize = what_to_slide.position().column().try_into().unwrap();
            let new_column = start_column_isize + (column_direction * step_isize);

            if let Some(end) = Coordinates::new(new_row, new_column) {
                // [KING DANGER CHECK]
                if is_king && context.danger_squares().contains(&end) {
                    continue 'distance_loop; // Skip this square, king can't step into danger
                }

                // [CHECK RESOLUTION CHECK]
                if !is_king && !context.resolves_single_check(end, king_position, None) {
                    // Cannot land here to resolve check, but we must still check if there is a piece
                    // on this square blocking the rest of the ray from evaluating!
                    if backend.get(end).is_some() {
                        break 'distance_loop;
                    }
                    continue 'distance_loop;
                }

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
