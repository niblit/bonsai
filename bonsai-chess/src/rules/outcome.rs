//! Defines the possible outcomes of a chess game, such as a win or a draw,
//! and the specific reasons for those results.

use crate::atoms::Team;

/// Represents the final result of a chess game.
///
/// This enum combines the result (Win/Draw) with the specific context ([`WinReason`]/[`DrawReason`]).
///
/// # Examples
///
/// ```rust
/// use bonsai_chess::prelude::{Team, Outcome, WinReason, DrawReason};
///
/// fn describe_result(outcome: Outcome) -> String {
///     match outcome {
///         Outcome::Win { winner, reason } => {
///             format!("{:?} wins by {:?}", winner, reason)
///         },
///         Outcome::Draw { reason } => {
///             format!("Draw declared due to {:?}", reason)
///         }
///     }
/// }
///
/// let mate = Outcome::Win { winner: Team::White, reason: WinReason::Checkmate };
/// assert_eq!(describe_result(mate), "White wins by Checkmate");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Outcome {
    /// The game ended in a decisive win for one team.
    Win {
        /// The team that won the game.
        winner: Team,

        /// The specific reason for the win.
        reason: WinReason,
    },
    /// The game ended in a draw.
    Draw {
        /// The specific reason for the draw.
        reason: DrawReason,
    },
}

impl Outcome {
    /// Returns the winner if the game was won, or None if it was a draw.
    #[must_use]
    pub const fn winner(&self) -> Option<Team> {
        match self {
            Self::Win { winner, .. } => Some(*winner),
            Self::Draw { .. } => None,
        }
    }

    /// Returns the specific win reason if the game was won.
    #[must_use]
    pub const fn win_reason(&self) -> Option<WinReason> {
        match self {
            Self::Win { reason, .. } => Some(*reason),
            Self::Draw { .. } => None,
        }
    }

    /// Returns the specific draw reason if the game was a draw.
    #[must_use]
    pub const fn draw_reason(&self) -> Option<DrawReason> {
        match self {
            Self::Draw { reason } => Some(*reason),
            Self::Win { .. } => None,
        }
    }

    /// Returns true if the game ended in a win.
    #[must_use]
    pub const fn is_win(&self) -> bool {
        matches!(self, Self::Win { .. })
    }

    /// Returns true if the game ended in a draw.
    #[must_use]
    pub const fn is_draw(&self) -> bool {
        matches!(self, Self::Draw { .. })
    }
}

/// Represents the specific reason a chess game ended with a win.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum WinReason {
    /// A player's king is in check and has no legal moves to escape.
    Checkmate,

    /// A player voluntarily resigned the game.
    Resign,

    /// A player won because their opponent ran out of time (flagged).
    WinOnTime,

    /// A player was awarded the win due to a forfeit by the opponent.
    ///
    /// According to FIDE Laws, this can be declared by the arbiter for
    /// reasons including:
    /// * The opponent arrives late (Article 6.5).
    /// * The opponent completes a third illegal move (Article 7.4.b).
    /// * The opponent's mobile phone or electronic device produces a sound
    ///   (Article 12.3.b).
    /// * The opponent persistently refuses to comply with the laws
    ///   (Article 12.8).
    /// * The opponent has an illegal, ambiguous, or unverifiable sealed
    ///   move in an adjourned game (Article 8).
    /// * The opponent arrives more than one hour late for the resumption
    ///   of an adjourned game (Article 10).
    Forfeit,
}

/// Represents the specific reason a chess game ended in a draw.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DrawReason {
    /// A draw declared by forfeit.
    ///
    /// This occurs when both players commit a forfeitable offense, or when a player commits a forfeitable offense (like a third
    /// illegal move or a mobile phone violation), but their opponent
    /// does not have sufficient mating material (i.e., cannot win the
    /// game by any series of legal moves).
    ///
    /// See FIDE Articles 7.4.b and 12.3.b.
    Forfeit,

    /// The game is a stalemate (no legal moves, king not in check).
    Stalemate,

    /// The game has reached a "dead position" where checkmate is impossible
    /// (also known as insufficient material).
    DeadPosition,

    /// The players mutually agreed to a draw.
    DrawByAgreement,

    /// The same board position has occurred three times.
    ThreefoldRepetition,

    /// The game has concluded under the fifty-move rule (50 moves by
    /// each player without a capture or pawn move).
    FiftyMoveRule,

    /// A draw resulting from a timeout, such as a player running out of
    /// time when their opponent has insufficient material to mate.
    DrawOnTime,
}
