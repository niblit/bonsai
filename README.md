# bonsai

![bonsai Logo](static/bonsai.svg)

**bonsai** is a modular chess workspace written in Rust. It aims to be a high-performance, strictly typed chess engine with clean architecture and multiple user interfaces.

The project is designed as a Cargo workspace, separating the core domain logic from the engine heuristics and the various user interfaces.

## Project Structure

* **[`bonsai-chess`](./bonsai-chess)**: The core library. Handles board representation, strictly typed coordinates, move generation, and rule enforcement.
* **[`bonsai-engine`](./bonsai-engine)**: The chess engine implementation, featuring evaluation functions, piece-square tables, and search algorithms (Alpha-Beta pruning, Quiescence search, Transposition Tables).
* **[`bonsai-cli`](./bonsai-cli)**: A terminal-based user interface (TUI) for playing against the engine or analyzing positions.
* **[`bonsai-gui`](./bonsai-gui)**: A web-based graphical user interface (GUI) built with Leptos and WebAssembly.

## Getting Started

### Prerequisites

* **Nix (Optional)**: If you use Nix, a `flake.nix` is provided for a reproducible development environment.
* **Rust**: This project targets **Rust Edition 2024** (1.93.0+). Ensure you have the latest stable toolchain installed.
* **Trunk**: Required to build and serve the web GUI (`cargo install trunk`).

### Development Environment

If you have `direnv` and `nix` installed, simply run:

```bash
direnv allow
```

Otherwise, install the Rust toolchain manually via rustup.

## Building the Workspace
To build the core libraries and the CLI:

```bash
cargo build --release
```

## Running Components
Performance Test (Perft) - Verify the correctness of the move generator:

```bash
cargo run --release -p bonsai-chess --bin perft
```


## Command Line Interface - Run the terminal interface:

```bash
cargo run --release -p bonsai-cli
```


## Graphical Interface - Run the Leptos web GUI:

```bash
cd bonsai-gui
trunk serve --open
```

## License
```
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>. 
```
