# bonsai-gui

`bonsai-gui` is a reactive, web-based graphical user interface for the Bonsai chess engine. It is written entirely in Rust and compiles to WebAssembly (Wasm) using Leptos.

## Features

* **Client-Side Rendering (CSR)**: Fast, reactive UI built with Leptos.
* **Interactive Board**: Click-to-move piece interactions and highlights.
* **Engine Integration**: Plays directly against `bonsai-engine` running entirely in the browser via WebAssembly.
* **Game History & Controls**: Sidebar interface for tracking moves, outcomes, and controlling game state.
* **Audio Feedback**: Web-sys integration for move and capture sound effects.

## Prerequisites

To build and run the GUI, you need the `wasm32-unknown-unknown` target and `trunk`:

```bash
# Add the WebAssembly target
rustup target add wasm32-unknown-unknown

# Install Trunk (Wasm web application bundler)
cargo install trunk
```

## Running Locally
Serve the application locally with hot-reloading:

```bash
# From the bonsai-gui directory
trunk serve --open
```

## License
```
This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.

You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>. 
```