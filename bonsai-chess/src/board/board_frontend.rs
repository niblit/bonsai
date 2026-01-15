use std::collections::HashMap;

use crate::{
    atoms::{CastlingRights, Coordinates, MoveCounter, Team},
    board::{Grid, board_backend::BoardBackend},
    moves::{Ply, generate_pseudo_legal_moves},
    pieces::{Kind, LocatedPiece, Piece},
    rules::{
        CAN_CLAIM_FIFTY_MOVE_RULE_THRESHOLD, CAN_CLAIM_THREEFOLD_REPETITION_THRESHOLD, DrawReason,
        FORCED_FIFTY_MOVE_RULE_THRESHOLD, FORCED_THREEFOLD_REPETITION_THRESHOLD, Outcome,
        WinReason,
    },
};

/// A hashable representation of the board state used to detect Threefold Repetition.
///
/// This struct captures only the essential data required to uniquely identify a position
/// according to FIDE rules (piece placement, active color, castling rights, and en passant).
/// It excludes move counters or history logs.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PositionSnapshot {
    pieces_positions: Grid,
    turn: Team,
    remaining_castling_rights: CastlingRights,
    en_passant: Option<Coordinates>,
}

/// The main game controller for a chess game.
///
/// `BoardFrontend` wraps the low-level [`BoardBackend`] and enforces the rules of chess.
/// It manages:
/// * **Turn Cycle**: Whose turn it is.
/// * **Move Validation**: Generating legal moves and preventing illegal ones (like moving into check).
/// * **Game History**: Tracking moves for undo functionality and the 50-move rule.
/// * **Game Endings**: Detecting Checkmate, Stalemate, Draws (Repetition, Insufficient Material, etc.).
/// * **FEN Parsing**: Loading game states from standard notation.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardFrontend {
    /// The physical state of the board (grid).
    backend: BoardBackend,

    /// The player currently authorized to move.
    turn: Team,

    /// A history of castling rights. Used to restore rights when undoing moves.
    castling_rights_log: Vec<CastlingRights>,

    /// The specific square available for En Passant capture, if any.
    en_passant_target: Option<Coordinates>,

    /// Tracks halfmoves, fullmoves, and the 50-move rule counter.
    move_counter: MoveCounter,

    /// A stack of all moves played in the game so far.
    move_log: Vec<Ply>,

    /// Tracks how many times a specific position has occurred (for Threefold Repetition).
    repetition_table: HashMap<PositionSnapshot, usize>,

    /// The final result of the game, if it has ended.
    outcome: Option<Outcome>,

    /// Cached status indicating if the current player's King is in check.
    in_check: bool,
}

impl BoardFrontend {
    /// Creates a hashable snapshot of the current position.
    #[must_use]
    pub fn create_snapshot(&self) -> PositionSnapshot {
        PositionSnapshot {
            pieces_positions: *self.backend.grid(),
            turn: self.turn,
            remaining_castling_rights: self
                .castling_rights_log
                .last()
                .copied()
                .unwrap_or(CastlingRights::no_rights()),
            en_passant: self.en_passant_target,
        }
    }

    /// Initializes a new game with the standard chess starting position.
    #[must_use]
    pub fn from_starting_position() -> Self {
        Self {
            backend: BoardBackend::from_starting_position(),
            turn: Team::White,
            castling_rights_log: vec![CastlingRights::new()],
            en_passant_target: None,

            move_counter: MoveCounter::new(),

            move_log: Vec::new(),

            repetition_table: HashMap::new(),

            outcome: None,

            in_check: false,
        }
    }

