#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[allow(clippy::struct_excessive_bools)]
pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl Default for CastlingRights {
    fn default() -> Self {
        Self::new()
    }
}

impl CastlingRights {
    #[must_use]
    pub const fn new() -> Self {
        Self {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true,
        }
    }

    #[must_use]
    pub const fn no_rights() -> Self {
        Self {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false,
        }
    }

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

    #[must_use]
    pub const fn white_king_side(self) -> bool {
        self.white_king_side
    }

    #[must_use]
    pub const fn white_queen_side(self) -> bool {
        self.white_queen_side
    }

    #[must_use]
    pub const fn black_king_side(self) -> bool {
        self.black_king_side
    }

    #[must_use]
    pub const fn black_queen_side(self) -> bool {
        self.black_queen_side
    }

    pub const fn enable_white_king_side(&mut self) {
        self.white_king_side = true;
    }

    pub const fn enable_white_queen_side(&mut self) {
        self.white_queen_side = true;
    }

    pub const fn enable_black_king_side(&mut self) {
        self.black_king_side = true;
    }

    pub const fn enable_black_queen_side(&mut self) {
        self.black_queen_side = true;
    }

    pub const fn disable_white_king_side(&mut self) {
        self.white_king_side = false;
    }

    pub const fn disable_white_queen_side(&mut self) {
        self.white_queen_side = false;
    }

    pub const fn disable_black_king_side(&mut self) {
        self.black_king_side = false;
    }

    pub const fn disable_black_queen_side(&mut self) {
        self.black_queen_side = false;
    }
}
