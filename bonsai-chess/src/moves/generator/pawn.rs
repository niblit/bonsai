use crate::{
    atoms::{Coordinates, Team},
    board::BoardBackend,
    moves::{Ply, SpecialMove},
    pieces::{Kind, LocatedPiece, Piece, ValidPromotions},
};

/// The row index where White pawns start the game (0-indexed).
/// White pawns start at row 6 and move towards row 0.
const WHITE_PAWN_STARTING_ROW: usize = 6;

/// The row index where Black pawns start the game (0-indexed).
/// Black pawns start at row 1 and move towards row 7.
const BLACK_PAWN_STARTING_ROW: usize = 1;

/// The destination row index for White pawns to promote.
const WHITE_PAWN_PROMOTION_ROW: usize = 0;

/// The destination row index for Black pawns to promote.
const BLACK_PAWN_PROMOTION_ROW: usize = 7;

/// Generates all pseudo-legal moves for a specific pawn.
///
/// This function handles the unique movement logic of pawns, including:
/// * **Pushes**: Moving one square forward (or two if on the starting rank).
/// * **Captures**: Moving diagonally to capture an enemy piece.
/// * **En Passant**: capturing a pawn that has just moved two squares.
/// * **Promotions**: Converting to a special piece upon reaching the opposite side.
///
/// # Arguments
///
/// * `what_to_move` - The pawn piece and its current coordinates on the board.
/// * `backend` - The current state of the chess board, used to check for occupancy.
/// * `en_passant_target` - The coordinate of the En Passant target square, if available.
///
/// # Returns
///
/// A vector of `Ply` representing all pseudo-legal moves available to this pawn.
pub fn pseudo_legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    en_passant_target: Option<Coordinates>,
    buffer: &mut Vec<Ply>,
) {
    // Determine move direction and critical rows based on the piece's team.
    // White moves -1 (Up), Black moves +1 (Down).
    let (direction, starting_row, promotion_row) = match what_to_move.piece().team() {
        Team::White => (-1isize, WHITE_PAWN_STARTING_ROW, WHITE_PAWN_PROMOTION_ROW),
        Team::Black => (1isize, BLACK_PAWN_STARTING_ROW, BLACK_PAWN_PROMOTION_ROW),
    };

    let current_position = what_to_move.position();

    #[allow(clippy::cast_possible_wrap)]
    let forward_row = current_position.row() as isize + direction;

    // --- 1. Forward Movement (Pushes) ---
    if let Some(one_forward_coords) = Coordinates::new(forward_row, current_position.column()) {
        // Standard Push: Target must be EMPTY (pawns cannot capture forward)
        if backend.get(one_forward_coords).is_none() {
            // A. Check for Promotion (reaching the last rank)
            if one_forward_coords.row() == promotion_row {
                for promotion in get_promotions() {
                    buffer.push(Ply::new(
                        current_position,
                        one_forward_coords,
                        what_to_move.piece(),
                        None,
                        Some(promotion),
                    ));
                }
            } else {
                // B. Standard Single Push
                buffer.push(Ply::new(
                    current_position,
                    one_forward_coords,
                    what_to_move.piece(),
                    None,
                    None,
                ));

                // C. Double Forward Push
                // Allowed only if:
                // 1. The pawn is on its starting row.
                // 2. The path is clear (one_forward is empty, checked above).
                // 3. The destination (two_forward) is empty.
                if current_position.row() == starting_row {
                    #[allow(clippy::cast_possible_wrap)]
                    let two_forward_row = current_position.row() as isize + 2 * direction;

                    if let Some(two_forward_coords) =
                        Coordinates::new(two_forward_row, current_position.column())
                        && backend.get(two_forward_coords).is_none()
                    {
                        buffer.push(Ply::new(
                            current_position,
                            two_forward_coords,
                            what_to_move.piece(),
                            None,
                            None,
                        ));
                    }
                }
            }
        }
    }

    // --- 2. Diagonal Captures ---
    // Check both diagonals: left (-1) and right (+1) relative to the pawn.
    for col_offset in [-1, 1] {
        #[allow(clippy::cast_possible_wrap)]
        let attack_col = current_position.column() as isize + col_offset;

        if let Some(capture_coords) = Coordinates::new(forward_row, attack_col) {
            // A. En Passant
            // Check if the current capture coordinate matches the en passant target.
            if let Some(available_en_passant) = en_passant_target
                && capture_coords == available_en_passant
            {
                let captured_pawn_position =
                    Coordinates::new(current_position.row(), capture_coords.column()).unwrap();
                buffer.push(Ply::new(
                    current_position,
                    capture_coords,
                    what_to_move.piece(),
                    // In En Passant, the captured piece is strictly a Pawn of the opposite team.
                    Some(Piece::new(
                        what_to_move.piece().team().opposite(),
                        Kind::Pawn,
                    )),
                    Some(SpecialMove::EnPassant(captured_pawn_position)),
                ));
            }

            // B. Standard Capture
            // Check if there is an enemy piece on the diagonal square.
            if let Some(target_piece) = backend.get(capture_coords)
                && target_piece.team() != what_to_move.piece().team()
            {
                // Capture Promotion
                if capture_coords.row() == promotion_row {
                    for promotion in get_promotions() {
                        buffer.push(Ply::new(
                            current_position,
                            capture_coords,
                            what_to_move.piece(),
                            Some(target_piece),
                            Some(promotion),
                        ));
                    }
                } else {
                    // Normal Capture
                    buffer.push(Ply::new(
                        current_position,
                        capture_coords,
                        what_to_move.piece(),
                        Some(target_piece),
                        None,
                    ));
                }
            }
        }
    }
}

/// Helper function to retrieve all valid promotion options.
///
/// Returns a vector containing `SpecialMove::Promotion` variants for:
/// * Queen
/// * Rook
/// * Bishop
/// * Knight
#[must_use]
#[inline]
fn get_promotions() -> Vec<SpecialMove> {
    vec![
        SpecialMove::Promotion(ValidPromotions::Queen),
        SpecialMove::Promotion(ValidPromotions::Rook),
        SpecialMove::Promotion(ValidPromotions::Bishop),
        SpecialMove::Promotion(ValidPromotions::Knight),
    ]
}
