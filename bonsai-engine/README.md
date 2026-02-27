# bonsai-engine

`bonsai-engine` is the artificial intelligence crate for the bonsai workspace. It consumes the board state and rules from `bonsai-chess` to evaluate positions and search for the best optimal moves.

## Features

* **Evaluation Heuristics**: 
  * Material counting and piece valuation.
  * Piece-Square Tables (PST) to encourage optimal piece placement.
  * Score positioning and move scoring mechanisms.
* **Search Algorithms**:
  * **Alpha-Beta Pruning**: Highly optimized minimax search to drastically reduce the number of nodes evaluated.
  * **Quiescence Search**: Extends the search depth at the end of the main search to evaluate tactical sequences (like captures) and prevent the horizon effect.
* **Transposition Tables**: Caches previously evaluated positions to prevent redundant calculations across different move orderings.
* **Opening Book**: Basic opening integrations to guide early-game decisions.

## Dependencies

* Relies heavily on `bonsai-chess` for game state generation and validation.

## License
```
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>. 
```