    /// Creates a game state from a Forsyth–Edwards Notation (FEN) string.
    ///
    /// # Panics
    ///
    /// This function will panic if the provided FEN string is malformed or contains
    /// invalid characters for pieces, ranks, or files.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::BoardFrontend;
    ///
    /// // Standard start
    /// let start = BoardFrontend::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
    /// ```
    #[must_use]
    pub fn from_fen(fen: &str) -> Self {
        let parts: Vec<&str> = fen.split_whitespace().collect();

        // 1. Placement Data
        // Parses ranks 8 down to 1.
        let placement = parts.first().expect("Invalid FEN: Missing placement data");
        let mut grid: Grid = [[None; crate::BOARD_COLUMNS]; crate::BOARD_ROWS];

        for (row_index, row_str) in placement.split('/').enumerate() {
            if row_index >= crate::BOARD_ROWS {
                break;
            }

            let mut column_index = 0;
            for c in row_str.chars() {
                if let Some(skip) = c.to_digit(10) {
                    column_index += skip as usize;
                } else {
                    let team = if c.is_uppercase() {
                        Team::White
                    } else {
                        Team::Black
                    };
                    let kind = match c.to_ascii_lowercase() {
                        'p' => Kind::Pawn,
                        'n' => Kind::Knight,
                        'b' => Kind::Bishop,
                        'r' => Kind::Rook,
                        'q' => Kind::Queen,
                        'k' => Kind::King,
                        _ => panic!("Invalid FEN piece: {c}"),
                    };

                    if column_index < crate::BOARD_COLUMNS {
                        grid[row_index][column_index] = Some(Piece::new(team, kind));
                        column_index += 1;
                    }
                }
            }
        }

        let backend = BoardBackend::new(grid);

        // 2. Active Color
        let active_color = parts.get(1).unwrap_or(&"w");
        let turn = if *active_color == "w" {
            Team::White
        } else {
            Team::Black
        };

        // 3. Castling Rights
        let castling = parts.get(2).unwrap_or(&"-");
        let mut castling_rights = CastlingRights::no_rights();
        if *castling != "-" {
            if castling.contains('K') {
                castling_rights.enable_white_king_side();
            }
            if castling.contains('Q') {
                castling_rights.enable_white_queen_side();
            }
            if castling.contains('k') {
                castling_rights.enable_black_king_side();
            }
            if castling.contains('q') {
                castling_rights.enable_black_queen_side();
            }
        }

        let castling_rights_log = vec![castling_rights];

        // 4. En Passant Target
        let en_passant_str = parts.get(3).unwrap_or(&"-");
        let en_passant_target = if *en_passant_str == "-" {
            None
        } else {
            let chars: Vec<char> = en_passant_str.chars().collect();

            if chars.len() == 2 {
                let file = chars[0];
                let rank = chars[1];

                // 'a' -> 0, 'b' -> 1...
                let col = (file as usize).wrapping_sub('a' as usize);

                // FEN Rank 8 is Row 0, Rank 1 is Row 7.
                // Row = 8 - Rank.
                let row = rank
                    .to_digit(10)
                    .map_or(99, |r| crate::BOARD_ROWS.wrapping_sub(r as usize));

                Coordinates::new(row, col)
            } else {
                None
            }
        };

        // 5. Clocks
        let halfmove_clock = parts.get(4).unwrap_or(&"0").parse().unwrap_or(0);
        let fullmove_clock = parts.get(5).unwrap_or(&"1").parse().unwrap_or(1);

        let move_counter = MoveCounter::from(halfmove_clock, 0, fullmove_clock);

        let mut board = Self {
            backend,
            turn,
            castling_rights_log,
            en_passant_target,
            move_counter,
            move_log: Vec::new(),
            repetition_table: HashMap::new(),
            outcome: None,
            in_check: false,
        };

        // Initialize derived state
        board.in_check = board.is_in_check();

        let snapshot = board.create_snapshot();
        board.repetition_table.insert(snapshot, 1);

        board
    }

    /// Returns the current turn
    #[must_use]
    pub const fn turn(&self) -> Team {
        self.turn
    }

    /// Returns a reference to the low-level board backend.
    #[must_use]
    pub const fn backend(&self) -> &BoardBackend {
        &self.backend
    }

    /// Generates all fully legal moves for the current position.
    ///
    /// This process involves:
    /// 1. Generating all "pseudo-legal" moves (geometry + capture rules).
    /// 2. Filtering out moves that would leave the King in check.
    #[must_use]
    pub fn get_legal_moves(&mut self) -> Vec<Ply> {
        let mut legal_moves = Vec::new();
        let pseudo_legal_moves = self.get_pseudo_legal_moves();

        for pseudo_legal_move in pseudo_legal_moves {
            // Tentatively make the move on the backend
            self.backend.make_move(&pseudo_legal_move);

            // If the King is safe, the move is legal
            if !self.is_in_check() {
                legal_moves.push(pseudo_legal_move);
            }

            // Undo the move to restore state
            self.backend.undo_move(&pseudo_legal_move);
        }

        legal_moves
    }

    /// Generates all pseudo-legal moves for the current turn.
    ///
    /// Pseudo-legal moves satisfy piece movement rules (e.g., Bishop moves diagonally)
    /// but do not account for the safety of the King.
    #[must_use]
    pub fn get_pseudo_legal_moves(&self) -> Vec<Ply> {
        let mut pseudo_legal_moves = Vec::new();

        let pieces = match self.turn {
            Team::White => self.backend.get_white_pieces(),
            Team::Black => self.backend.get_black_pieces(),
        };

        for current_piece in pieces {
            let mut current_piece_legal_moves = generate_pseudo_legal_moves(
                current_piece,
                &self.backend,
                self.en_passant_target,
                self.castling_rights_log
                    .last()
                    .copied()
                    .unwrap_or(CastlingRights::no_rights()),
            );
            pseudo_legal_moves.append(&mut current_piece_legal_moves);
        }
        pseudo_legal_moves
    }

