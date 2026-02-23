//! # Board Backend
//!
//! This module provides the [`BoardBackend`] struct, the low-level mechanical
//! engine of the chess board. It is responsible for the raw physical placement,
//! movement, and removal of pieces on the 8x8 grid.
//!
//! It also houses the critical, high-performance ray-casting logic used to
//! determine if squares are under attack and to calculate the [`LegalityContext`]
//! (pins, checks, and danger zones) for move generation.

use crate::{
    BOARD_COLUMNS_RANGE, BOARD_ROWS_RANGE,
    atoms::{Coordinates, Team},
    board::{Grid, Square, positions::STARTING_POSITION},
    moves::{
        CastlingSide, LegalityContext, Ply, SpecialMove,
        directions::{
            DIAGONAL_DIRECTIONS, DIAGONALLY_DOWN_LEFT, DIAGONALLY_DOWN_RIGHT, DIAGONALLY_UP_LEFT,
            DIAGONALLY_UP_RIGHT, KING_DIRECTIONS, KNIGHT_DIRECTIONS, ORTHOGONAL_DIRECTIONS,
        },
    },
    pieces::{Kind, LocatedPiece, Piece},
};

/// Manages the low-level state of the chess board (the 8x8 grid).
///
/// `BoardBackend` is responsible for:
/// * Storing the position of all pieces.
/// * Executing moves on the grid (updating coordinates, clearing squares).
/// * Handling the mechanical side effects of special moves (e.g., moving the rook during a castle).
/// * Checking if squares are under attack.
///
/// It **does not** handle:
/// * Turn orders (whose move it is).
/// * Game endings (Checkmate/Stalemate).
/// * Move history logs (for 50-move rule or repetition).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BoardBackend {
    grid: Grid,
    white_king_location: Coordinates,
    black_king_location: Coordinates,
}

impl BoardBackend {
    /// Creates a new board set up with the standard chess starting position.
    ///
    /// # Panics
    ///
    /// This function will not panic, the unwrap for `Coordinates` is safe.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::BoardBackend;
    ///
    /// let backend = BoardBackend::from_starting_position();
    /// ```
    #[must_use]
    pub fn from_starting_position() -> Self {
        Self {
            grid: STARTING_POSITION,
            white_king_location: Coordinates::new(7, 4).unwrap(),
            black_king_location: Coordinates::new(0, 4).unwrap(),
        }
    }

    /// Creates a new backend from a raw grid.
    ///
    /// This function scans the grid to locate the Kings and caches their coordinates
    /// for high-performance lookups later.
    ///
    /// # Panics
    ///
    /// The backend will panic if:
    /// - One or both kings are missing.
    /// - There is more than one White King or Black King.
    #[must_use]
    pub fn new(grid: Grid) -> Self {
        let mut white_king_location = None;
        let mut black_king_location = None;

        for (row_index, row) in grid.0.iter().enumerate() {
            for (column_index, sq) in row.iter().enumerate() {
                if let Some(p) = sq {
                    let location = Coordinates::new(row_index, column_index);

                    if p.kind() == Kind::King {
                        match p.team() {
                            Team::White => {
                                if white_king_location.is_none() {
                                    white_king_location = location;
                                } else {
                                    panic!("You can only have one white king on the board")
                                }
                            }
                            Team::Black => {
                                if black_king_location.is_none() {
                                    black_king_location = location;
                                } else {
                                    panic!("You can only have one black king on the board")
                                }
                            }
                        }
                    }
                }
            }
        }

        assert!(
            white_king_location.is_some() && black_king_location.is_some(),
            "There needs to be one king on either side"
        );

        let white_king_location = white_king_location.unwrap();
        let black_king_location = black_king_location.unwrap();

        Self {
            grid,
            white_king_location,
            black_king_location,
        }
    }

