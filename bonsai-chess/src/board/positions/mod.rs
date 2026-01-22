//! Contains standard board configurations and presets.
//!
//! This module currently defines the standard FIDE starting position.
//! Future expansions could include constants for Chess960 (Fischer Random) setups,
//! specific endgame scenarios, or tactical puzzles.

use crate::{
    BOARD_COLUMNS,
    atoms::Team,
    board::Grid,
    pieces::{Kind, Piece},
};

/// The standard initial configuration of a chess board.
///
/// This constant represents the state of the board before any moves are made,
/// arranged according to FIDE Laws of Chess.
///
/// # Layout
/// * **Row 0 (Rank 8)**: Black Major Pieces (Rook, Knight, Bishop, Queen, King...)
/// * **Row 1 (Rank 7)**: Black Pawns
/// * **Rows 2-5**: Empty
/// * **Row 6 (Rank 2)**: White Pawns
/// * **Row 7 (Rank 1)**: White Major Pieces
///
/// # Usage
/// This serves as the source of truth for [`crate::board::BoardBackend::from_starting_position`].
pub const STARTING_POSITION: Grid = Grid::new([
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
]);
