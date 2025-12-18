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
        // Define the probe piece type and the enemy pieces that threaten via that movement path
        let checks = [
            (Kind::Pawn, &[Kind::Pawn] as &[Kind]),
            (Kind::Knight, &[Kind::Knight]),
            (Kind::Bishop, &[Kind::Bishop, Kind::Queen]),
            (Kind::Rook, &[Kind::Rook, Kind::Queen]),
            (Kind::King, &[Kind::King]),
        ];

        let check_threat = |probe_kind: Kind, threats: &[Kind]| -> bool {
            let probe = Piece::new(attacker_team.opposite(), probe_kind);
            let moves = generate_pseudo_legal_moves(
                LocatedPiece::new(probe, location),
                self,
                None,
                CastlingRights::no_rights(),
            );

            moves.into_iter().any(|m| {
                m.piece_captured().is_some_and(|captured| {
                    captured.team() == attacker_team && threats.contains(&captured.kind())
                })
            })
        };

        // Iterate through checks; returns true immediately if any check passes
        checks
            .iter()
            .any(|(probe, threats)| check_threat(*probe, threats))
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
