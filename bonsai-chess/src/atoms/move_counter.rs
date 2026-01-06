/// Tracks the number of moves played and the state of the 50-move rule.
///
/// # Terminology
/// * **Halfmove (Ply)**: A single move by one player (e.g., White moves e4).
/// * **Fullmove**: A complete turn cycle (White moves, then Black moves).
/// * **Fifty-Move Rule**: The game can be drawn if 50 full moves (100 halfmoves)
///   are made without a pawn move or capture.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct MoveCounter {
    /// A stack tracking the reversible 50-move rule counter.
    ///
    /// We use a vector/stack because simply decrementing a counter upon undo
    /// is insufficient if the counter was reset to 0 by a capture or pawn push.
    /// We need to restore the *previous* non-zero value.
    fifty_move_rule_counter: Vec<usize>,

    /// The total number of halfmoves played since the start of the game.
    halfmove: usize,

    /// The number of the current full turn. Starts at 1.
    fullmove: usize,
}

impl Default for MoveCounter {
    fn default() -> Self {
        Self::new()
    }
}

impl MoveCounter {
    /// Creates a new `MoveCounter` starting at the beginning of a game.
    ///
    /// * Halfmove: 0
    /// * Fullmove: 1
    /// * 50-move counter: 0
    #[must_use]
    pub fn new() -> Self {
        Self {
            fifty_move_rule_counter: vec![0],
            halfmove: 0,
            fullmove: 1,
        }
    }

    /// Creates a `MoveCounter` from specific values (e.g., from a FEN string).
    ///
    /// * `fifty_move_rule_counter`: The current count towards the 50-move rule.
    /// * `halfmove`: The total number of halfmoves played.
    /// * `fullmove`: The current full move number.
    #[must_use]
    pub fn from(fifty_move_rule_counter: usize, halfmove: usize, fullmove: usize) -> Self {
        Self {
            fifty_move_rule_counter: vec![fifty_move_rule_counter],
            halfmove,
            fullmove,
        }
    }

    /// Advances the counters after a ply is made.
    ///
    /// # Arguments
    ///
    /// * `reset_fifty_move_rule_counter`: Set to `true` if the move was an "irreversible move". This pushes a `0` onto
    ///   the stack. Otherwise, the current counter is incremented.
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

    /// Reverses the counters (used during `undo_move`).
    ///
    /// This restores the previous state of the 50-move rule counter by popping
    /// from the stack or decrementing.
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

    /// Returns the current value of the fifty-move rule counter.
    ///
    /// If this reaches 100 (50 full moves), the players can claim a draw.
    /// If this reaches 150 (75 full moves), the game ends inmediately in a draw.
    #[must_use]
    pub fn fifty_move_rule_counter(&self) -> usize {
        self.fifty_move_rule_counter.last().copied().unwrap_or(0)
    }

    /// Returns the total number of halfmoves played.
    #[must_use]
    pub const fn halfmove(&self) -> usize {
        self.halfmove
    }

    /// Returns the current fullmove number.
    #[must_use]
    pub const fn fullmove(&self) -> usize {
        self.fullmove
    }
}