    /// Updates the grid to reflect the execution of a move (`Ply`).
    ///
    /// This handles the movement of the primary piece as well as any side effects
    /// defined in [`SpecialMove`]:
    /// * **En Passant**: Removes the captured pawn (which is on a different square than the destination).
    /// * **Castle**: Moves the corresponding Rook to its new position.
    /// * **Promotion**: Replaces the pawn with the promoted piece type.
    pub fn make_move(&mut self, ply: &Ply) {
        self.unset(ply.starting_square());
        self.set(ply.piece_moved(), ply.ending_square());

        if let Some(special_move) = ply.special_move() {
            match special_move {
                // Remove the pawn that was captured en passant
                SpecialMove::EnPassant(coordinates) => {
                    self.unset(coordinates);
                }
                SpecialMove::Castle(side) => {
                    const LONG_CASTLE_ROOK_START_COLUMN: usize = 0; // (a column)
                    const LONG_CASTLE_ROOK_END_COLUMN: usize = 3; // (d column)

                    const SHORT_CASTLE_ROOK_START_COLUMN: usize = 7; // (h column)
                    const SHORT_CASTLE_ROOK_END_COLUMN: usize = 5; // (f column)

                    let (rook_start, rook_end) = match side {
                        CastlingSide::Short => (
                            Coordinates::new(
                                ply.ending_square().row(),
                                SHORT_CASTLE_ROOK_START_COLUMN,
                            ),
                            Coordinates::new(
                                ply.ending_square().row(),
                                SHORT_CASTLE_ROOK_END_COLUMN,
                            ),
                        ),
                        CastlingSide::Long => (
                            Coordinates::new(
                                ply.ending_square().row(),
                                LONG_CASTLE_ROOK_START_COLUMN,
                            ),
                            Coordinates::new(
                                ply.ending_square().row(),
                                LONG_CASTLE_ROOK_END_COLUMN,
                            ),
                        ),
                    };

                    if let (Some(rook_start), Some(rook_end)) = (rook_start, rook_end)
                        && let Some(rook_to_move) = self.get(rook_start)
                    {
                        self.set(rook_to_move, rook_end);
                        self.unset(rook_start);
                    }
                }
                SpecialMove::Promotion(valid_promotion) => {
                    self.set(
                        Piece::new(
                            ply.piece_moved().team(),
                            Kind::from_valid_promotions(valid_promotion),
                        ),
                        ply.ending_square(),
                    );
                }
            }
        }
    }

    /// Reverts the grid to the state before the provided move was made.
    ///
    /// This is critical for search algorithms (like Minimax) that explore the game tree
    /// by making and unmaking moves sequentially rather than cloning the board state.
    pub fn undo_move(&mut self, ply: &Ply) {
        // 1. Move the piece back to start
        self.set(ply.piece_moved(), ply.starting_square());

        // 2. Restore the content of the ending square
        // Check for En Passant specifically to avoid placing debris on the ending_square
        if let Some(SpecialMove::EnPassant(_)) = ply.special_move() {
            // For En Passant, the destination square must be empty after undo
            self.unset(ply.ending_square());
        } else if let Some(piece_captured) = ply.piece_captured() {
            // Standard capture: restore the piece to the square it was on
            self.set(piece_captured, ply.ending_square());
        } else {
            // Quiet move: the destination square becomes empty
            self.unset(ply.ending_square());
        }

        // 3. Handle Special Move side effects (un-castle, restore captured EP pawn)
        if let Some(special_move) = ply.special_move() {
            match special_move {
                SpecialMove::Castle(side) => {
                    // Now the constants are flipped, we use the *_ROOK_END_COLUMN as the variable rook_start because we are undoing the move
                    const LONG_CASTLE_ROOK_START_COLUMN: usize = 0; // (a column)
                    const LONG_CASTLE_ROOK_END_COLUMN: usize = 3; // (d column)

                    const SHORT_CASTLE_ROOK_START_COLUMN: usize = 7; // (h column)
                    const SHORT_CASTLE_ROOK_END_COLUMN: usize = 5; // (f column)

                    let (rook_start, rook_end) = match side {
                        CastlingSide::Short => (
                            Coordinates::new(
                                ply.ending_square().row(),
                                SHORT_CASTLE_ROOK_END_COLUMN,
                            ),
                            Coordinates::new(
                                ply.ending_square().row(),
                                SHORT_CASTLE_ROOK_START_COLUMN,
                            ),
                        ),
                        CastlingSide::Long => (
                            Coordinates::new(
                                ply.ending_square().row(),
                                LONG_CASTLE_ROOK_END_COLUMN,
                            ),
                            Coordinates::new(
                                ply.ending_square().row(),
                                LONG_CASTLE_ROOK_START_COLUMN,
                            ),
                        ),
                    };

                    if let (Some(rook_start), Some(rook_end)) = (rook_start, rook_end)
                        && let Some(rook_to_move) = self.get(rook_start)
                    {
                        // Move Rook back to original corner
                        self.set(rook_to_move, rook_end);
                        self.unset(rook_start);
                    }
                }
                SpecialMove::EnPassant(captured_pawn_coordinates) => {
                    // Put the captured pawn back where it was (not on the move path)
                    self.set(
                        Piece::new(ply.piece_moved().team().opposite(), Kind::Pawn),
                        captured_pawn_coordinates,
                    );
                }
                SpecialMove::Promotion(_valid_promotions) => {
                    // Nothing extra to do; step 1 restored the original pawn
                }
            }
        }
    }

