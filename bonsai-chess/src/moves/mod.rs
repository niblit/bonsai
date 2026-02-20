//! The `moves` module handles the definition and generation of chess moves.
//!
//! It is responsible for:
//! * **Definitions**:
//!     * [`Ply`]: A concrete move (start square -> end square).
//!     * [`SpecialMove`]: Complex mechanics like castling, en passant, and promotion.
//! * **Generation**:
//!     * [`generate_pseudo_legal_moves`]: The core logic that determines where pieces can physically go.
mod generator;
mod ply;
mod special_move;

pub use generator::directions;
pub use generator::generate_pseudo_legal_moves;
pub use ply::Ply;
pub use special_move::{CastlingSide, SpecialMove};
