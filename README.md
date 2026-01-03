# bonsai

![bonsai Logo](static/bonsai.svg)

**bonsai** is a modular chess project written in Rust. It is organized as a workspace containing the core chess logic, an engine, and user interfaces.

## Project Structure

The project is divided into the following crates:

* **[`bonsai-chess`](./bonsai-chess)**: The core library containing game logic, board representation, piece definitions, and move generation.
* **[`bonsai-engine`](./bonsai-engine)**: My implementation of a chess engine (evaluation functions and search algorithms).
* **[`bonsai-cli`](./bonsai-cli)**: A command-line interface for interacting with the chess engine and playing games in the terminal.
* **[`bonsai-gui`](./bonsai-gui)**: A graphical user interface for the application.

## Getting Started

### Prerequisites

* [Rust](https://www.rust-lang.org/tools/install) (Edition 2024)

### Building

To build the entire workspace, run:

```bash
cargo build
```

### Running
To run a specific interface, use the -p flag:


```bash
# Run the CLI
cargo run -p bonsai-cli

# Run the GUI
cargo run -p bonsai-gui

# Run the perft from bonsai-chess
cargo run --release -p bonsai-chess --bin perft
```
## Status
This project is currently under active development.

- Core data structures (Board, Piece, Coordinates) are implemented.

- Move generation infrastructure is in place, with logic for sliding pieces partially implemented.

- Engine and Interface implementations are initializing.
