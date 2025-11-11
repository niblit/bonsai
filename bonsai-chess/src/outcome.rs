//! Defines the possible outcomes of a chess game, such as a win or a draw,
//! and the specific reasons for those results.

use crate::team::Team;

/// Represents the final result of a chess game.
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
    ///     (Article 12.3.b).
    /// * The opponent persistently refuses to comply with the laws
    ///     (Article 12.8).
    /// * The opponent has an illegal, ambiguous, or unverifiable sealed
    ///     move in an adjourned game (Article 8).
    /// * The opponent arrives more than one hour late for the resumption
    ///     of an adjourned game (Article 10).
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

    // The game is a stalemate (no legal moves, king not in check).
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