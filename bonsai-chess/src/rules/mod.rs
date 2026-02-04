//! The `rules` module defines the final states of a chess game.
//!
//! It provides the types necessary to describe *how* a game ended, distinguishing
//! between decisive results (Wins) and Draws, and cataloging the specific FIDE
//! regulations that led to that conclusion (e.g., Checkmate vs. Resignation, or
//! Stalemate vs. Threefold Repetition).

mod outcome;
pub use outcome::{DrawReason, Outcome, WinReason};

/// The number of halfmoves (ply) without a pawn move or capture required for a player to claim a draw.
///
/// This corresponds to the standard "Fifty-Move Rule" (50 full moves * 2 sides = 100 halfmoves).
/// According to FIDE Article 9.3, the game is drawn upon a correct claim by the player having the move.
pub const CAN_CLAIM_FIFTY_MOVE_RULE_THRESHOLD: usize = 100;

/// The number of halfmoves (ply) without a pawn move or capture after which the game is drawn automatically.
///
/// This corresponds to the "Seventy-Five Move Rule" (75 full moves * 2 sides = 150 halfmoves).
/// According to FIDE Article 9.6.2, the game is drawn by the arbiter, regardless of any claim.
pub const FORCED_FIFTY_MOVE_RULE_THRESHOLD: usize = 150;

/// The number of times the exact same board position must occur for a player to claim a draw.
///
/// This corresponds to "Threefold Repetition". According to FIDE Article 9.2, the game is drawn
/// upon a correct claim by the player if the position is about to appear for the third time or has just appeared.
pub const CAN_CLAIM_THREEFOLD_REPETITION_THRESHOLD: usize = 3;

/// The number of times the exact same board position must occur for the game to be drawn automatically.
///
/// This corresponds to "Fivefold Repetition". According to FIDE Article 9.6.1, the game is drawn
/// automatically if the same position has appeared for at least five times.
pub const FORCED_THREEFOLD_REPETITION_THRESHOLD: usize = 5;
