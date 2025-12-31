use std::collections::HashMap;

use crate::{
    atoms::{CastlingRights, Coordinates, Team},
    board::board_backend::BoardBackend,
    moves::{Ply, generate_pseudo_legal_moves},
    pieces::{Kind, LocatedPiece, Piece},
    rules::Outcome,
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

    in_check: bool,
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
        self.move_log.push(ply);

        self.backend.unset(ply.starting_square());
        self.backend.set(ply.piece_moved(), ply.ending_square());

        if let Some(special_move) = ply.special_move() {
            match special_move {
                crate::moves::SpecialMove::EnPassant(coordinates) => {
                    self.backend.unset(coordinates);
                }
                crate::moves::SpecialMove::Castle => {
                    // TODO: refactor to avoid magic numbers
                    let (rook_start, rook_end) = if (ply.starting_square().column() as isize
                        - ply.ending_square().column() as isize)
                        < 0
                    {
                        (
                            Coordinates::new(
                                ply.ending_square().row(),
                                ply.ending_square().column() + 1,
                            ),
                            Coordinates::new(
                                ply.ending_square().row(),
                                ply.ending_square().column() - 1,
                            ),
                        )
                    } else {
                        (
                            Coordinates::new(
                                ply.ending_square().row(),
                                ply.ending_square().column() - 2,
                            ),
                            Coordinates::new(
                                ply.ending_square().row(),
                                ply.ending_square().column() + 1,
                            ),
                        )
                    };

                    if let (Some(rook_start), Some(rook_end)) = (rook_start, rook_end) {
                        self.backend
                            .set(self.backend.get(rook_start).unwrap(), rook_end);
                        self.backend.unset(rook_start);
                    }
                }
                crate::moves::SpecialMove::Promotion(valid_promotion) => {
                    self.backend.set(
                        Piece::new(
                            ply.piece_moved().team(),
                            Kind::from_valid_promotions(valid_promotion),
                        ),
                        ply.ending_square(),
                    );
                }
            }
        }

        self.en_passant_target = None;
        if ply.piece_moved().kind() == Kind::Pawn {
            let jump_distance = ply
                .starting_square()
                .row()
                .abs_diff(ply.ending_square().row());
            if jump_distance == 2 {
                self.en_passant_target = Some(
                    Coordinates::new(
                        match ply.piece_moved().team() {
                            Team::White => ply.starting_square().row() - 1,
                            Team::Black => ply.starting_square().row() + 1,
                        },
                        ply.starting_square().column(),
                    )
                    .unwrap(),
                );
            }
        }

        // TODO: update CastlingRights
        self.change_turn();
        self.in_check = self.is_in_check();
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
        todo!()
    }

    pub fn redo_move(&mut self) {
        if let Some(last_move) = self.undo_log.pop() {
            self.make_move(last_move);
        }
    }
}