    /// Helper to identify if a move is a pawn double-push that enables En Passant.
    #[must_use]
    fn get_en_passant_target(ply: &Ply) -> Option<Coordinates> {
        if ply.piece_moved().kind() == Kind::Pawn {
            let jump_distance = ply
                .starting_square()
                .row()
                .abs_diff(ply.ending_square().row());
            if jump_distance == 2
                && let Some(en_passant_coords) = Coordinates::new(
                    match ply.piece_moved().team() {
                        Team::White => ply.starting_square().row() - 1,
                        Team::Black => ply.starting_square().row() + 1,
                    },
                    ply.starting_square().column(),
                )
            {
                return Some(en_passant_coords);
            }
        }
        None
    }

    /// Executes a move and updates the game state.
    ///
    /// This function handles:
    /// * Making the move on the backend.
    /// * Logging the move.
    /// * Updating Castling Rights and En Passant targets.
    /// * Switching turns.
    /// * Detecting Check, Threefold Repetition, the 50-Move Rule, and Dead Positions.
    /// * Determining the Game Outcome (Checkmate/Stalemate/Draw).
    pub fn make_move(&mut self, ply: &Ply) {
        // Cannot perform action if game is over
        if self.outcome.is_some() {
            return;
        }

        // Do low level board move
        self.backend.make_move(ply);

        // Add last move to move log
        self.move_log.push(*ply);

        // If move is a pawn double extension, keep track of en_passant possibility
        self.en_passant_target = Self::get_en_passant_target(ply);

        // update CastlingRights
        self.update_castling_rights(ply);

        // Set turn to opponent
        self.change_turn();

        // Check if last move left opponent's king in check
        self.in_check = self.is_in_check();

        // --- Draw Detection: Threefold Repetition ---
        let snapshot = self.create_snapshot();

        // Increment the repetition count for the current position
        let repetitions = self.repetition_table.entry(snapshot).or_insert(0);
        *repetitions += 1;

        // Only the current position got incremented, so check for Threefold Repetition
        if *repetitions >= FORCED_THREEFOLD_REPETITION_THRESHOLD && self.outcome.is_none() {
            self.outcome = Some(Outcome::Draw {
                reason: DrawReason::ThreefoldRepetition,
            });
        }

        // --- Draw Detection: Dead Position (Insufficient Material) ---
        // Currently only checks for King vs King.
        // TODO: Expand to K vs K+N, K vs K+B, etc.
        if self
            .backend
            .get_all_pieces()
            .iter()
            .all(|located_piece| located_piece.piece().kind() == Kind::King)
        {
            self.outcome = Some(Outcome::Draw {
                reason: DrawReason::DeadPosition,
            });
        }

        // --- Draw Detection: 50-Move Rule ---
        // The rule resets if a Pawn is moved or a capture is made.
        let is_pawn_move = ply.piece_moved().kind() == Kind::Pawn;
        let is_capture = ply.piece_captured().is_some();
        self.move_counter.tick(is_pawn_move || is_capture);

        if self.move_counter.fifty_move_rule_counter() >= FORCED_FIFTY_MOVE_RULE_THRESHOLD
            && self.outcome.is_none()
        {
            self.outcome = Some(Outcome::Draw {
                reason: DrawReason::FiftyMoveRule,
            });
        }

        // --- Win/Loss Detection: Checkmate & Stalemate ---
        // If the current player has NO legal moves...
        let legal_moves_after_move = self.get_legal_moves();
        if legal_moves_after_move.is_empty() {
            if self.is_in_check() {
                // ...and is in check -> Checkmate.
                self.outcome = Some(Outcome::Win {
                    winner: self.turn.opposite(),
                    reason: WinReason::Checkmate,
                });
            } else {
                // ...and is NOT in check -> Stalemate.
                self.outcome = Some(Outcome::Draw {
                    reason: DrawReason::Stalemate,
                });
            }
        }
    }

    /// Reverts the most recent move played.
    ///
    /// Restores the board, turn, castling rights, and move counters to their previous state.
    /// Commonly used in search algorithms (Perft, Minimax).
    pub fn undo_last_move(&mut self) {
        if let Some(last_move) = self.move_log.pop() {
            self.undo_move(&last_move);
        }
    }

