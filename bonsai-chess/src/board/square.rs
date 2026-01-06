use crate::pieces::Piece;

/// Represents the content of a single cell on the chess board.
///
/// * `Some(Piece)`: The square is occupied by a piece.
/// * `None`: The square is empty.
///
/// This abstraction allows for concise checking of occupancy using standard Rust
/// `Option` combinators like `.is_some()`, `.map()`, or `match` statements.
///
/// # Examples
///
/// ```rust
/// use bonsai_chess::prelude::{Square, Piece, Kind, Team};
///
/// let empty_square: Square = None;
/// let occupied_square: Square = Some(Piece::new(Team::White, Kind::Pawn));
///
/// assert!(empty_square.is_none());
/// assert!(occupied_square.is_some());
/// ```
pub type Square = Option<Piece>;
