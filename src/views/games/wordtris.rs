use rand::Rng;

#[derive(Clone, Debug)]
pub struct WordBeam {
    pub text: String,
    pub typed_len: usize,
    pub column: usize, // 0..4
    pub y: f32,        // 0.0 to 1.0 (reaches floor at 0.9)
    pub is_settled: bool,
}

#[derive(Clone, Debug)]
pub struct WordTrisGame {
    pub columns: usize,
    pub active_beams: Vec<WordBeam>,
    pub stack_heights: Vec<usize>, // number of blocks stacked per column
    pub max_stack: usize,          // e.g. 6 = danger / game over
    pub score: u32,
    pub lines_cleared: u32,
    pub is_game_over: bool,
    pub spawn_timer: f32,
    pub fall_speed: f32,
    pub word_bank: Vec<&'static str>,
}

impl WordTrisGame {
    pub fn new() -> Self {
        Self {
            columns: 5,
            active_beams: Vec::new(),
            stack_heights: vec![0; 5],
            max_stack: 6,
            score: 0,
            lines_cleared: 0,
            is_game_over: false,
            spawn_timer: 0.0,
            fall_speed: 0.09,
            word_bank: vec![
                "code", "rust", "game", "type", "fast", "gold", "byte", "loop", "data", "task",
                "core", "flow", "star", "grid", "wave", "drop", "time", "test", "hand", "mind",
                "master", "stream", "engine", "system", "vector", "player", "action", "memory",
            ],
        }
    }

    pub fn spawn_beam(&mut self) {
        let mut rng = rand::thread_rng();
        let column = rng.gen_range(0..self.columns);
        let word = self.word_bank[rng.gen_range(0..self.word_bank.len())];

        self.active_beams.push(WordBeam {
            text: word.to_string(),
            typed_len: 0,
            column,
            y: 0.0,
            is_settled: false,
        });
    }

    pub fn update(&mut self, delta_secs: f32) -> (bool, bool) {
        if self.is_game_over {
            return (false, false);
        }

        let mut dropped_sound = false;
        let mut error_sound = false;

        self.spawn_timer += delta_secs;
        if self.spawn_timer >= 2.5 {
            self.spawn_timer = 0.0;
            self.spawn_beam();
        }

        for beam in &mut self.active_beams {
            if !beam.is_settled {
                beam.y += self.fall_speed * delta_secs;
                let floor_limit = 0.85 - (self.stack_heights[beam.column] as f32 * 0.12);
                if beam.y >= floor_limit {
                    beam.y = floor_limit;
                    beam.is_settled = true;
                    dropped_sound = true;
                }
            }
        }

        // Check newly settled beams
        let mut newly_settled_columns = Vec::new();
        self.active_beams.retain(|beam| {
            if beam.is_settled {
                newly_settled_columns.push(beam.column);
                false
            } else {
                true
            }
        });

        for col in newly_settled_columns {
            self.stack_heights[col] += 1;
            if self.stack_heights[col] >= self.max_stack {
                self.is_game_over = true;
                error_sound = true;
            }
        }

        (dropped_sound, error_sound)
    }

    pub fn handle_char(&mut self, c: char) -> bool {
        if self.is_game_over {
            return false;
        }

        let lower = c.to_ascii_lowercase();

        // Find beam that matches
        let mut match_idx = None;
        for (idx, beam) in self.active_beams.iter().enumerate() {
            if beam.typed_len < beam.text.len() {
                let expected = beam.text.chars().nth(beam.typed_len).unwrap().to_ascii_lowercase();
                if expected == lower {
                    match_idx = Some(idx);
                    break;
                }
            }
        }

        if let Some(idx) = match_idx {
            let beam = &mut self.active_beams[idx];
            beam.typed_len += 1;

            if beam.typed_len >= beam.text.len() {
                self.score += (beam.text.len() as u32) * 15;
                self.lines_cleared += 1;
                self.active_beams.remove(idx);
                // Lower a stack column if available
                for h in &mut self.stack_heights {
                    if *h > 0 {
                        *h -= 1;
                        break;
                    }
                }
            }
            true
        } else {
            false
        }
    }

    pub fn restart(&mut self) {
        *self = Self::new();
    }
}
