# bonsai

![bonsai Logo](static/bonsai.svg)

**bonsai** is a modular chess workspace written in Rust. It aims to be a high-performance, strictly typed chess engine with clean architecture and multiple user interfaces.

The project is designed as a Cargo workspace, separating the core domain logic from the engine heuristics and user interfaces.

## Project Structure

* **[`bonsai-chess`](./bonsai-chess)**: The core library. Handles board representation, strictly typed coordinates, move generation, and rule enforcement (checkmate, stalemate, 50-move rule).
* **[`bonsai-engine`](./bonsai-engine)**: *[WIP]* The chess engine implementation, featuring evaluation functions and search algorithms (Minimax/AlphaBeta).
* **[`bonsai-cli`](./bonsai-cli)**: *[WIP]* A terminal-based user interface (TUI) for playing against the engine or analyzing positions.
* **[`bonsai-gui`](./bonsai-gui)**: *[WIP]* A graphical user interface (GUI) for the application.

## Getting Started

### Prerequisites

* **Rust**: This project targets **Rust Edition 2024**. Ensure you have the latest stable toolchain installed.
* **Nix (Optional)**: If you use Nix, a `flake.nix` is provided for a reproducible development environment.

### Development Environment

If you have `direnv` and `nix` installed, simply run:

```bash
direnv allow
```

Otherwise, install the Rust toolchain manually via rustup.

### Building
To build the entire workspace (all crates):
```bash
cargo build --release
```

### Running Components
You can run specific components of the workspace using the -p (package) flag.
1. **Performance Test (Perft)** Verify the correctness of the move generator in `bonsai-chess`:

```bash
cargo run --release -p bonsai-chess --bin perft
```

2. **Command Line Interface** Run the terminal interface (currently a skeleton):

```bash
cargo run -p bonsai-cli
```

3. **Graphical Interface** Run the GUI (currently in development):

```bash
trunk serve
```

## Status
- ✅ **Move Generation**: Complete for all pieces (Pawns, Knights, Kings, Sliding Pieces).

- ✅ **Game Rules**: Checkmate, Stalemate, Threefold Repetition, and 50-Move Rule detection are implemented.

- ✅ **Testing**: The core logic passes standard Perft (Performance Test) benchmarks.

- 🚧 **Engine**: Evaluation and Search are currently in development.

- 🚧 **Interfaces**: CLI and GUI are initialized but not yet functional.

## License
This project is licensed under the GNU General Public License v3.0 (GPL-3.0-or-later). See the **[`LICENSE`](./LICENSE)** file for details.