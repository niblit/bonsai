# bonsai-chess

`bonsai-chess` is the core library for the bonsai project. It handles the fundamental rules of chess, including board representation, piece movement, and game states.

## Features

* **Strongly Typed Coordinates**: Uses a `Coordinates` struct to ensure safe and valid board access, preventing out-of-bounds errors.
* **Board Abstraction**: Defines a `BoardBackend` trait, allowing for different internal representations of the chess board (e.g., `BoardGrid`).
* **Move Generation**: Includes a modular move generator.
    * Supports pseudo-legal move generation logic.
    * Specific handling for sliding pieces (Rook, Bishop, Queen) via generic direction and distance logic.
    * *Note: Move generation for Leapers (Knights, King) and Pawns is currently under construction.*
* **Game Outcomes**: Comprehensive definitions for game results, including specific win reasons (Checkmate, Resignation, Time) and draw reasons (Stalemate, Threefold Repetition, 50-Move Rule).

## Architecture

* **`Board`**: The central struct managing the game state, turn, castling rights, and move history.
* **`Piece` & `Kind`**: Enums and structs representing the standard chess pieces and their teams (White/Black).
* **`Ply`**: Represents a single half-move, tracking the start/end squares and any captured pieces or special moves (like En Passant or Castling).

## Usage

*This library is currently a work in progress. APIs may change.*

## License
Part of the Bonsai project workspace.
