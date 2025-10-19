#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Team {
    White,
    Black,
}

impl Team {
    pub fn opposite(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}
