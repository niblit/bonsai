/// Represents the castling permissions for both players.
///
/// In chess, a player loses the right to castle if:
/// 1. The King moves (losing rights on both sides).
/// 2. The specific Rook moves (losing rights only on that side).
/// 3. The Rook is captured (losing rights only on that side).
///
/// This struct tracks these rights independently of the board state.
/// It corresponds to the "`KQkq`" portion of a FEN string.
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
    /// Useful for setting up late-game scenarios or puzzles.
    #[must_use]
    pub const fn no_rights() -> Self {
        Self {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false,
        }
    }

    /// Creates a custom set of castling rights.
    ///
    /// Useful when parsing FEN strings.
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
    pub const fn disable_white_king_side(&mut self) {
        self.white_king_side = false;
    }

    /// Revokes White's right to castle Queen-side.
    pub const fn disable_white_queen_side(&mut self) {
        self.white_queen_side = false;
    }

    /// Revokes Black's right to castle King-side.
    pub const fn disable_black_king_side(&mut self) {
        self.black_king_side = false;
    }

    /// Revokes Black's right to castle Queen-side.
    pub const fn disable_black_queen_side(&mut self) {
        self.black_queen_side = false;
    }
}
