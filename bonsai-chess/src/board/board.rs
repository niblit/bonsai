use std::collections::HashMap;

use crate::{
    board::{BoardBackend, board_backend::BoardGrid},
    castling_rights::CastlingRights,
    coordinates::Coordinates,
    outcome::Outcome,
    ply::Ply,
    team::Team,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Board<T: BoardBackend> {
    board_backend: T,
    turn: Team,
    castling_rights: CastlingRights,
    en_passant_target: Option<Coordinates>,
    halfmove_clock: usize,
    fullmove_clock: usize,

    move_log: Vec<Ply>,
    undo_log: Vec<Ply>,

    repetition_table: HashMap<BoardGrid, usize>,

    outcome: Outcome,
}

impl Board<BoardGrid> {
    pub fn from_starting_position() -> Self {
        todo!()
    }

    pub fn from_fen(fen: String) -> Self {
        todo!()
    }

    pub fn get_legal_moves(&mut self) -> Vec<Ply> {
        todo!()
    }

    pub fn make_move(&mut self) {}

    pub fn undo_last_move(&mut self) {
        todo!()
    }

    pub fn redo_move(&mut self) {
        todo!()
    }
}
