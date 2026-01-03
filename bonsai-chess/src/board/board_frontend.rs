use std::collections::HashMap;

use crate::{
    atoms::{CastlingRights, Coordinates, Team},
    board::{Grid, board_backend::BoardBackend},
    moves::{Ply, SpecialMove, generate_pseudo_legal_moves},
    pieces::{Kind, LocatedPiece},
    rules::{DrawReason, Outcome},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PositionSnapshot {
    pieces_positions: Grid,
    turn: Team,
    remaining_castling_rights: CastlingRights,
    en_passant: Option<Coordinates>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardFrontend {
    backend: BoardBackend,

    turn: Team,
    castling_rights: CastlingRights,
    en_passant_target: Option<Coordinates>,

    halfmove_clock: usize,
    fullmove_clock: usize,

    move_log: Vec<Ply>,

    repetition_table: HashMap<PositionSnapshot, usize>,

    outcome: Option<Outcome>,

    in_check: bool,
}

impl BoardFrontend {
    #[must_use]
    pub const fn create_snapshot(&self) -> PositionSnapshot {
        PositionSnapshot {
            pieces_positions: *self.backend.grid(),
            turn: self.turn,
            remaining_castling_rights: self.castling_rights,
            en_passant: self.en_passant_target,
        }
    }

    #[must_use]
    pub fn from_starting_position() -> Self {
        Self {
            backend: BoardBackend::from_starting_position(),
            turn: Team::White,
            castling_rights: CastlingRights::new(),
            en_passant_target: None,

            halfmove_clock: 0,
            fullmove_clock: 1,

            move_log: Vec::new(),

            repetition_table: HashMap::new(),

            outcome: None,

            in_check: false,
        }
    }

    #[must_use]
    pub fn from_fen(fen: &str) -> Self {
        todo!()
    }

    #[must_use]
    pub const fn backend(&self) -> &BoardBackend {
        &self.backend
    }

    #[must_use]
    pub fn get_legal_moves(&self) -> Vec<Ply> {
        self.get_pseudo_legal_moves()
    }

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
                self.castling_rights,
            );
            pseudo_legal_moves.append(&mut current_piece_legal_moves);
        }
        pseudo_legal_moves
    }

    pub fn make_move(&mut self, ply: &Ply) {
        // Do low level board move
        self.backend.apply_move(ply);

        // Add last move to move log
        self.move_log.push(*ply);

        // If move is a pawn double extension, keep track of en_passant possibility
        self.en_passant_target = None;
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
                self.en_passant_target = Some(en_passant_coords);
            }
        }

        // update CastlingRights
        self.update_castling_rights(ply);

        // Set turn to opponent
        self.change_turn();

        // Check if last move left opponent's king in check
        self.in_check = self.is_in_check();

        // threefold repetition
        let snapshot = self.create_snapshot();
        self.repetition_table.entry(snapshot).or_insert(0);

        if let Some(repetitions) = self.repetition_table.get_mut(&snapshot) {
            *repetitions += 1;
        }

        if self
            .repetition_table
            .values()
            .any(|repetitions| *repetitions >= 3)
            && self.outcome.is_none()
        {
            self.outcome = Some(Outcome::Draw {
                reason: DrawReason::ThreefoldRepetition,
            });
        }

        // check for dead position
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

        // check for fifty-move rule
        // TODO: refactor for better readability
        if let Some(SpecialMove::Promotion(_)) = ply.special_move() {
            self.halfmove_clock = 0;
        } else if let Some(SpecialMove::EnPassant(_)) = ply.special_move() {
            self.halfmove_clock = 0;
        } else if ply.piece_moved().kind() == Kind::Pawn || ply.piece_captured().is_some() {
            self.halfmove_clock = 0;
        } else {
            self.halfmove_clock = self.halfmove_clock.saturating_add(1);
        }

        if self.halfmove_clock >= 100 && self.outcome.is_none() {
            self.outcome = Some(Outcome::Draw {
                reason: DrawReason::FiftyMoveRule,
            });
        }
    }

    pub fn update_castling_rights(&mut self, ply: &Ply) {
        // TODO: castling rights should be tracked as a log
        if ply.piece_moved().kind() == Kind::King {
            match ply.piece_moved().team() {
                Team::White => {
                    self.castling_rights.disable_white_king_side();
                    self.castling_rights.disable_white_queen_side();
                }
                Team::Black => {
                    self.castling_rights.disable_black_king_side();
                    self.castling_rights.disable_black_queen_side();
                }
            }
        }

        let mut check_then_ban = |x, y| match (x, y) {
            (0, 0) => self.castling_rights.disable_black_queen_side(),
            (0, 7) => self.castling_rights.disable_black_king_side(),
            (7, 0) => self.castling_rights.disable_white_queen_side(),
            (7, 7) => self.castling_rights.disable_white_king_side(),
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
    }

    pub const fn change_turn(&mut self) {
        self.turn = self.turn.opposite();
    }

    /// # Panics
    ///
    /// Will panic the is no king on the board
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

    pub fn undo_last_move(&mut self) {
        todo!();
    }
}
