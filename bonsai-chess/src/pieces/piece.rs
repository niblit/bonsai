//! # Piece Representation
//!
//! This module provides the [`Piece`] struct, representing a concrete chess piece
//! with both a specific type ([`Kind`]) and allegiance ([`Team`]). It serves as the
//! standard unit occupying squares on the chess board.

use crate::{atoms::Team, pieces::Kind};

/// Represents a standard chess piece (e.g., "White Pawn", "Black King").
///
/// This struct is the fundamental unit of the board's content. It is a lightweight,
/// `Copy` type that simply pairs a [`Team`] (color) with a [`Kind`] (rank).
///
/// It does *not* store its location; for that, see [`super::LocatedPiece`].
///
/// # Examples
///
/// ```rust
/// use bonsai_chess::prelude::{Team, Kind, Piece};
///
/// let white_knight = Piece::new(Team::White, Kind::Knight);
/// assert_eq!(white_knight.team(), Team::White);
/// assert_eq!(white_knight.kind(), Kind::Knight);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Piece {
    kind: Kind,
    team: Team,
}

impl Piece {
    /// Creates a new piece with the specified team and kind.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::{Team, Kind, Piece};
    ///
    /// let black_queen = Piece::new(Team::Black, Kind::Queen);
    /// ```
    #[must_use]
    pub const fn new(team: Team, kind: Kind) -> Self {
        Self { kind, team }
    }

    /// Returns the type/rank of the piece (e.g., King, Rook).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::{Team, Kind, Piece};
    ///
    /// let piece = Piece::new(Team::White, Kind::Bishop);
    /// assert_eq!(piece.kind(), Kind::Bishop);
    /// ```
    #[must_use]
    pub const fn kind(self) -> Kind {
        self.kind
    }

    /// Returns the team/color of the piece (e.g., White, Black).
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::{Team, Kind, Piece};
    ///
    /// let piece = Piece::new(Team::Black, Kind::Pawn);
    /// assert_eq!(piece.team(), Team::Black);
    /// ```
    #[must_use]
    pub const fn team(self) -> Team {
        self.team
    }
}

/// Returns a single character representation for a piece.
/// Uppercase = White, Lowercase = Black.
///
/// This output strictly aligns with standard Forsyth-Edwards Notation (FEN).
///
/// # Examples
///
/// ```rust
/// use bonsai_chess::prelude::{Team, Kind, Piece};
///
/// let white_king = Piece::new(Team::White, Kind::King);
/// assert_eq!(white_king.to_string(), "K");
///
/// let black_knight = Piece::new(Team::Black, Kind::Knight);
/// assert_eq!(black_knight.to_string(), "n");
/// ```
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
