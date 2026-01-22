use crate::{atoms::Team, pieces::Kind};

/// Represents a standard chess piece (e.g., "White Pawn", "Black King").
///
/// This struct is the fundamental unit of the board's content. It is a lightweight,
/// `Copy` type that simply pairs a [`Team`] (color) with a [`Kind`] (rank).
///
/// It does *not* store its location; for that, see [`super::LocatedPiece`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Piece {
    kind: Kind,
    team: Team,
}

impl Piece {
    /// Creates a new piece with the specified team and kind.
    #[must_use]
    pub const fn new(team: Team, kind: Kind) -> Self {
        Self { kind, team }
    }

    /// Returns the type/rank of the piece (e.g., King, Rook).
    #[must_use]
    pub const fn kind(self) -> Kind {
        self.kind
    }

    /// Returns the team/color of the piece (e.g., White, Black).
    #[must_use]
    pub const fn team(self) -> Team {
        self.team
    }
}

/// Returns a single character representation for a piece.
/// Uppercase = White, Lowercase = Black.
impl std::fmt::Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let fen_piece = match (self.team(), self.kind()) {
            (Team::White, Kind::King) => 'K',
            (Team::White, Kind::Queen) => 'Q',
            (Team::White, Kind::Rook) => 'R',
            (Team::White, Kind::Bishop) => 'B',
            (Team::White, Kind::Knight) => 'N',
            (Team::White, Kind::Pawn) => 'P',

            (Team::Black, Kind::King) => 'k',
            (Team::Black, Kind::Queen) => 'q',
            (Team::Black, Kind::Rook) => 'r',
            (Team::Black, Kind::Bishop) => 'b',
            (Team::Black, Kind::Knight) => 'n',
            (Team::Black, Kind::Pawn) => 'p',
        };

        write!(f, "{fen_piece}")
    }
}
