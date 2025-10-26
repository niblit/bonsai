use crate::{coordinates::Coordinates, kind::Kind, piece::Piece, team::Team, BOARD_COLUMNS};

pub type GridSquare = Option<Piece>;

pub type Grid = [[GridSquare; BOARD_COLUMNS]; BOARD_COLUMNS];

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct BoardGrid {
    grid: Grid
}

impl BoardGrid {
    #[must_use]
    pub fn new(grid: Grid) -> Self {
        Self {
            grid
        }
    }

    #[must_use]
    pub fn starting_position() -> Self {
        let starting_position = [
            [
                Some(Piece::new(Team::Black, Kind::Rook)),
                Some(Piece::new(Team::Black, Kind::Knight)),
                Some(Piece::new(Team::Black, Kind::Bishop)),
                Some(Piece::new(Team::Black, Kind::Queen)),
                Some(Piece::new(Team::Black, Kind::King)),
                Some(Piece::new(Team::Black, Kind::Bishop)),
                Some(Piece::new(Team::Black, Kind::Knight)),
                Some(Piece::new(Team::Black, Kind::Rook))
            ],
            [
                Some(Piece::new(Team::Black, Kind::Pawn)); BOARD_COLUMNS
            ],
            [None; BOARD_COLUMNS],
            [None; BOARD_COLUMNS],
            [None; BOARD_COLUMNS],
            [None; BOARD_COLUMNS],
            [
                Some(Piece::new(Team::White, Kind::Pawn)); BOARD_COLUMNS
            ],
            [
                Some(Piece::new(Team::White, Kind::Rook)),
                Some(Piece::new(Team::White, Kind::Knight)),
                Some(Piece::new(Team::White, Kind::Bishop)),
                Some(Piece::new(Team::White, Kind::Queen)),
                Some(Piece::new(Team::White, Kind::King)),
                Some(Piece::new(Team::White, Kind::Bishop)),
                Some(Piece::new(Team::White, Kind::Knight)),
                Some(Piece::new(Team::White, Kind::Rook))
            ],
        ];

        Self { grid: starting_position }
    }

    #[must_use]
    pub fn grid(&self) -> Grid {
        self.grid
    }

    pub fn set(&mut self, piece: Piece, coordinates: Coordinates) {
        self.grid[coordinates.row()][coordinates.column()] = Some(piece);
    }

    pub fn unset(&mut self, coordinates: Coordinates) {
        self.grid[coordinates.row()][coordinates.column()] = None;
    }

    #[must_use]
    pub fn get(&self, coordinates: Coordinates) -> GridSquare {
        self.grid[coordinates.row()][coordinates.column()]
    }
}
