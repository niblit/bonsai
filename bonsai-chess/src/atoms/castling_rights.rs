//! # Castling Rights
//!
//! This module provides the [`CastlingRights`] struct, which is responsible for
//! tracking the availability of castling for both White and Black on both the
//! King-side and Queen-side. It is a critical component of the board's state
//! and is required for accurate move generation and FEN parsing.

/// Represents the castling permissions for both players.
///
/// In chess, a player loses the right to castle if:
/// 1. The King moves (losing rights on both sides).
/// 2. The specific Rook moves (losing rights only on that side).
/// 3. The Rook is captured (losing rights only on that side).
///
/// This struct tracks these rights independently of the board state.
/// It corresponds to the "`KQkq`" portion of a FEN string.
///
/// # Examples
///
/// ```rust
/// use bonsai_chess::prelude::CastlingRights;
///
/// // Create a default game state where everyone can castle
/// let mut rights = CastlingRights::new();
/// assert!(rights.white_king_side());
///
/// // White moves their King, losing all castling rights
/// rights.disable_white_king_side();
/// rights.disable_white_queen_side();
/// assert!(!rights.white_king_side());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::struct_excessive_bools)]
pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl Default for CastlingRights {
    /// Creates a default set of rights where castling is allowed on all sides.
    fn default() -> Self {
        Self::new()
    }
}

impl CastlingRights {
    /// Creates a new `CastlingRights` instance with all rights enabled.
    ///
    /// This is the standard state for the start of a new chess game.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::CastlingRights;
    ///
    /// let rights = CastlingRights::new();
    /// assert!(rights.black_queen_side());
    /// ```
    #[must_use]
    pub const fn new() -> Self {
        Self {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }

    /// Creates an instance with absolutely no castling rights.
    ///
    /// Useful for setting up late-game scenarios, tactical puzzles, or custom
    /// variants where castling is disabled from the start.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::CastlingRights;
    ///
    /// let rights = CastlingRights::no_rights();
    /// assert!(!rights.white_king_side());
    /// ```
    #[must_use]
    pub const fn no_rights() -> Self {
        Self {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false,
        }
    }

    /// Creates a custom set of castling rights from explicit boolean values.
    ///
    /// This constructor is particularly useful when parsing FEN strings or
    /// restoring a previous game state.
    ///
    /// # Arguments
    ///
    /// * `white_king_side` - `true` if White can castle short.
    /// * `white_queen_side` - `true` if White can castle long.
    /// * `black_king_side` - `true` if Black can castle short.
    /// * `black_queen_side` - `true` if Black can castle long.
    #[allow(clippy::fn_params_excessive_bools)]
    #[must_use]
    pub const fn from(
        white_king_side: bool,
        white_queen_side: bool,
        black_king_side: bool,
        black_queen_side: bool,
    ) -> Self {
        Self {
            white_king_side,
            white_queen_side,
            black_king_side,
            black_queen_side,
        }
    }

    /// Returns `true` if White still has the right to castle King-side.
    #[must_use]
    pub const fn white_king_side(self) -> bool {
        self.white_king_side
    }

    /// Returns `true` if White still has the right to castle Queen-side.
    #[must_use]
    pub const fn white_queen_side(self) -> bool {
        self.white_queen_side
    }

    /// Returns `true` if Black still has the right to castle King-side.
    #[must_use]
    pub const fn black_king_side(self) -> bool {
        self.black_king_side
    }

    /// Returns `true` if Black still has the right to castle Queen-side.
    #[must_use]
    pub const fn black_queen_side(self) -> bool {
        self.black_queen_side
    }

    /// Grants White the right to castle King-side.
    pub const fn enable_white_king_side(&mut self) {
        self.white_king_side = true;
    }

    /// Grants White the right to castle Queen-side.
    pub const fn enable_white_queen_side(&mut self) {
        self.white_queen_side = true;
    }

    /// Grants Black the right to castle King-side.
    pub const fn enable_black_king_side(&mut self) {
        self.black_king_side = true;
    }

    /// Grants Black the right to castle Queen-side.
    pub const fn enable_black_queen_side(&mut self) {
        self.black_queen_side = true;
    }

    /// Revokes White's right to castle King-side.
    ///
    /// Typically called when White's King or h1 Rook moves, or when the h1 Rook is captured.
    pub const fn disable_white_king_side(&mut self) {
        self.white_king_side = false;
    }

    /// Revokes White's right to castle Queen-side.
    ///
    /// Typically called when White's King or a1 Rook moves, or when the a1 Rook is captured.
    pub const fn disable_white_queen_side(&mut self) {
        self.white_queen_side = false;
    }

    /// Revokes Black's right to castle King-side.
    ///
    /// Typically called when Black's King or h8 Rook moves, or when the h8 Rook is captured.
    pub const fn disable_black_king_side(&mut self) {
        self.black_king_side = false;
    }

    /// Revokes Black's right to castle Queen-side.
    ///
    /// Typically called when Black's King or a8 Rook moves, or when the a8 Rook is captured.
    pub const fn disable_black_queen_side(&mut self) {
        self.black_queen_side = false;
    }
}
