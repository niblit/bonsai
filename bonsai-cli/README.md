# bonsai-cli

`bonsai-cli` provides a terminal-based interface for interacting with the Bonsai chess engine. 

It glues together `bonsai-chess` (for game state and validation) and `bonsai-engine` (for AI opponents) into a lightweight, fast, and accessible command-line application.

## Usage

Run the CLI directly via Cargo from the root of the workspace:

```bash
cargo run --release -p bonsai-cli
```

(Currently a work-in-progress. Future updates will include UCI protocol support.)

## License
```
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>. 
```