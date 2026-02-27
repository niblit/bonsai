# bonsai-chess

`bonsai-chess` is the core library for the Bonsai chess workspace. It implements the domain logic of chess, including board representation, strictly-typed move generation, and rule enforcement (checkmate, stalemate, 50-move rule, etc.).

It is designed to be modular and type-safe, preventing invalid board states through the type system wherever possible.

## Features

* **Strongly Typed Coordinates**: Uses a `Coordinates` struct to guarantee safe array indexing, making out-of-bounds errors impossible at the logic level.
* **Dual-Layer Board Architecture**:
    * **`BoardBackend`**: Manages the raw 8x8 grid and low-level piece placement.
    * **`BoardFrontend`**: Manages high-level game state, including turn cycles, castling rights, move history, and repetition detection.
* **Complete Move Generation**:
    * Generates strictly legal moves for all piece types (Pawns, Knights, Kings, and Sliding pieces).
    * Handles special moves: Castling, En Passant, and Pawn Promotion.
* **Perft Tested**: Validated against standard Perft (Performance Test) positions to ensure strict adherence to move generation rules.
* **Rich Game Outcomes**: Distinguishes between various end-game states, including Checkmate, Stalemate, Threefold Repetition, Insufficient Material, and the 50-Move Rule.

## Architecture

The library is organized into the following modules:
* **`atoms`**: Fundamental types like `Square`, `Team`, `CastlingRights`, and `Coordinates`.
* **`board`**: The core state containers (`BoardFrontend`, `BoardBackend`).
* **`moves`**: Move definitions (`Ply`, `SpecialMove`) and the move generator logic.
* **`pieces`**: Piece definitions (`Piece`, `Kind`) and location wrappers (`LocatedPiece`).
* **`rules`**: Enums describing game results (`Outcome`).

## License
```
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>. 
```