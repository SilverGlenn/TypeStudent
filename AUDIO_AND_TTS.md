# Audio and Speech Synthesis Architecture

This document describes the audio pipeline, neural text-to-speech (TTS) integration, anticipatory pre-caching mechanism, and keystroke audio triggers in TypeStudent.

---

## 1. Subsystem Overview

TypeStudent uses a dual-engine audio architecture:
1. **Procedural Sound Generator (`rodio`)**: Generates real-time procedural PCM waveforms for keystroke clicks, errors, chimes, and game sounds.
2. **Neural Speech Synthesis (`Piper` + OS Fallback)**: Pronounces letter names for beginner drills and reads completed words in real-time.

```
[Keystroke / Lifecycle Event]
            │
            ├──► Sound Generator (Procedural PCM: KeyClick, KeyError, Chime) ──► Rodio Sink
            │
            └──► Speech Subsystem
                     │
                     ├── In-Memory Sample Cache Hit? ─────────────────────────► Rodio Sink (0 ms)
                     │
                     └── Cache Miss ──► Piper CLI / OS TTS ──► Insert to Cache ─► Rodio Sink
```

---

## 2. Piper Neural TTS Integration

### Asset Layout
The bundled Piper files reside in `assets/piper/`:

```
assets/piper/
├── bin/
│   ├── piper.exe                       # Standalone Piper inference binary
│   ├── onnxruntime.dll                 # ONNX Runtime execution engine
│   ├── piper_phonemize.dll             # Text-to-phoneme phonemizer
│   ├── espeak-ng.dll                   # eSpeak-NG phonetic dictionary
│   ├── espeak-ng-data/                 # Language data tables
│   └── libtashkeel_model.ort
└── models/
    ├── en_US-amy-medium.onnx           # VITS neural voice model (~60 MB)
    └── en_US-amy-medium.onnx.json      # Model metadata and phoneme configuration
```

### Synthesis Pipeline
- **Command Invocation**:
  ```bash
  piper.exe --model assets/piper/models/en_US-amy-medium.onnx --output_raw --length_scale 1.15 --noise_scale 0.85 --noise_w 0.90 -q
  ```
- **Input**: UTF-8 text sent through `stdin` followed by a newline.
- **Output Format**: Raw 16-bit signed integer little-endian (`i16`) mono PCM at **22,050 Hz**.
- **Conversion to Floating-Point PCM**:
  ```rust
  let mut samples = Vec::with_capacity(raw_bytes.len() / 2);
  for chunk in raw_bytes.chunks_exact(2) {
      let s = i16::from_le_bytes([chunk[0], chunk[1]]);
      samples.push(s as f32 / 32768.0);
  }
  let source = rodio::buffer::SamplesBuffer::new(1, 22050, samples);
  ```

---

## 3. Pre-Caching & Low-Latency Playback

### The Problem
Spawning `piper.exe` per keystroke incurs an ~800 ms process initialization and ONNX model loading penalty.

### The Solution: Prioritized Pre-Caching & Dual-Thread Architecture
1. **Thread Separation**:
   - **Playback / Speech Thread**: Manages immediate playback through `rodio::Sink` with zero latency when samples are in memory.
   - **Pre-cache Thread**: Dedicated background worker processing words strictly in order of appearance via an `Arc<(Mutex<VecDeque<String>>, Condvar)>` priority queue.
2. **Prioritized In-Order Caching**:
   - When an exercise starts, all words and characters from the passage are prepended to the front of the queue in sequential order.
   - Word 1 and character 1 are synthesized first, followed by word 2, word 3, etc., during the pre-activity warmup and 3-second countdown.
   - Each word is placed into the shared `Arc<Mutex<HashMap<String, Vec<f32>>>>` cache immediately upon synthesis completion, making it instantly playable.
3. **Instant Fast-Path Playback**:
   - When the student finishes typing a word on `Space`, `speak_word` locks the shared cache. If present, it directly streams into `rodio::Sink` with **$<1\text{ ms}$ latency** without queue bottlenecks.

---

## 4. Keystroke Speech Triggers

In `AppState::on_keystroke` (`src/state.rs`):

| Exercise Context | Keystroke Condition | Action |
| :--- | :--- | :--- |
| **Pre-Activity Warmup** | Valid key pressed | Plays `KeyClick` (Speech muted during warmup to prioritize pre-caching) |
| **Letter Drills** (`KeyIntro`, `KeyDrill`) | Correct key pressed | Plays `KeyClick` + calls `speak_letter(c)` |
| **Word / Story / Test Drills** | Word boundary reached (`Space` or end) | Plays `KeyClick` + calls `speak_word(word)` |
| **Any Activity** | Incorrect key pressed | Plays `KeyError` (does not play incorrect speech) |

*Note: Speech is non-interrupting (`interrupt: false`) to ensure students hear full, natural English pronunciations.*

---

## 5. Keystroke Boundary and Final Character Logic

In `TypingSession::handle_char_input` (`src/engine/typing_session.rs`):

```rust
if is_correct {
    self.correct_keystrokes += 1;
    self.char_statuses[self.cursor_idx] = CharStatus::Correct;
    self.cursor_idx += 1;

    if self.cursor_idx >= self.target_chars.len() {
        self.finish(now);
    }
} else {
    self.error_keystrokes += 1;
    self.char_statuses[self.cursor_idx] = CharStatus::Incorrect(input);
    // Prevent premature session completion on a final character mistake
    if self.cursor_idx + 1 < self.target_chars.len() {
        self.cursor_idx += 1;
    }
}
```

- **Mistake on intermediate characters**: Advances cursor to allow continuous typing; error is recorded in red.
- **Mistake on the final character**: Cursor remains at the final character, requiring the student to enter the correct key (or press Backspace) to complete the exercise.

---

## 6. Future Improvement Roadmap

1. **Persistent Piper IPC Process**:
   - Instead of one process invocation per cache miss, run `piper.exe` with `--json-input` as a persistent background process over standard I/O to reduce uncached inference to ~30 ms.
2. **Direct C-FFI / ONNX Runtime In-Process Binding**:
   - Link `onnxruntime` and `libpiper_phonemize` directly into the Rust binary to eliminate subprocess execution entirely.
3. **Phonics Mode for Early Childhood (Ages 3–6)**:
   - Add a toggle between **Letter Names** (*"A"*, *"B"*, *"C"*) and **Phonetic Sounds** (*/æ/*, */b/*, */k/*) for early phonemic awareness.
4. **LRU Cache Eviction**:
   - If memory footprint needs tight constraints on low-spec hardware, bound `HashMap<String, Vec<f32>>` with an LRU capacity (e.g. 500 words).
