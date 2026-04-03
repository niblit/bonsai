use bonsai_chess::prelude::Team;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum EngineRole {
    White,
    Black,
    Both,
    Neither,
}

impl EngineRole {
    pub fn compare_with_team(&self, other: Team) -> bool {
        match self {
            Self::White => other == Team::White,
            Self::Black => other == Team::Black,
            Self::Both => true,
            Self::Neither => false,
        }
    }
}
