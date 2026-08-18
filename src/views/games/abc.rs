use std::time::Instant;

#[derive(Clone, Debug)]
pub struct AbcGame {
    pub target_sequence: Vec<char>,
    pub current_idx: usize,
    pub mistakes: usize,
    pub start_time: Option<Instant>,
    pub finish_time: Option<Instant>,
    pub best_time_secs: Option<f32>,
    pub is_completed: bool,
}

impl AbcGame {
    pub fn new() -> Self {
        Self {
            target_sequence: ('a'..='z').collect(),
            current_idx: 0,
            mistakes: 0,
            start_time: None,
            finish_time: None,
            best_time_secs: None,
            is_completed: false,
        }
    }

    pub fn handle_char(&mut self, c: char) -> (bool, bool) {
        if self.is_completed {
            return (false, false);
        }

        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }

        let expected = self.target_sequence[self.current_idx];
        if c.to_ascii_lowercase() == expected {
            self.current_idx += 1;
            if self.current_idx >= self.target_sequence.len() {
                let finish = Instant::now();
                self.finish_time = Some(finish);
                self.is_completed = true;
                let elapsed = self.elapsed_secs();
                if self.best_time_secs.map_or(true, |b| elapsed < b) {
                    self.best_time_secs = Some(elapsed);
                }
                (true, true) // correct & finished
            } else {
                (true, false) // correct
            }
        } else {
            self.mistakes += 1;
            (false, false)
        }
    }

    pub fn elapsed_secs(&self) -> f32 {
        if let Some(start) = self.start_time {
            let end = self.finish_time.unwrap_or_else(Instant::now);
            end.duration_since(start).as_secs_f32()
        } else {
            0.0
        }
    }

    pub fn current_char(&self) -> Option<char> {
        if self.current_idx < self.target_sequence.len() {
            Some(self.target_sequence[self.current_idx])
        } else {
            None
        }
    }

    pub fn restart(&mut self) {
        let best = self.best_time_secs;
        *self = Self::new();
        self.best_time_secs = best;
    }
}
