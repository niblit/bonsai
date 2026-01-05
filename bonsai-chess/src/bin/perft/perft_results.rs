use std::ops::{Add, AddAssign};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PerftResults {
    pub nodes: usize,
    pub captures: usize,
    pub en_passant: usize,
    pub castles: usize,
    pub promotions: usize,
}

impl PerftResults {
    pub const fn new() -> Self {
        Self {
            nodes: 0,
            captures: 0,
            en_passant: 0,
            castles: 0,
            promotions: 0,
        }
    }
}

impl Add for PerftResults {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            nodes: self.nodes + rhs.nodes,
            captures: self.captures + rhs.captures,
            en_passant: self.en_passant + rhs.en_passant,
            castles: self.castles + rhs.castles,
            promotions: self.promotions + rhs.promotions,
        }
    }
}

impl AddAssign for PerftResults {
    fn add_assign(&mut self, rhs: Self) {
        let new = *self + rhs;

        *self = new;
    }
}
