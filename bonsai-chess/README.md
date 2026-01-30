# bonsai-chess

`bonsai-chess` is the core library for the Bonsai chess workspace. It implements the domain logic of chess, including board representation, strictly-typed move generation, and rule enforcement (checkmate, stalemate, 50-move rule, etc.).

It is designed to be modular and type-safe, preventing invalid board states through the type system wherever possible.

## Features

* **Strongly Typed Coordinates**: Uses a `Coordinates` struct to guarantee safe array indexing, making out-of-bounds errors impossible at the logic level.
* **Dual-Layer Board Architecture**:
    * **`BoardBackend`**: Manages the raw 8x8 grid and low-level piece placement.
    * **`BoardFrontend`**: Manages high-level game state, including turn cycles, castling rights, move history, and repetition detection.
* **Complete Move Generation**:
    * Generates pseudo-legal moves for all piece types (Pawns, Knights, Kings, and Sliding pieces).
    * Validates moves against check constraints to produce strictly legal moves.
    * Handles special moves: Castling, En Passant, and Pawn Promotion.
* **Perft Tested**: Validated against standard Perft (Performance Test) positions to ensure strict adherence to move generation rules.
* **Rich Game Outcomes**: Distinguishes between various end-game states, including Checkmate, Stalemate, Threefold Repetition, Insufficient Material, and the 50-Move Rule.

## Architecture

The library is organized into the following modules:

* **`atoms`**: Fundamental types like `Square`, `Color` (Team), `CastlingRights`, and `Coordinates`.
* **`board`**: The core state containers (`BoardFrontend`, `BoardBackend`).
* **`moves`**: Move definitions (`Ply`, `SpecialMove`) and the move generator logic.
* **`pieces`**: Piece definitions (`Piece`, `Kind`) and location wrappers (`LocatedPiece`).
* **`rules`**: Enums describing game results (`Outcome`, `WinReason`, `DrawReason`).

## Usage

Add `bonsai-chess` to your dependencies, then use the `prelude` to access core types:

```rust
use bonsai_chess::prelude::*;

fn main() {
    // 1. Initialize the game with the standard starting position
    let mut game = BoardFrontend::from_starting_position();

    // 2. Generate legal moves for the current side (White)
    let legal_moves = game.get_legal_moves();
    println!("Number of legal moves: {}", legal_moves.len());

    // 3. Make a move (e.g., picking the first available one)
    if let Some(first_move) = legal_moves.first() {
        game.make_move(first_move);
        println!("Move: {first_move}");
    }

    // 4. Check game status
    if let Some(outcome) = game.outcome() {
        println!("Game over: {:?}", outcome);
    } else {
        println!("Game is ongoing.");
    }
}
```

## License
Part of the Bonsai project workspace. Licensed under GPL-3.0-or-later.
