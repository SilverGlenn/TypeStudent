# TypeStudent

Offline desktop touch typing tutor and educational arcade built with Rust and GPUI.

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)

## Features

- **12-Lesson Touch Typing Curriculum**: Progressive exercises covering home row (`ASDF JKL;`), reach keys, uppercase shift coordination, number row, and punctuation.
- **Typing Guidance Console**: Virtual keyboard paired with animated hand placement guides showing finger lift and home-row anchor positions.
- **4 Typing Mini-Games**:
  - `Bubbles`: Pop descending character and word bubbles.
  - `WordTris`: Clear falling word beams before column stacks overflow.
  - `Clouds`: Type passing cloud text and press Space to dismiss.
  - `ABC Sprint`: Alphabet reaction benchmark from A to Z.
- **Trophy System**: 12 milestone badges tracking speed thresholds, accuracy streaks, and lesson completion.
- **Story Studio**: Reading passages for sentence-level typing practice.
- **Timed Tests and Printable Diplomas**: 1 to 5 minute standardized typing tests with one-click HTML certificate export.
- **Weak Key Diagnostics**: Per-key accuracy heatmap and automatic remedial drill generation.
- **Multi-User Profiles**: Local profile storage (`profiles_data.json`) for multiple student accounts.
- **Offline Procedural Audio**: Synthesized keystroke clicks and audio feedback via `rodio` with zero external audio assets.

## Getting Started

### Prerequisites

- Rust 1.80 or newer with Cargo

### Build and Run

```bash
# Clone the repository
git clone https://github.com/SilverGlenn/TypeStudent.git
cd TypeStudent

# Run development build
cargo run

# Run release build
cargo run --release

# Run unit tests
cargo test
```

## Project Structure

```
src/
├── app.rs                  # Primary GPUI view and UI layout
├── audio.rs                # Procedural sound synthesis (rodio)
├── main.rs                 # Application entry point and window setup
├── profile.rs              # Profile store and weak key analysis
├── state.rs                # Application state machine
├── trophies.rs             # Achievement criteria and unlock registry
├── components/
│   ├── gauge.rs            # Speedometer and accuracy gauge components
│   ├── hands.rs            # Hand geometry and finger guide states
│   └── keyboard.rs         # Layout definitions and finger zone mapping
├── course/
│   └── data.rs             # 12-lesson curriculum definitions
├── engine/
│   └── typing_session.rs   # Keystroke engine, WPM calculation, and accuracy tracking
└── views/
    ├── diploma_export.rs   # HTML certificate generator
    ├── games_view.rs       # Mini-games launcher
    ├── sidebar.rs          # Navigation sidebar definition
    ├── stories_data.rs     # Reading practice passages
    └── games/
        ├── abc.rs          # Alphabet sprint game
        ├── bubbles.rs      # Bubbles game
        ├── clouds.rs       # Clouds game
        └── wordtris.rs     # WordTris falling block game
```

## License

MIT License. See [LICENSE](LICENSE) for details.
