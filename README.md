# Peg Solitaire (Rust + Slint)

A desktop implementation of the classic Peg Solitaire board game, built in Rust with [Slint](https://slint.rs/) for the UI. The game supports both manual play and an automated solver mode, with a resizable board and real-time status updates.

## Features

- **Manual Mode** — Click a peg, then click a destination two cells away to jump it, following standard Peg Solitaire rules (a peg is captured if it's jumped over).
- **Automated Mode** — Toggle to an auto-play mode where the game selects and executes a valid move for you.
- **Configurable Board Size** — Adjust the board dimensions and the game grid rebuilds and resets automatically.
- **Randomize** — Shuffle the current pegs across the board while preserving the total peg count.
- **New Game / Reset** — Start over at any time.
- **Live Status Tracking** — Displays remaining peg count and detects when no valid moves remain ("Game Over").

## Tech Stack

- **Language:** Rust
- **UI Framework:** [Slint](https://slint.rs/) (`.slint` markup + Rust callbacks)
- **Key Dependencies:**
  - `slint` / `slint-build` — UI rendering and code generation
  - `rand` — board randomization and automated move selection

## Project Structure

```
├── src/
│   ├── main.rs      # App entry point, UI wiring, and event callbacks
│   ├── game.rs       # Game trait, ManualGame/AutomatedGame implementations, GameManager
│   └── grid.rs       # Board/grid logic: move validation, jump execution, win/loss detection
├── ui/
│   └── app-window.slint   # Slint UI markup (board rendering, controls, styling)
├── Cargo.toml
└── Cargo.lock
```

## Getting Started

### Prerequisites

- Rust and Cargo — install via the [official guide](https://www.rust-lang.org/learn/get-started)

### Build

```bash
cargo build
```

### Run

```bash
cargo run
```

### Test

Unit tests for core game/grid logic (move validation, jump legality, etc.) live alongside the code in `src/grid.rs`:

```bash
cargo test
```

## How to Play

1. Launch the app — you'll start in **Manual Mode** with a default board.
2. Click a peg to select it, then click a cell two spaces away (horizontally or vertically) to jump over and capture the peg in between.
3. Continue until no more valid moves remain, or switch on **Automated Mode** to watch the solver play instead.
4. Use **Randomize** to shuffle the board, adjust the **board size** control to change difficulty, or hit **New Game** to reset.

## Development Notes

This project was built iteratively across multiple sprints, with each phase adding functionality (manual play → automated play → board resizing → testing/refactoring). Code reviews and test coverage reviews were conducted as part of the development process to improve code quality and catch design issues early.

## License

Add your license of choice here (e.g., MIT).
