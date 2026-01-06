use crate::{
    atoms::{CastlingRights, Coordinates, Team},
    board::BoardBackend,
    moves::{
        Ply, SpecialMove,
        generator::{
            directions::{
                DIAGONALLY_DOWN_LEFT, DIAGONALLY_DOWN_RIGHT, DIAGONALLY_UP_LEFT,
                DIAGONALLY_UP_RIGHT, DOWN, LEFT, RIGHT, UP,
            },
            sliding::slide,
        },
    },
    pieces::{Kind, LocatedPiece},
};

/// Generates pseudo-legal moves for a King.
///
/// The King moves exactly one square in any direction.
/// Additionally, if conditions are met, it can perform a special "Castling" move.
///
/// # Arguments
/// * `castling_rights`: Needed to determine if castling is even a candidate move.
pub fn pseudo_legal_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    castling_rights: CastlingRights,
) -> Vec<Ply> {
    let directions = [
        UP,
        DOWN,
        LEFT,
        RIGHT,
        DIAGONALLY_UP_LEFT,
        DIAGONALLY_UP_RIGHT,
        DIAGONALLY_DOWN_LEFT,
        DIAGONALLY_DOWN_RIGHT,
    ];
    let mut king_moves = slide(what_to_move, 1, &directions, backend);

    if castling_rights != CastlingRights::no_rights() {
        king_moves.append(&mut get_castling_moves(
            what_to_move,
            backend,
            castling_rights,
        ));
    }

    king_moves
}

/// Helper to generate Castling moves if valid.
///
/// Castling is valid only if:
/// 1. The King is not currently in check.
/// 2. The player has the right to castle (King/Rook haven't moved).
/// 3. The path between King and Rook is empty.
/// 4. The squares the King travels through (and lands on) are not under attack.
fn get_castling_moves(
    what_to_move: LocatedPiece,
    backend: &BoardBackend,
    castling_rights: CastlingRights,
) -> Vec<Ply> {
    // File Indices (0-7 for A-H)
    const FILE_B: usize = 1; // Occupancy check on long castle
    const FILE_C: usize = 2; // Long Castle Destination
    const FILE_D: usize = 3; // Long Castle Cross / Rook Dest
    const FILE_F: usize = 5; // Short Castle Cross
    const FILE_G: usize = 6; // Short Castle Destination

    const FILE_A: usize = 0; // where long side rook is
    const FILE_H: usize = 7; // where short side rook is

    let mut castling_moves = Vec::new();
    let ally = what_to_move.piece().team();
    let enemy = ally.opposite();

    // 1. Cannot castle if currently in check
    if backend.is_square_under_attack(what_to_move.position(), enemy) {
        return castling_moves;
    }

    let castling_row = match ally {
        Team::White => 7,
        Team::Black => 0,
    };

    // Helper to get Coordinate safely
    let to_coordinates =
        |column: usize| -> Coordinates { Coordinates::new(castling_row, column).unwrap() };

    // --- Kingside (Short) ---
    // Checks: Rights + Path Empty (F, G) + Path Safe (F, G)
    let can_castle_short = match ally {
        Team::White => castling_rights.white_king_side(),
        Team::Black => castling_rights.black_king_side(),
    };

    if can_castle_short {
        let f_square = to_coordinates(FILE_F);
        let g_square = to_coordinates(FILE_G);

        // You cannot go through other pieces
        let is_path_clear = backend.get(f_square).is_none() && backend.get(g_square).is_none();

        // FIDE Rule: You cannot castle *through* check.
        let is_path_safe = !backend.is_square_under_attack(f_square, enemy)
            && !backend.is_square_under_attack(g_square, enemy);

        let is_rook_in_place = backend
            .get(Coordinates::new(castling_row, FILE_H).unwrap())
            .is_some_and(|potential_rook| {
                potential_rook.kind() == Kind::Rook && potential_rook.team() == ally
            });

        if is_path_clear && is_path_safe && is_rook_in_place {
            castling_moves.push(Ply::new(
                what_to_move.position(),
                g_square,
                what_to_move.piece(),
                None,
                Some(SpecialMove::Castle),
            ));
        }
    }

    // --- Queenside (Long) ---
    // Checks: Rights + Path Empty (B, C, D) + Path Safe (C, D)
    // Note: B-file must be empty, but does NOT need to be safe from attack.
    let can_castle_long = match ally {
        Team::White => castling_rights.white_queen_side(),
        Team::Black => castling_rights.black_queen_side(),
    };

    if can_castle_long {
        let b_square = to_coordinates(FILE_B);
        let c_square = to_coordinates(FILE_C);
        let d_square = to_coordinates(FILE_D);

        // B, C, and D must be empty
        let is_path_clear = backend.get(b_square).is_none()
            && backend.get(c_square).is_none()
            && backend.get(d_square).is_none();

        // Only C and D (where King moves) must be safe
        // The Rook passes through B, but the King does not, so B need not be safe.
        let is_path_safe = !backend.is_square_under_attack(c_square, enemy)
            && !backend.is_square_under_attack(d_square, enemy);

        let is_rook_in_place = backend
            .get(Coordinates::new(castling_row, FILE_A).unwrap())
            .is_some_and(|potential_rook| {
                potential_rook.kind() == Kind::Rook && potential_rook.team() == ally
            });

        if is_path_clear && is_path_safe && is_rook_in_place {
            castling_moves.push(Ply::new(
                what_to_move.position(),
                c_square,
                what_to_move.piece(),
                None,
                Some(SpecialMove::Castle),
            ));
        }
    }

    castling_moves
}