    /// Internal logic for reverting a move.
    fn undo_move(&mut self, ply: &Ply) {
        // check for dead position
        self.outcome = None;

        // undo threefold repetition table
        let snapshot = self.create_snapshot();
        if let std::collections::hash_map::Entry::Occupied(mut entry) =
            self.repetition_table.entry(snapshot)
        {
            let count = entry.get_mut();
            *count -= 1;
            if *count == 0 {
                entry.remove();
            }
        }

        // Low level move
        self.backend.undo_move(ply);

        // Keep track of en en_passant_target
        if let Some(possible_pawn_move) = self.move_log.last() {
            self.en_passant_target = Self::get_en_passant_target(possible_pawn_move);
        } else {
            self.en_passant_target = None;
        }

        // reduce move MoveCounter
        self.move_counter.untick();

        // update castling_rights
        if self.castling_rights_log.len() > 1 {
            self.castling_rights_log.pop();
        }

        // set turn to opponent
        self.change_turn();

        // check for king check
        self.in_check = self.is_in_check();
    }

    /// Updates castling rights based on the move played.
    ///
    /// Disables rights if the King moves, or if a Rook moves or is captured.
    pub fn update_castling_rights(&mut self, ply: &Ply) {
        let mut castling_rights = self
            .castling_rights_log
            .last()
            .copied()
            .unwrap_or(CastlingRights::no_rights());

        if ply.piece_moved().kind() == Kind::King {
            match ply.piece_moved().team() {
                Team::White => {
                    castling_rights.disable_white_king_side();
                    castling_rights.disable_white_queen_side();
                }
                Team::Black => {
                    castling_rights.disable_black_king_side();
                    castling_rights.disable_black_queen_side();
                }
            }
        }

        let mut check_then_ban = |x, y| match (x, y) {
            (0, 0) => castling_rights.disable_black_queen_side(),
            (0, 7) => castling_rights.disable_black_king_side(),
            (7, 0) => castling_rights.disable_white_queen_side(),
            (7, 7) => castling_rights.disable_white_king_side(),
            _ => {}
        };

        if ply.piece_moved().kind() == Kind::Rook {
            check_then_ban(ply.starting_square().row(), ply.starting_square().column());
        }

        if let Some(piece_captured) = ply.piece_captured()
            && piece_captured.kind() == Kind::Rook
        {
            check_then_ban(ply.ending_square().row(), ply.ending_square().column());
        }

        self.castling_rights_log.push(castling_rights);
    }

    /// Switches the active turn to the opposite team.
    pub const fn change_turn(&mut self) {
        self.turn = self.turn.opposite();
    }

    /// Checks if the current player's King is under attack.
    ///
    /// # Panics
    ///
    /// This function will panic if there is no King of the current turn's color on the board.
    pub fn is_in_check(&self) -> bool {
        let pieces = match self.turn {
            Team::White => self.backend.get_white_pieces(),
            Team::Black => self.backend.get_black_pieces(),
        };

        // TODO: cache both kings' position
        // 1. Find the King
        let king_pos = pieces
            .iter()
            .find(|lp| lp.piece().kind() == Kind::King)
            .map(LocatedPiece::position)
            .expect("Invalid Board: The King is missing!");

        // 2. Check if that square is under attack
        self.backend
            .is_square_under_attack(king_pos, self.turn.opposite())
    }

    /// Returns the game outcome (Win, Draw, or None if ongoing).
    #[must_use]
    pub const fn outcome(&self) -> Option<Outcome> {
        self.outcome
    }

    /// Resigns the game on behalf of the specified player.
    ///
    /// This immediately ends the game, awarding the win to the opposing team.
    ///
    /// # Arguments
    ///
    /// * `resigning_player` - The team that is resigning (e.g., `Team::White`).
    pub const fn resign(&mut self, resigning_player: Team) {
        self.outcome = Some(Outcome::Win {
            winner: resigning_player.opposite(),
            reason: WinReason::Resign,
        });
    }

    /// Declares a win because the opponent ran out of time (flagged).
    ///
    /// This should be called when a player's clock hits zero and the opponent
    /// has sufficient material to checkmate. If the opponent does *not* have
    /// sufficient material, use [`draw_on_time`](Self::draw_on_time) instead.
    ///
    /// # Arguments
    ///
    /// * `flagged_player` - The team that ran out of time.
    pub const fn win_on_time(&mut self, flagged_player: Team) {
        self.outcome = Some(Outcome::Win {
            winner: flagged_player.opposite(),
            reason: WinReason::WinOnTime,
        });
    }

