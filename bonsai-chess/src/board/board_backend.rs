use crate::{
    BOARD_COLUMNS, BOARD_COLUMNS_RANGE, BOARD_ROWS, BOARD_ROWS_RANGE,
    board::square::Square,
    coordinates::Coordinates,
    kind::Kind,
    located_piece::LocatedPiece,
    piece::Piece,
    team::Team,
};

pub type Grid = [[Square; BOARD_COLUMNS]; BOARD_ROWS];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BoardGrid {
    grid: Grid,
}

impl BoardGrid {
    #[must_use] 
    pub const fn from_starting_position() -> Self
    where
        Self: std::marker::Sized,
    {
        let starting_position = [
            [
                Some(Piece::new(Team::Black, Kind::Rook)),
                Some(Piece::new(Team::Black, Kind::Knight)),
                Some(Piece::new(Team::Black, Kind::Bishop)),
                Some(Piece::new(Team::Black, Kind::Queen)),
                Some(Piece::new(Team::Black, Kind::King)),
                Some(Piece::new(Team::Black, Kind::Bishop)),
                Some(Piece::new(Team::Black, Kind::Knight)),
                Some(Piece::new(Team::Black, Kind::Rook)),
            ],
            [Some(Piece::new(Team::Black, Kind::Pawn)); BOARD_COLUMNS],
            [None; BOARD_COLUMNS],
            [None; BOARD_COLUMNS],
            [None; BOARD_COLUMNS],
            [None; BOARD_COLUMNS],
            [Some(Piece::new(Team::White, Kind::Pawn)); BOARD_COLUMNS],
            [
                Some(Piece::new(Team::White, Kind::Rook)),
                Some(Piece::new(Team::White, Kind::Knight)),
                Some(Piece::new(Team::White, Kind::Bishop)),
                Some(Piece::new(Team::White, Kind::Queen)),
                Some(Piece::new(Team::White, Kind::King)),
                Some(Piece::new(Team::White, Kind::Bishop)),
                Some(Piece::new(Team::White, Kind::Knight)),
                Some(Piece::new(Team::White, Kind::Rook)),
            ],
        ];

        Self {
            grid: starting_position,
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
