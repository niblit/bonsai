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

mod castling_rights;
mod coordinates;
mod move_counter;
mod team;

pub use castling_rights::CastlingRights;
pub use coordinates::Coordinates;
pub use move_counter::MoveCounter;
pub use team::Team;
