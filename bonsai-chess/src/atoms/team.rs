#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
/// Represents the two opposing sides in a game of chess.
pub enum Team {
    /// The White pieces, which traditionally move first.
    White,
    /// The Black pieces, which move second.
    Black,
}

impl Team {
    /// Returns the opposing team.
    ///
    /// This is commonly used to switch turns or determine enemy pieces.
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