    /// Places a piece on the board at the specified coordinates.
    ///
    /// Overwrites whatever was previously there. If the piece is a King, it also
    /// updates the cached King location.
    pub fn set(&mut self, piece: Piece, coordinates: Coordinates) {
        self.grid[coordinates.row()][coordinates.column()] = Some(piece);

        if piece.kind() == Kind::King {
            match piece.team() {
                Team::White => self.white_king_location = coordinates,
                Team::Black => self.black_king_location = coordinates,
            }
        }
    }

    /// Removes a piece from the board, leaving the square empty.
    pub fn unset(&mut self, coordinates: Coordinates) {
        self.grid[coordinates.row()][coordinates.column()] = None;
    }

    /// Retrieves the content of a square.
    #[must_use]
    pub fn get(&self, coordinates: Coordinates) -> Square {
        self.grid[coordinates.row()][coordinates.column()]
    }

    /// Returns the cached position of the White King.
    #[must_use]
    pub const fn get_white_king(&self) -> Coordinates {
        self.white_king_location
    }

    /// Returns the cached position of the Black King.
    #[must_use]
    pub const fn get_black_king(&self) -> Coordinates {
        self.black_king_location
    }

    /// Returns a list of all pieces currently on the board.
    #[must_use]
    pub fn get_all_pieces(&self) -> Vec<LocatedPiece> {
        self.filter_pieces(|_| true)
    }

    /// Returns a list of all White pieces.
    #[must_use]
    pub fn get_white_pieces(&self) -> Vec<LocatedPiece> {
        self.filter_pieces(|p: Piece| p.team() == Team::White)
    }

    /// Returns a list of all Black pieces.
    #[must_use]
    pub fn get_black_pieces(&self) -> Vec<LocatedPiece> {
        self.filter_pieces(|p: Piece| p.team() == Team::Black)
    }

    /// Returns a reference to the underlying [`Grid`].
    #[must_use]
    pub const fn grid(&self) -> &Grid {
        &self.grid
    }

