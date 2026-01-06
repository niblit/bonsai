use std::ops::{Add, AddAssign};

/// Aggregates statistics from a Perft (Performance Test) run.
///
/// Perft is a debugging function that traverses the move generation tree up to a
/// specified depth and counts the leaf nodes. It is used to validate the move
/// generator by comparing these counts against known correct values.
///
/// This struct tracks not just total nodes, but specific move types to help pinpoint
/// where bugs might be occurring (e.g., if the total nodes are wrong, knowing that
/// `castles` is off by 2 helps narrow down the issue).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct PerftResults {
    /// The total number of leaf nodes (positions) reached.
    pub nodes: usize,

    /// The total number of capture moves found.
    pub captures: usize,

    /// The total number of En Passant captures found.
    pub en_passant: usize,

    /// The total number of castling moves found.
    pub castles: usize,

    /// The total number of promotion moves found.
    pub promotions: usize,
}

impl PerftResults {
    /// Creates a new, empty results counter.
    #[must_use]
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

/// Allows combining results from different branches of the move tree.
///
/// This is essential for the recursive nature of Perft, where the results of
/// children nodes are summed up to form the parent's result.
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

/// Allows accumulating results in place.
impl AddAssign for PerftResults {
    fn add_assign(&mut self, rhs: Self) {
        let new = *self + rhs;

        *self = new;
    }
}
