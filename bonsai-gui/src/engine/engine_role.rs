use bonsai_chess::prelude::Side;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EngineRole {
    White,
    Black,
    Both,
    Neither,
}

impl EngineRole {
    #[must_use]
    pub fn compare_with_team(self, other: Side) -> bool {
        match self {
            Self::White => other == Side::White,
            Self::Black => other == Side::Black,
            Self::Both => true,
            Self::Neither => false,
        }
    }
}
