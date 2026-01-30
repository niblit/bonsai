use std::{collections::HashMap, vec};

use crate::{
    atoms::{CastlingRights, Coordinates, MoveCounter, Team},
    board::{PositionSnapshot, board_backend::BoardBackend, from_fen, to_fen},
    moves::{Ply, generate_pseudo_legal_moves},
    pieces::Kind,
    rules::{
        CAN_CLAIM_FIFTY_MOVE_RULE_THRESHOLD, CAN_CLAIM_THREEFOLD_REPETITION_THRESHOLD, DrawReason,
        FORCED_FIFTY_MOVE_RULE_THRESHOLD, FORCED_THREEFOLD_REPETITION_THRESHOLD, Outcome,
        WinReason,
    },
};

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
        PositionSnapshot::new(
            *self.backend.grid(),
            self.turn,
            self.castling_rights_log
                .last()
                .copied()
                .unwrap_or(CastlingRights::no_rights()),
            self.en_passant_target,
        )
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
        if let Ok((position_snapshot, clock)) = from_fen(fen) {
            let backend = BoardBackend::new(position_snapshot.get_grid());
            Self {
                backend,
                turn: position_snapshot.get_turn(),
                castling_rights_log: vec![position_snapshot.get_castling_rights()],
                en_passant_target: position_snapshot.get_en_passant(),
                move_counter: clock,
                move_log: Vec::new(),
                repetition_table: HashMap::new(),
                outcome: None,
                in_check: backend.is_square_under_attack(
                    match position_snapshot.get_turn() {
                        Team::White => backend.get_white_king(),
                        Team::Black => backend.get_black_king(),
                    },
                    position_snapshot.get_turn().opposite(),
                ),
            }
        } else {
            Self::from_starting_position()
        }
    }

    #[must_use]
    pub fn to_fen(&self) -> String {
        to_fen(self.create_snapshot(), &self.move_counter)
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
        // According to the rules of a dead position, Article 5.2 b, when there is no possibility of checkmate for either side with any series of legal moves, the position is an immediate draw if

        //    Both Sides have a bare King
        //    One Side has a King and a Minor Piece against a bare King
        //    Both Sides have a King and a Bishop, the Bishops being the same Color
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
    #[must_use]
    pub fn is_in_check(&self) -> bool {
        // 1. Find the King
        let king_position = match self.turn {
            Team::White => self.backend.get_white_king(),
            Team::Black => self.backend.get_black_king(),
        };

        // 2. Check if that square is under attack
        self.backend
            .is_square_under_attack(king_position, self.turn.opposite())
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

    /// Gives the complete move history
    #[must_use]
    pub fn get_move_log(&self) -> Vec<Ply> {
        self.move_log.clone()
    }
}
