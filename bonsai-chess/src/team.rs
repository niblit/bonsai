#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Team {
    White,
    Black,
}

impl Team {
    #[must_use]
    pub fn opposite(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::team::Team;

     #[test]
     fn double_opposite() {
        let t = Team::White;
        assert_eq!(t, t.opposite().opposite());

        let t = Team::Black;
        assert_eq!(t, t.opposite().opposite());
     }
}
