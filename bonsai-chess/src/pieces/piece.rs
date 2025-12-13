use crate::{atoms::Team, pieces::Kind};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Piece {
    kind: Kind,
    team: Team,
}

impl Piece {
    #[must_use]
    pub const fn new(team: Team, kind: Kind) -> Self {
        Self { kind, team }
    }

    #[must_use]
    pub const fn kind(self) -> Kind {
        self.kind
    }

    #[must_use]
    pub const fn team(self) -> Team {
        self.team
    }
}
