use crate::{
    BOARD_COLUMNS,
    atoms::Team,
    board::Grid,
    pieces::{Kind, Piece},
};

pub const STARTING_POSITION: Grid = [
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
