//! The `moves` module handles the definition and generation of chess moves.
//!
//! It is responsible for:
//! * **Definitions**:
//!     * [`Ply`]: A concrete move (start square -> end square).
//!     * [`SpecialMove`]: Complex mechanics like castling, en passant, and promotion.
//! * **Generation**:
//!     * [`generate_legal_moves`]: The core logic that determines where pieces can physically go.

/// Contains the logic for generating valid moves for all piece types, including sliding pieces, knights, pawns, and kings.
mod generator;

/// Defines a single half-move (ply) from a starting square to a target square.
mod ply;

/// Defines complex board mechanics such as castling, en passant, and pawn promotion.
mod special_move;

pub use generator::{LegalityContext, directions, generate_legal_moves};
pub use ply::Ply;
pub use special_move::{CastlingSide, SpecialMove};
