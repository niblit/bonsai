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
        let kind = self.kind().to_string();

        let fen_piece = match self.team() {
            Team::White => kind.to_ascii_uppercase(),
            Team::Black => kind.to_ascii_lowercase(),
        };

        write!(f, "{fen_piece}")
    }
}
