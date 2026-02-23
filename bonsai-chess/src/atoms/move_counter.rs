//! # Move Counter
//!
//! This module provides the [`MoveCounter`] struct, which is responsible for
//! tracking the progression of a chess game. It accurately counts halfmoves
//! (plies), full moves, and maintains a reversible history for the 50-move
//! draw rule.

/// Tracks the number of moves played and the state of the 50-move rule.
///
/// # Terminology
/// * **Halfmove (Ply)**: A single move by one player (e.g., White moves e4).
/// * **Fullmove**: A complete turn cycle (White moves, then Black moves).
/// * **Fifty-Move Rule**: The game can be drawn if 50 full moves (100 halfmoves)
///   are made without a pawn move or capture.
///
/// # Examples
///
/// ```rust
/// use bonsai_chess::prelude::MoveCounter;
///
/// let mut counter = MoveCounter::new();
///
/// // A standard reversible move (e.g., Nf3)
/// counter.tick(false);
/// assert_eq!(counter.halfmove(), 1);
/// assert_eq!(counter.fifty_move_rule_counter(), 1);
///
/// // An irreversible move (e.g., e4 pawn push)
/// counter.tick(true);
/// assert_eq!(counter.halfmove(), 2);
/// assert_eq!(counter.fifty_move_rule_counter(), 0); // Resets to 0
/// ```
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
    /// Creates a default `MoveCounter`, representing the start of a standard game.
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
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::MoveCounter;
    ///
    /// let counter = MoveCounter::new();
    /// assert_eq!(counter.fullmove(), 1);
    /// assert_eq!(counter.halfmove(), 0);
    /// ```
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
    /// This is typically used when restoring a game from a specific position rather
    /// than starting from the initial setup.
    ///
    /// # Arguments
    ///
    /// * `fifty_move_rule_counter` - The current count towards the 50-move rule.
    /// * `halfmove` - The total number of halfmoves played.
    /// * `fullmove` - The current full move number.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::MoveCounter;
    ///
    /// // Restoring a game at move 20
    /// let counter = MoveCounter::from(10, 39, 20);
    /// assert_eq!(counter.fullmove(), 20);
    /// ```
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
    /// This function handles the logic for advancing the halfmove counter,
    /// incrementing the fullmove counter at the correct intervals, and tracking
    /// the reversible/irreversible nature of the 50-move rule.
    ///
    /// # Arguments
    ///
    /// * `reset_fifty_move_rule_counter` - Set to `true` if the move was an "irreversible move"
    ///   (a pawn move or a capture). This pushes a `0` onto the stack. Otherwise, the current
    ///   counter at the top of the stack is incremented.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::MoveCounter;
    ///
    /// let mut counter = MoveCounter::new();
    ///
    /// // White plays a pawn move (irreversible)
    /// counter.tick(true);
    /// assert_eq!(counter.fifty_move_rule_counter(), 0);
    ///
    /// // Black plays a knight move (reversible)
    /// counter.tick(false);
    /// assert_eq!(counter.fifty_move_rule_counter(), 1);
    /// ```
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
    /// from the stack if the undone move was irreversible, or decrementing the
    /// current count if it was reversible. It also properly decrements halfmoves
    /// and fullmoves.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use bonsai_chess::prelude::MoveCounter;
    ///
    /// let mut counter = MoveCounter::new();
    /// counter.tick(false); // Make a reversible move
    ///
    /// // Undo the move
    /// counter.untick();
    /// assert_eq!(counter.halfmove(), 0);
    /// assert_eq!(counter.fifty_move_rule_counter(), 0);
    /// ```
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
    /// If this reaches 150 (75 full moves), the game ends immediately in a draw.
    ///
    /// # Returns
    ///
    /// A `usize` representing the number of halfmoves since the last pawn advance or capture.
    #[must_use]
    pub fn fifty_move_rule_counter(&self) -> usize {
        self.fifty_move_rule_counter.last().copied().unwrap_or(0)
    }

    /// Returns the total number of halfmoves played since the game began.
    ///
    /// This is strictly increasing and represents the absolute ply count.
    #[must_use]
    pub const fn halfmove(&self) -> usize {
        self.halfmove
    }

    /// Returns the current fullmove number.
    ///
    /// A fullmove consists of White's turn followed by Black's turn.
    /// This number increments after Black completes a ply.
    #[must_use]
    pub const fn fullmove(&self) -> usize {
        self.fullmove
    }
}
