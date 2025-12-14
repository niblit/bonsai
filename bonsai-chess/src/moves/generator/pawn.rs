use crate::{
    atoms::{Coordinates, Team},
    board::BoardBackend,
    moves::{Ply, SpecialMove},
    pieces::{Kind, LocatedPiece, Piece, ValidPromotions},
};

const WHITE_PAWN_STARTING_ROW: usize = 6;
const BLACK_PAWN_STARTING_ROW: usize = 1;

const WHITE_PAWN_PROMOTION_ROW: usize = 0;
const BLACK_PAWN_PROMOTION_ROW: usize = 7;

pub fn pseudo_legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    en_passant_target: Option<Coordinates>,
) -> Vec<Ply> {
    let mut pawn_moves = Vec::new();
    // pawn's possible moves:
    // 1- one forward
    // 2- two forward if pawn is in the home row
    // 3- diagonally forward capture
    // 4- promotion one forward
    // 5- promotion on capture
    // 6- en passant

    // Determine direction and constants based on team
    let (direction, starting_row, promotion_row) = match what_to_move.piece().team() {
        Team::White => (-1isize, WHITE_PAWN_STARTING_ROW, WHITE_PAWN_PROMOTION_ROW),
        Team::Black => (1isize, BLACK_PAWN_STARTING_ROW, BLACK_PAWN_PROMOTION_ROW),
    };

    let current_position = what_to_move.position();

    let forward_row = (current_position.row() as isize) + direction;
    // --- 1. One Forward ---
    if let Some(one_forward_coords) =
        Coordinates::new(forward_row, current_position.column() as isize)
    {
        // Pawn Push: Target must be EMPTY (pawns cannot capture forward)
        if backend.get(one_forward_coords).is_none() {
            // --- 4. Promotion one forward ---
            if one_forward_coords.row() == promotion_row {
                for promotion in get_promotions() {
                    pawn_moves.push(Ply::new(
                        current_position,
                        one_forward_coords,
                        what_to_move.piece(),
                        None,
                        Some(promotion),
                    ));
                }
            } else {
                pawn_moves.push(Ply::new(
                    current_position,
                    one_forward_coords,
                    what_to_move.piece(),
                    None,
                    None,
                ));
            }

            // --- 2. Two Forward ---
            // This is nested here because you can only move 2 squares if:
            // a. You are on the starting row
            // b. The path is clear (meaning the 'one forward' square we just checked is empty)
            if current_position.row() == starting_row {
                let two_forward_row = (current_position.row() as isize) + 2 * direction;

                if let Some(two_forward_coords) =
                    Coordinates::new(two_forward_row, current_position.column() as isize)
                {
                    // Check if the second square is also empty
                    if backend.get(two_forward_coords).is_none() {
                        pawn_moves.push(
                            // Ply::simple(current_position, two_forward_coords)
                            Ply::new(
                                current_position,
                                two_forward_coords,
                                what_to_move.piece(),
                                None,
                                None,
                            ),
                        );
                    }
                }
            }
        }
    }

    // --- 3. Diagonal Captures (and Capture Promotions) ---
    // We check both the left (-1) and right (+1) columns relative to the pawn
    for col_offset in [-1, 1] {
        let attack_col = (current_position.column() as isize) + col_offset;

        if let Some(capture_coords) = Coordinates::new(forward_row, attack_col) {
            // --- 6. en passant ---
            if let Some(available_en_passant) = en_passant_target
                && capture_coords == available_en_passant
            {
                pawn_moves.push(Ply::new(
                    current_position,
                    capture_coords,
                    what_to_move.piece(),
                    Some(Piece::new(
                        what_to_move.piece().team().opposite(),
                        Kind::Pawn,
                    )),
                    Some(SpecialMove::EnPassant(en_passant_target.unwrap())),
                ));
            }

            // Check if there is a piece on the diagonal
            if let Some(target_piece) = backend.get(capture_coords) {
                // Essential: You can only capture enemies
                if target_piece.team() != what_to_move.piece().team() {
                    // --- 5. Promotion on capture ---
                    if capture_coords.row() == promotion_row {
                        for promotion in get_promotions() {
                            pawn_moves.push(Ply::new(
                                current_position,
                                capture_coords,
                                what_to_move.piece(),
                                Some(target_piece),
                                Some(promotion),
                            ));
                        }
                    } else {
                        // Standard Capture
                        pawn_moves.push(Ply::new(
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

    pawn_moves
}

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
