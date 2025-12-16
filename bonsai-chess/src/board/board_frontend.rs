use std::collections::HashMap;

use crate::{
    atoms::CastlingRights, atoms::Coordinates, atoms::Team, board::board_backend::BoardBackend,
    moves::Ply, moves::generate_pseudo_legal_moves, rules::Outcome,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BoardFrontend {
    backend: BoardBackend,

    turn: Team,
    castling_rights: CastlingRights,
    en_passant_target: Option<Coordinates>,

    halfmove_clock: usize,
    fullmove_clock: usize,

    move_log: Vec<Ply>,
    undo_log: Vec<Ply>,

    repetition_table: HashMap<BoardBackend, usize>,

    outcome: Option<Outcome>,
}

impl BoardFrontend {
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
            undo_log: Vec::new(),

            repetition_table: HashMap::new(),

            outcome: None,
        }
    }

    #[must_use]
    pub fn from_fen(fen: &str) -> Self {
        todo!()
    }

    pub fn get_legal_moves(&mut self) -> Vec<Ply> {
        let mut legal_moves = Vec::new();
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
            legal_moves.append(&mut current_piece_legal_moves);
        }
        legal_moves
    }

    pub fn make_move(&mut self, ply: Ply) {
        self.undo_log.clear();
        self.move_log.push(ply);
    }

    pub fn undo_last_move(&mut self) {
        todo!()
    }

    pub fn redo_move(&mut self) {
        if let Some(last_move) = self.undo_log.pop() {
            self.make_move(last_move);
        }
    }
}
