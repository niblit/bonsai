use crate::{
    BOARD_COLUMNS_RANGE, BOARD_ROWS_RANGE,
    atoms::{CastlingRights, Coordinates, Team},
    board::{Grid, Square, positions::STARTING_POSITION},
    moves::{directions, generate_pseudo_legal_moves, slide},
    pieces::{Kind, LocatedPiece, Piece},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BoardBackend {
    grid: Grid,
}

impl BoardBackend {
    #[must_use]
    pub const fn from_starting_position() -> Self
    where
        Self: std::marker::Sized,
    {
        Self {
            grid: STARTING_POSITION,
        }
    }

    pub const fn set(&mut self, piece: Piece, coordinates: Coordinates) {
        self.grid[coordinates.row()][coordinates.column()] = Some(piece);
    }

    pub const fn unset(&mut self, coordinates: Coordinates) {
        self.grid[coordinates.row()][coordinates.column()] = None;
    }

    #[must_use]
    pub const fn get(&self, coordinates: Coordinates) -> Square {
        self.grid[coordinates.row()][coordinates.column()]
    }

    #[must_use]
    pub fn get_white_pieces(&self) -> Vec<LocatedPiece> {
        self.filter_pieces(|p: Piece| p.team() == Team::White)
    }

    #[must_use]
    pub fn get_black_pieces(&self) -> Vec<LocatedPiece> {
        self.filter_pieces(|p: Piece| p.team() == Team::Black)
    }

    #[must_use]
    pub const fn new(grid: Grid) -> Self {
        Self { grid }
    }

    #[must_use]
    pub const fn grid(&self) -> &Grid {
        &self.grid
    }

    #[must_use]
    pub fn is_square_under_attack(&self, location: Coordinates, attacker_team: Team) -> bool {
        let pawn = Piece::new(attacker_team.opposite(), Kind::Pawn);
        let pawn_moves = generate_pseudo_legal_moves(
            LocatedPiece::new(pawn, location),
            self,
            None,
            CastlingRights::no_rights(),
        );
        for pm in pawn_moves {
            if let Some(attacker) = pm.piece_captured()
                && attacker.team() == attacker_team
                && attacker.kind() == Kind::Pawn
            {
                return true;
            }
        }

        let knight = Piece::new(attacker_team.opposite(), Kind::Knight);
        let knight_moves = generate_pseudo_legal_moves(
            LocatedPiece::new(knight, location),
            self,
            None,
            CastlingRights::no_rights(),
        );
        for km in knight_moves {
            if let Some(attacker) = km.piece_captured()
                && attacker.team() == attacker_team
                && attacker.kind() == Kind::Knight
            {
                return true;
            }
        }

        let bishop = Piece::new(attacker_team.opposite(), Kind::Bishop);
        let bishop_moves = generate_pseudo_legal_moves(
            LocatedPiece::new(bishop, location),
            self,
            None,
            CastlingRights::no_rights(),
        );
        for bm in bishop_moves {
            if let Some(attacker) = bm.piece_captured()
                && attacker.team() == attacker_team
                && (attacker.kind() == Kind::Bishop || attacker.kind() == Kind::Queen)
            {
                return true;
            }
        }

        let rook = Piece::new(attacker_team.opposite(), Kind::Rook);
        let rook_moves = generate_pseudo_legal_moves(
            LocatedPiece::new(rook, location),
            self,
            None,
            CastlingRights::no_rights(),
        );
        for rm in rook_moves {
            if let Some(attacker) = rm.piece_captured()
                && attacker.team() == attacker_team
                && (attacker.kind() == Kind::Rook || attacker.kind() == Kind::Queen)
            {
                return true;
            }
        }

        let king = Piece::new(attacker_team.opposite(), Kind::King);
        let king_moves = generate_pseudo_legal_moves(
            LocatedPiece::new(king, location),
            self,
            None,
            CastlingRights::no_rights(),
        );
        for km in king_moves {
            if let Some(attacker) = km.piece_captured()
                && attacker.team() == attacker_team
                && attacker.kind() == Kind::King
            {
                return true;
            }
        }

        false
    }

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
