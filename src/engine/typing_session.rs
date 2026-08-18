use std::time::{Duration, Instant};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CharStatus {
    Pending,
    Correct,
    Incorrect(char), // Typed char that didn't match
}

#[derive(Clone, Debug)]
pub struct KeyTiming {
    pub char_expected: char,
    pub char_typed: char,
    pub is_correct: bool,
    pub latency_ms: u64,
}

#[derive(Clone, Debug)]
pub struct TypingSession {
    pub target_text: String,
    pub target_chars: Vec<char>,
    pub char_statuses: Vec<CharStatus>,
    pub cursor_idx: usize,
    
    pub start_time: Option<Instant>,
    pub finish_time: Option<Instant>,
    pub last_keystroke_time: Option<Instant>,
    
    pub total_keystrokes: usize,
    pub correct_keystrokes: usize,
    pub error_keystrokes: usize,
    pub backspaces_count: usize,
    
    pub key_timings: Vec<KeyTiming>,
    pub is_completed: bool,
    pub duration_limit: Option<Duration>, // For timed tests (e.g. 60s, 120s, 300s)
}

impl TypingSession {
    pub fn new(text: &str) -> Self {
        let chars: Vec<char> = text.chars().collect();
        let len = chars.len();
        Self {
            target_text: text.to_string(),
            target_chars: chars,
            char_statuses: vec![CharStatus::Pending; len],
            cursor_idx: 0,
            start_time: None,
            finish_time: None,
            last_keystroke_time: None,
            total_keystrokes: 0,
            correct_keystrokes: 0,
            error_keystrokes: 0,
            backspaces_count: 0,
            key_timings: Vec::new(),
            is_completed: false,
            duration_limit: None,
        }
    }

    pub fn with_time_limit(text: &str, limit: Duration) -> Self {
        let mut session = Self::new(text);
        session.duration_limit = Some(limit);
        session
    }

    pub fn current_char(&self) -> Option<char> {
        if self.cursor_idx < self.target_chars.len() {
            Some(self.target_chars[self.cursor_idx])
        } else {
            None
        }
    }

    pub fn next_char(&self) -> Option<char> {
        if self.cursor_idx + 1 < self.target_chars.len() {
            Some(self.target_chars[self.cursor_idx + 1])
        } else {
            None
        }
    }

    pub fn handle_char_input(&mut self, input: char) -> bool {
        if self.is_completed || self.cursor_idx >= self.target_chars.len() {
            return false;
        }

        let now = Instant::now();
        if self.start_time.is_none() {
            self.start_time = Some(now);
            self.last_keystroke_time = Some(now);
        }

        let latency_ms = if let Some(last) = self.last_keystroke_time {
            now.duration_since(last).as_millis() as u64
        } else {
            0
        };
        self.last_keystroke_time = Some(now);

        let expected = self.target_chars[self.cursor_idx];
        let is_correct = input == expected;

        self.total_keystrokes += 1;
        if is_correct {
            self.correct_keystrokes += 1;
            self.char_statuses[self.cursor_idx] = CharStatus::Correct;
        } else {
            self.error_keystrokes += 1;
            self.char_statuses[self.cursor_idx] = CharStatus::Incorrect(input);
        }

        self.key_timings.push(KeyTiming {
            char_expected: expected,
            char_typed: input,
            is_correct,
            latency_ms,
        });

        self.cursor_idx += 1;

        if self.cursor_idx >= self.target_chars.len() {
            self.finish(now);
        }

        is_correct
    }

    pub fn handle_backspace(&mut self) -> bool {
        if self.cursor_idx > 0 && !self.is_completed {
            self.cursor_idx -= 1;
            self.char_statuses[self.cursor_idx] = CharStatus::Pending;
            self.backspaces_count += 1;
            return true;
        }
        false
    }

    pub fn tick(&mut self) {
        if self.is_completed {
            return;
        }
        if let (Some(start), Some(limit)) = (self.start_time, self.duration_limit) {
            let elapsed = Instant::now().duration_since(start);
            if elapsed >= limit {
                self.finish(Instant::now());
            }
        }
    }

    pub fn finish(&mut self, at: Instant) {
        self.finish_time = Some(at);
        self.is_completed = true;
    }

    pub fn elapsed_seconds(&self) -> f32 {
        if let Some(start) = self.start_time {
            let end = self.finish_time.unwrap_or_else(Instant::now);
            end.duration_since(start).as_secs_f32().max(0.1)
        } else {
            0.0
        }
    }

    pub fn remaining_seconds(&self) -> Option<u32> {
        if let (Some(start), Some(limit)) = (self.start_time, self.duration_limit) {
            let elapsed = Instant::now().duration_since(start);
            if elapsed >= limit {
                Some(0)
            } else {
                Some((limit - elapsed).as_secs() as u32)
            }
        } else if let Some(limit) = self.duration_limit {
            Some(limit.as_secs() as u32)
        } else {
            None
        }
    }

    pub fn gross_wpm(&self) -> f32 {
        let secs = self.elapsed_seconds();
        if secs <= 0.5 || self.total_keystrokes == 0 {
            return 0.0;
        }
        let words = (self.total_keystrokes as f32) / 5.0;
        let minutes = secs / 60.0;
        (words / minutes).max(0.0)
    }

    pub fn net_wpm(&self) -> f32 {
        let secs = self.elapsed_seconds();
        if secs <= 0.5 || self.total_keystrokes == 0 {
            return 0.0;
        }
        let gross = self.gross_wpm();
        let minutes = secs / 60.0;
        let penalty = (self.error_keystrokes as f32) / minutes;
        (gross - penalty).max(0.0)
    }

    pub fn accuracy_percent(&self) -> f32 {
        if self.total_keystrokes == 0 {
            return 100.0;
        }
        ((self.correct_keystrokes as f32) / (self.total_keystrokes as f32)) * 100.0
    }

    pub fn progress_ratio(&self) -> f32 {
        if self.target_chars.is_empty() {
            0.0
        } else {
            (self.cursor_idx as f32) / (self.target_chars.len() as f32)
        }
    }

    pub fn calculate_stars(&self, target_wpm: u32) -> u8 {
        let acc = self.accuracy_percent();
        let net = self.net_wpm();
        let target = target_wpm as f32;

        if acc >= 98.0 && net >= target * 1.1 {
            5
        } else if acc >= 95.0 && net >= target * 0.9 {
            4
        } else if acc >= 90.0 && net >= target * 0.7 {
            3
        } else if acc >= 80.0 {
            2
        } else {
            1
        }
    }

    pub fn reset(&mut self) {
        let len = self.target_chars.len();
        self.char_statuses = vec![CharStatus::Pending; len];
        self.cursor_idx = 0;
        self.start_time = None;
        self.finish_time = None;
        self.last_keystroke_time = None;
        self.total_keystrokes = 0;
        self.correct_keystrokes = 0;
        self.error_keystrokes = 0;
        self.backspaces_count = 0;
        self.key_timings.clear();
        self.is_completed = false;
    }
}