    /// Determines if a specific square is under attack by the opposing team.
    ///
    /// This uses direct grid lookups (ray casting and offsets) from the target square
    /// outwards to find attackers. This completely avoids memory allocations and
    /// redundant move generation, making it highly efficient.
    ///
    /// # Arguments
    ///
    /// * `location`: The coordinate to check.
    /// * `attacker_team`: The team that might be attacking this square.
    #[must_use]
    pub fn is_square_under_attack(&self, location: Coordinates, attacker_team: Team) -> bool {
        let start_row = location.row().cast_signed();
        let start_column = location.column().cast_signed();

        for (row_delta, column_delta) in KNIGHT_DIRECTIONS {
            if let Some(target) =
                Coordinates::new(start_row + row_delta, start_column + column_delta)
                && let Some(piece) = self.get(target)
                && piece.team() == attacker_team
                && piece.kind() == Kind::Knight
            {
                return true;
            }
        }

        for (row_delta, column_delta) in KING_DIRECTIONS {
            if let Some(target) =
                Coordinates::new(start_row + row_delta, start_column + column_delta)
                && let Some(piece) = self.get(target)
                && piece.team() == attacker_team
                && piece.kind() == Kind::King
            {
                return true;
            }
        }

        // Pawn Attacks
        // Pawns attack diagonally forward. To see if we are attacked, we look diagonally backward.
        // - White pawns attack UP (-1 row), so we look DOWN (+1 row) to find them.
        // - Black pawns attack DOWN (+1 row), so we look UP (-1 row) to find them.
        let pawn_dirs = match attacker_team {
            Team::White => [DIAGONALLY_DOWN_LEFT, DIAGONALLY_DOWN_RIGHT],
            Team::Black => [DIAGONALLY_UP_LEFT, DIAGONALLY_UP_RIGHT],
        };

        for (row_delta, column_delta) in pawn_dirs {
            if let Some(target) =
                Coordinates::new(start_row + row_delta, start_column + column_delta)
                && let Some(piece) = self.get(target)
                && piece.team() == attacker_team
                && piece.kind() == Kind::Pawn
            {
                return true;
            }
        }

        for (row_delta, column_delta) in ORTHOGONAL_DIRECTIONS {
            let mut step = 1;
            while let Some(target) = Coordinates::new(
                start_row + row_delta * step,
                start_column + column_delta * step,
            ) {
                if let Some(piece) = self.get(target) {
                    if piece.team() == attacker_team
                        && (piece.kind() == Kind::Rook || piece.kind() == Kind::Queen)
                    {
                        return true;
                    }
                    break; // Blocked by a piece (friendly, or an enemy that isn't a Rook/Queen)
                }
                step += 1;
            }
        }

        for (row_delta, column_delta) in DIAGONAL_DIRECTIONS {
            let mut step = 1;
            while let Some(target) = Coordinates::new(
                start_row + row_delta * step,
                start_column + column_delta * step,
            ) {
                if let Some(piece) = self.get(target) {
                    if piece.team() == attacker_team
                        && (piece.kind() == Kind::Bishop || piece.kind() == Kind::Queen)
                    {
                        return true;
                    }
                    break; // Blocked by a piece
                }
                step += 1;
            }
        }

        false // If we made it here, no attacks were found
    }

