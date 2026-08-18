# ⌨️ TypeStudent Pro

> **A modern, offline touch typing tutor and educational arcade built with Rust & GPUI.**  
> *A spiritual successor to the classic TypingMaster Pro, designed for kids, students, and touch typing enthusiasts.*

[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/Rust-1.80%2B-orange.svg)](https://www.rust-lang.org/)
[![GPUI](https://img.shields.io/badge/UI-GPUI%200.2.2-blueviolet.svg)](https://github.com/zed-industries/zed)
[![Offline](https://img.shields.io/badge/Offline-100%25-success.svg)](#)

---

## 🌟 Highlights

- 🎓 **12-Lesson Progressive Touch Typing Course**: Comprehensive curriculum starting from Home Row (`ASDF JKL;`) through reaching keys, `Shift` capital coordination, and numbers/symbols.
- 🖐️ **Visual Hands & Keyboard Guide**: Dynamic color-coded 5-finger zones (Pinky, Ring, Middle, Index, Thumb) and live hand indicator highlighting the exact finger for every keystroke.
- 🎮 **The 4 Classic TypingMaster Games**:
  - 🫧 **Bubbles**: Descending letter bubbles with combos and multipliers over an ocean backdrop.
  - 🧱 **WordTris**: Falling word beams stacking up in columns before reaching the danger ceiling.
  - ☁️ **Clouds**: Words drifting across a sunny sky—type and press `Space` to blow them away!
  - ⚡ **ABC Sprint**: A lightning-fast alphabet sprint testing reaction times from `A` to `Z`.
- 🏆 **Trophy Room & Achievement Badges**: 12 unlockable retro badges for speed milestones, accuracy streaks, game high scores, and course completion.
- 📖 **Story Studio**: Kid-friendly reading and typing stories featuring dragons, space rovers, treehouse adventures, and dinosaur typists.
- ⏱️ **Timed Tests & Printable Diplomas**: Standardized 1–5 minute tests generating official gold-bordered Certificates of Achievement exportable to clean HTML for home printing.
- 🎯 **Smart Review & Weak Key Heatmap**: Automatically analyzes per-key error rates and latency to generate targeted remedial drills.
- 👥 **Multi-Student Profiles**: Multiple children, siblings, or students can practice on the same computer with independent progress, stars, and diplomas saved locally in `profiles_data.json`.
- 🔊 **Procedural Waveform Audio**: Real-time synthesized mechanical keyclicks, error thuds, and victory fanfares with zero external audio assets required.
- 🔌 **100% Offline**: Requires zero internet connection, zero tracking, and zero ads.

---

## 📸 Overview & Design

TypeStudent Pro uses a clean, soothing **Light Theme** engineered specifically to prevent eye fatigue for children, using neutral slates, crisp whites, and gentle pastel finger zone highlights.

```
+-------------------------------------------------------------------------------+
|  TypeStudent Pro                                            Offline Mode 🔌   |
+-------------------------------------------------------------------------------+
| [📚 Lessons]       |  Lesson 1: Home Row (A S D F  J K L ;)                   |
| [🎯 Smart Review]  |  +----------------------------------------------------+  |
| [📖 Story Studio]  |  |  a s d f   j k l ;   a s d f   j k l ;   f j d k   |  |
| [🏆 Trophy Room]   |  +----------------------------------------------------+  |
| [⏱️ Typing Tests]  |                                                          |
| [🎮 Mini Games]    |         [ Q ][ W ][ E ][ R ][ T ][ Y ][ U ][ I ][ O ][ P ]|
| [📊 Heatmap Stats] |       [ A ][ S ][ D ][ F ][ G ][ H ][ J ][ K ][ L ][ ; ]  |
| [👥 Profiles]      |         [ Z ][ X ][ C ][ V ][ B ][ N ][ M ][ , ][ . ][ / ]|
| [⚙️ Settings]      |                     [      SPACE      ]                  |
|                    |             ( Left Hand )         ( Right Hand )         |
+-------------------------------------------------------------------------------+
```

---

## 🚀 Getting Started

### Prerequisites

Make sure you have **Rust & Cargo** installed:
```bash
# Verify Rust installation
cargo --version
```

### Installation & Run

1. Clone the repository:
```bash
git clone https://github.com/SilverGlenn/TypeStudent.git
cd TypeStudent
```

2. Run the application:
```bash
cargo run --release
```

3. Run the automated test suite:
```bash
cargo test
```

---

## 🏗️ Architecture

- **`src/app.rs`**: Core GPUI application view, state coordinator, light theme styling, and view routing.
- **`src/engine/`**: Keystroke engine, gross/net WPM calculations, latency tracking, and session state machine.
- **`src/components/`**: Interactive visual keyboard (`keyboard.rs`), active hands diagram (`hands.rs`), and HUD speedometer (`gauge.rs`).
- **`src/course/`**: 12 touch typing lessons, key introductions, word drills, and exams (`data.rs`).
- **`src/views/games/`**: The 4 classic games: `bubbles.rs`, `wordtris.rs`, `clouds.rs`, and `abc.rs`.
- **`src/trophies.rs`**: Achievement badges and trophy milestone tracking.
- **`src/views/stories_data.rs`**: Creative reading passages for kids.
- **`src/views/diploma_export.rs`**: HTML diploma certificate exporter.
- **`src/profile.rs`**: Multi-student JSON persistence and weak key heatmap extractor.
- **`src/audio.rs`**: Offline procedural audio synthesizer powered by `rodio`.

---

## 📜 License

Distributed under the **MIT License**. See [`LICENSE`](LICENSE) for more information.

---

## ❤️ Acknowledgements

Inspired by the cherished elementary school memories of **TypingMaster Pro**. Built with love to help the next generation master the keyboard with confidence and joy!
