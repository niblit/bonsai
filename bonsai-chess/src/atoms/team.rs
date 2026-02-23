//! # Team Representation
//!
//! This module provides the [`Team`] enum, which represents the two opposing
//! sides in a standard game of chess: White and Black. It is a foundational
//! type used throughout the engine to determine turn order, piece ownership,
//! and evaluation perspectives.

/// Represents the two opposing sides in a game of chess.
///
/// This enum is used extensively to track whose turn it is to move, as well as
/// to identify the allegiance of pieces on the board.
///
/// # Examples
///
/// ```rust
/// use bonsai_chess::prelude::Team;
///
/// let player_color = Team::White;
/// assert_eq!(player_color, Team::White);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Team {
    /// The White pieces, which traditionally move first.
    White,
    /// The Black pieces, which move second.
    Black,
}

impl Team {
    /// Returns the opposing team.
    ///
    /// This is commonly used to switch turns after a move is made, or to
    /// determine enemy pieces when generating captures and evaluating positions.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::Team;
    ///
    /// let current_turn = Team::White;
    /// let next_turn = current_turn.opposite();
    ///
    /// assert_eq!(next_turn, Team::Black);
    /// assert_eq!(next_turn.opposite(), Team::White);
    /// ```
    #[must_use]
    pub const fn opposite(self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