    /// Pre-calculates the legality constraints (pins, checks, and danger squares) for the active player.
    ///
    /// This function performs complex ray-casting from the active King's position to map out
    /// the board's tactical state. By calculating this context *once* per position, the move
    /// generators can operate strictly and efficiently without doing redundant check validations.
    ///
    /// # Logic
    /// 1. **Danger Squares**: Calculates which adjacent squares the King cannot step to. It temporarily
    ///    removes the King from the board to detect x-ray attacks (where a slider attacks a square "through" the King).
    /// 2. **Pins & Sliding Checks**: Shoots rays outward from the King. The first friendly piece encountered
    ///    becomes a potential pin. If the next piece on that ray is an enemy slider (Queen/Rook/Bishop),
    ///    that friendly piece is permanently pinned. If the *first* piece encountered is an enemy slider, it's a check.
    /// 3. **Stepping Checks**: Checks Knight L-shapes and reverse Pawn attack vectors for immediate checks.
    ///
    /// # Arguments
    ///
    /// * `turn`: The team whose King is being evaluated.
    #[must_use]
    pub fn calculate_legalty_context(&self, turn: Team) -> LegalityContext {
        let king_position = match turn {
            Team::White => self.white_king_location,
            Team::Black => self.black_king_location,
        };

        let mut checkers = Vec::with_capacity(2);
        let mut pinned_pieces = Vec::with_capacity(8);
        let mut danger_squares = Vec::with_capacity(8);

        // The King itself blocks rays in `is_square_under_attack`.
        // We must temporarily remove the King to see squares attacked "through" the King.
        let mut board_without_king = *self;
        board_without_king.unset(king_position);

        // 1. Calculate danger squares around king_position
        for direction in KING_DIRECTIONS {
            if let Some(possible_danger_square) = king_position.with_offset(direction, 1)
                && board_without_king
                    .is_square_under_attack(possible_danger_square, turn.opposite())
            {
                danger_squares.push(possible_danger_square);
            }
        }

        // 2. Shoot rays to find pins and sliding checkers
        for direction in ORTHOGONAL_DIRECTIONS {
            let mut distance = 1;
            let mut current_pin = None;
            while let Some(target) = king_position.with_offset(direction, distance) {
                if let Some(some_piece) = self.get(target) {
                    if some_piece.team() == turn {
                        match current_pin {
                            Some(_) => {
                                break;
                            }
                            None => current_pin = Some(target),
                        }
                    } else if some_piece.kind() == Kind::Queen || some_piece.kind() == Kind::Rook {
                        if let Some(pinned) = current_pin {
                            pinned_pieces.push((pinned, direction));
                            break;
                        }
                        checkers.push(target);
                        break;
                    } else {
                        break;
                    }
                }
                distance += 1;
            }
        }

        for direction in DIAGONAL_DIRECTIONS {
            let mut distance = 1;
            let mut current_pin = None;
            while let Some(target) = king_position.with_offset(direction, distance) {
                if let Some(some_piece) = self.get(target) {
                    if some_piece.team() == turn {
                        match current_pin {
                            Some(_) => {
                                break;
                            }
                            None => current_pin = Some(target),
                        }
                    } else if some_piece.kind() == Kind::Queen || some_piece.kind() == Kind::Bishop
                    {
                        if let Some(pinned) = current_pin {
                            pinned_pieces.push((pinned, direction));
                            break;
                        }
                        checkers.push(target);
                        break;
                    } else {
                        break;
                    }
                }
                distance += 1;
            }
        }

        // 3. Check knight/pawn offsets to find stepping checkers
        for direction in KNIGHT_DIRECTIONS {
            if let Some(target) = king_position.with_offset(direction, 1)
                && let Some(piece) = self.get(target)
                && piece.team() == turn.opposite()
                && piece.kind() == Kind::Knight
            {
                checkers.push(target);
            }
        }

        // Pawn Attacks
        // Pawns attack diagonally forward. To see if we are attacked, we look diagonally backward.
        // - White pawns attack UP (-1 row), so we look DOWN (+1 row) to find them.
        // - Black pawns attack DOWN (+1 row), so we look UP (-1 row) to find them.
        let pawn_dirs = match turn.opposite() {
            Team::White => [DIAGONALLY_DOWN_LEFT, DIAGONALLY_DOWN_RIGHT],
            Team::Black => [DIAGONALLY_UP_LEFT, DIAGONALLY_UP_RIGHT],
        };

        for direction in pawn_dirs {
            if let Some(target) = king_position.with_offset(direction, 1)
                && let Some(piece) = self.get(target)
                && piece.team() == turn.opposite()
                && piece.kind() == Kind::Pawn
            {
                checkers.push(target);
            }
        }

        LegalityContext::from(checkers, pinned_pieces, danger_squares)
    }

    /// Helper to collect pieces matching a filter predicate.
    #[must_use]
    #[inline]
    fn filter_pieces(&self, filter: impl Fn(Piece) -> bool) -> Vec<LocatedPiece> {
        let mut filtered_pieces = Vec::new();
        for row in BOARD_ROWS_RANGE {
            for column in BOARD_COLUMNS_RANGE {
                if let Some(current) = self.grid[row][column]
                    && filter(current)
                {
                    let location = Coordinates::new(row, column).expect("Board iteration produced invalid coordinates, either BOARD_ROWS_RANGE or BOARD_COLUMNS_RANGE is not correctly defined");

                    let located_piece = LocatedPiece::new(current, location);

                    filtered_pieces.push(located_piece);
                }
            }
        }

        filtered_pieces
    }
}
