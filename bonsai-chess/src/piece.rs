use crate::{kind::Kind, team::Team};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Piece {
    team: Team,
    kind: Kind,
}

impl Piece {
    #[must_use]
    pub fn new(team: Team, kind: Kind) -> Self {
        Self { team, kind }
    }

    #[must_use]
    pub fn kind(&self) -> Kind {
        self.kind
    }

    #[must_use]
    pub fn team(&self) -> Team {
        self.team
    }
}
