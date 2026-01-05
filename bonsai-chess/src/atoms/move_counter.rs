#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MoveCounter {
    fifty_move_rule_counter: Vec<usize>,
    halfmove: usize,
    fullmove: usize,
}

impl Default for MoveCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveCounter {
    #[must_use]
    pub fn new() -> Self {
        Self {
            fifty_move_rule_counter: vec![0],
            halfmove: 0,
            fullmove: 1,
        }
    }

    #[must_use]
    pub fn from(fifty_move_rule_counter: usize, halfmove: usize, fullmove: usize) -> Self {
        Self {
            fifty_move_rule_counter: vec![fifty_move_rule_counter],
            halfmove,
            fullmove,
        }
    }

    pub fn tick(&mut self, reset_fifty_move_rule_counter: bool) {
        if reset_fifty_move_rule_counter {
            self.fifty_move_rule_counter.push(0);
        } else if let Some(count) = self.fifty_move_rule_counter.last_mut() {
            *count = count.saturating_add(1);
        }

        self.halfmove = self.halfmove.saturating_add(1);
        if self.halfmove.is_multiple_of(2) {
            self.fullmove = self.fullmove.saturating_add(1);
        }
    }

    pub fn untick(&mut self) {
        if let Some(count) = self.fifty_move_rule_counter.last_mut() {
            if *count > 0 {
                *count = count.saturating_sub(1);
            } else if self.fifty_move_rule_counter.len() > 1 {
                self.fifty_move_rule_counter.pop();
            }
        }

        self.halfmove = self.halfmove.saturating_sub(1);
        if self.halfmove.is_multiple_of(2) {
            self.fullmove = self.fullmove.saturating_sub(1);
        }
    }

    #[must_use]
    pub fn fifty_move_rule_counter(&self) -> usize {
        self.fifty_move_rule_counter.last().copied().unwrap_or(0)
    }

    #[must_use]
    pub const fn halfmove(&self) -> usize {
        self.halfmove
    }

    #[must_use]
    pub const fn fullmove(&self) -> usize {
        self.fullmove
    }
}
