# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

A chess game with game logic implemented in **Rust compiled to WebAssembly**, and a **Vite + TypeScript** frontend. The two sub-projects are:

- `chess-wasm/` — Rust library compiled to WASM via `wasm-pack`
- `chess-web/` — Vite + TypeScript frontend that imports the WASM package as a local npm dependency

## Development Commands

### Running the frontend
```bash
cd chess-web
npm run dev
```

### Building the WASM module (run from `chess-wasm/`)
```bash
cd chess-wasm
wasm-pack build --target web
```
The build outputs to `chess-wasm/pkg/`, which `chess-web` references via `"chess-wasm": "file:../chess-wasm/pkg"` in its `package.json`.

### Full build cycle after modifying Rust code
1. Run `wasm-pack build --target web` in `chess-wasm/`
2. The Vite dev server (`npm run dev` in `chess-web/`) will auto-reload via HMR

### Building the frontend for production
```bash
cd chess-web
npm run build   # runs tsc + vite build
```

## Architecture

### Rust/WASM Layer (`chess-wasm/src/`)
- `board.rs` — Data types: `Color`, `PieceType`, `Piece`, `Square`, `BoardGrid`
- `rules.rs` — `GameState` struct with all chess logic: move validation, check detection, move execution
- `lib.rs` — `GameEngine` WASM-exported struct; the bridge between Rust and JS. Serializes board state and moves as JSON strings via `serde_json`

### TypeScript Layer (`chess-web/src/`)
- `main.ts` — Initializes the WASM module with `await init()`, creates a `GameEngine` instance, renders the board via DOM manipulation, and handles click-based piece selection and move execution
- Board state flows: Rust `GameState` → JSON string → TypeScript interface → DOM render

### WASM ↔ JS Interface
- `engine.get_board_state()` → JSON string of the 8×8 board grid
- `engine.get_current_turn()` → JSON string of `"White"` or `"Black"`
- `engine.get_valid_moves(row, col)` → `JsValue[]` array of `{row, col}` objects
- `engine.move_piece(fr, fc, tr, tc)` → JSON string `{ success: boolean, message: string }`

### Known Limitations (not yet implemented)
- Castling
- En passant
- Pawn promotion
- Checkmate/stalemate detection