    /// Ends the game due to a forfeit by a specific player.
    ///
    /// A forfeit is distinct from resignation and is usually imposed by an arbiter
    /// for rule violations (e.g., cheating, arriving late, or refusing to comply with laws).
    ///
    /// # Arguments
    ///
    /// * `forfeited_player` - The team that lost the game.
    pub const fn forfeit(&mut self, forfeited_player: Team) {
        self.outcome = Some(Outcome::Win {
            winner: forfeited_player.opposite(),
            reason: WinReason::Forfeit,
        });
    }

    /// Ends the game as a draw by forfeit.
    ///
    /// This rare outcome occurs if:
    /// 1. Both players commit a forfeitable offense simultaneously.
    /// 2. One player commits a forfeitable offense, but the opponent does not have
    ///    sufficient material to mate.
    pub const fn draw_by_forfeit(&mut self) {
        self.outcome = Some(Outcome::Draw {
            reason: DrawReason::Forfeit,
        });
    }

    /// Ends the game as a draw by mutual agreement.
    ///
    /// This represents the players agreeing to a draw during the game.
    pub const fn draw_by_agreement(&mut self) {
        self.outcome = Some(Outcome::Draw {
            reason: DrawReason::DrawByAgreement,
        });
    }

    /// Ends the game as a draw because a player ran out of time.
    ///
    /// This outcome is used when a player flags (runs out of time), but their
    /// opponent cannot theoretically checkmate them (insufficient material).
    pub const fn draw_on_time(&mut self) {
        self.outcome = Some(Outcome::Draw {
            reason: DrawReason::DrawOnTime,
        });
    }

    /// Attempts to claim a draw based on the Threefold Repetition rule.
    ///
    /// According to FIDE rules (Article 9.2), a player can claim a draw if the
    /// same position has appeared for the third time (or is about to appear).
    /// Unlike automatic draws, this must be claimed by the player.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the claim is valid, the game outcome is set to `Draw`.
    /// * `Err(&str)` - If the conditions for the claim are not met.
    ///
    /// # Errors
    ///
    /// The function will return an error if the current position has not
    /// appeared at least 3 times
    pub fn claim_threefold_repetition(&mut self) -> Result<(), &'static str> {
        if !self.can_claim_threefold_repetition() {
            return Err("Cannot claim Threefold Repetition: Conditions not met.");
        }

        self.outcome = Some(Outcome::Draw {
            reason: DrawReason::ThreefoldRepetition,
        });
        Ok(())
    }

    /// Attempts to claim a draw based on the Fifty-Move Rule.
    ///
    /// According to FIDE rules (Article 9.3), a player can claim a draw if
    /// the last 50 moves have been completed by each player without the
    /// movement of any pawn and without any capture.
    /// Unlike automatic draws, this must be claimed by the player.
    ///
    /// # Returns
    ///
    /// * `Ok(())` - If the claim is valid, the game outcome is set to `Draw`.
    /// * `Err(&str)` - If the conditions for the claim are not met.
    ///
    /// # Errors
    ///
    /// The function will return an error if in last 100 plys there was an
    /// pawn move or capture
    pub fn claim_fifty_move_rule(&mut self) -> Result<(), &'static str> {
        if !self.can_claim_fifty_move_rule() {
            return Err("Cannot claim Fifty Move Rule: Conditions not met.");
        }

        self.outcome = Some(Outcome::Draw {
            reason: DrawReason::FiftyMoveRule,
        });
        Ok(())
    }

    /// Checks if the current player is eligible to claim a draw by Threefold Repetition.
    ///
    /// # Note
    ///
    /// This checks if the *current* position on the board has occurred at least 3 times.
    /// FIDE rules require the claim to be made before the player changes the position
    /// (i.e., on their turn).
    #[must_use]
    pub fn can_claim_threefold_repetition(&self) -> bool {
        // We only care if the *current* position has appeared 3+ times.
        // FIDE rules: You lose the right to claim if you change the position.
        let current_snapshot = self.create_snapshot();
        self.repetition_table
            .get(&current_snapshot)
            .is_some_and(|&count| count >= CAN_CLAIM_THREEFOLD_REPETITION_THRESHOLD)
    }

    /// Checks if the current player is eligible to claim a draw by the Fifty-Move Rule.
    #[must_use]
    pub fn can_claim_fifty_move_rule(&self) -> bool {
        self.move_counter.fifty_move_rule_counter() >= CAN_CLAIM_FIFTY_MOVE_RULE_THRESHOLD
    }
}
