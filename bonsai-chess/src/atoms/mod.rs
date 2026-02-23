//! The `atoms` module contains the fundamental building blocks of the chess game state.
//!
//! These types are "atomic" in the sense that they represent the smallest units of
//! meaningful data in the chess domain, upon which complex structures like the
//! Board and Moves are built.
//!
//! # Components
//!
//! * [`Team`]: Represents the two sides playing the game (White and Black).
//! * [`Coordinates`]: Represents a validated, strongly-typed location on the board (Ranks = Rows, Files = Columns).
//! * [`CastlingRights`]: Tracks the availability of castling for both sides.
//! * [`MoveCounter`]: Tracks turn history for rules like the 50-move rule and draw claims.

/// Tracks castling permissions for both players on the King-side and Queen-side.
mod castling_rights;

/// Provides a safe, validated representation of an 8x8 board square.
mod coordinates;

/// Tracks half-moves and full-moves for game rule enforcement (e.g., the 50-move rule).
mod move_counter;

/// Represents the two opposing sides in a game of chess (White and Black).
mod team;

pub use castling_rights::CastlingRights;
pub use coordinates::Coordinates;
pub use move_counter::MoveCounter;
pub use team::Team;
