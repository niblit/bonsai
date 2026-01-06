//! The `rules` module defines the final states of a chess game.
//!
//! It provides the types necessary to describe *how* a game ended, distinguishing
//! between decisive results (Wins) and Draws, and cataloging the specific FIDE
//! regulations that led to that conclusion (e.g., Checkmate vs. Resignation, or
//! Stalemate vs. Threefold Repetition).

mod outcome;

pub use outcome::{DrawReason, Outcome, WinReason};
