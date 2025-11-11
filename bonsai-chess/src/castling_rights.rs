#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct CastlingRights {
    white_king_side: bool,
    white_queen_side: bool,
    black_king_side: bool,
    black_queen_side: bool,
}

impl CastlingRights {
    pub fn new() -> Self {
        Self {
            white_king_side: true,
            white_queen_side: true,
            black_king_side: true,
            black_queen_side: true
        }
    }

    pub fn no_rights() -> Self {
        Self {
            white_king_side: false,
            white_queen_side: false,
            black_king_side: false,
            black_queen_side: false
        }
    }

    pub fn from(white_king_side: bool, white_queen_side: bool, black_king_side: bool, black_queen_side: bool) -> Self {
        Self {
            white_king_side,
            white_queen_side,
            black_king_side,
            black_queen_side
        }
    }

    pub fn white_king_side(&self) -> bool {
        self.white_king_side
    }

    pub fn white_queen_side(&self) -> bool {
        self.white_queen_side
    }

    pub fn black_king_side(&self) -> bool {
        self.black_king_side
    }

    pub fn black_queen_side(&self) -> bool {
        self.black_queen_side
    }

    pub fn enable_white_king_side(&mut self) {
        self.white_king_side = true;
    }

    pub fn enable_white_queen_side(&mut self) {
        self.white_queen_side = true;
    }

    pub fn enable_black_king_side(&mut self) {
        self.black_king_side = true;
    }

    pub fn enable_black_queen_side(&mut self) {
        self.black_queen_side = true;
    }

    pub fn disable_white_king_side(&mut self) {
        self.white_king_side = false;
    }

    pub fn disable_white_queen_side(&mut self) {
        self.white_queen_side = false;
    }

    pub fn disable_black_king_side(&mut self) {
        self.black_king_side = false;
    }

    pub fn disable_black_queen_side(&mut self) {
        self.black_queen_side = false;
    }
}