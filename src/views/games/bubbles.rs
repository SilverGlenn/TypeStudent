use rand::Rng;

#[derive(Clone, Debug)]
pub struct BubbleItem {
    pub id: u64,
    pub text: String,
    pub typed_len: usize,
    pub x: f32, // 0.0 to 1.0 (horizontal ratio)
    pub y: f32, // 0.0 (top) to 1.0 (bottom)
    pub speed: f32,
    pub radius: f32,
    pub color_hue: f32,
    pub is_popping: bool,
    pub pop_anim: f32, // 0.0 to 1.0
}

#[derive(Clone, Debug)]
pub struct BubblesGame {
    pub bubbles: Vec<BubbleItem>,
    pub score: u32,
    pub streak: u32,
    pub lives: u8,
    pub max_lives: u8,
    pub is_game_over: bool,
    pub next_id: u64,
    pub spawn_timer: f32,
    pub difficulty_level: u32,
    pub words_bank: Vec<&'static str>,
}

impl BubblesGame {
    pub fn new() -> Self {
        Self {
            bubbles: Vec::new(),
            score: 0,
            streak: 0,
            lives: 5,
            max_lives: 5,
            is_game_over: false,
            next_id: 1,
            spawn_timer: 0.0,
            difficulty_level: 1,
            words_bank: vec![
                "cat", "dog", "sun", "sky", "run", "joy", "sea", "fox", "cup", "gem",
                "star", "tree", "blue", "gold", "fish", "book", "bird", "rain", "moon", "wind",
                "apple", "quick", "flash", "light", "space", "water", "dream", "happy", "cloud", "music",
            ],
        }
    }

    pub fn spawn_bubble(&mut self) {
        let mut rng = rand::thread_rng();
        let (text, radius) = if self.difficulty_level <= 2 && rng.gen_bool(0.6) {
            // Single character
            let ch = rng.gen_range(b'a'..=b'z') as char;
            (ch.to_string(), 28.0)
        } else {
            // Word
            let word = self.words_bank[rng.gen_range(0..self.words_bank.len())];
            (word.to_string(), 36.0 + (word.len() as f32 * 4.0))
        };

        let x = rng.gen_range(0.08..0.88);
        let speed = 0.08 + (self.difficulty_level as f32 * 0.015) + rng.gen_range(0.0..0.03);
        let hue = rng.gen_range(0.0..360.0);

        self.bubbles.push(BubbleItem {
            id: self.next_id,
            text,
            typed_len: 0,
            x,
            y: -0.05,
            speed,
            radius,
            color_hue: hue,
            is_popping: false,
            pop_anim: 0.0,
        });
        self.next_id += 1;
    }

    pub fn update(&mut self, delta_secs: f32) -> (bool, bool) {
        if self.is_game_over {
            return (false, false);
        }

        let sound_pop = false;
        let mut sound_miss = false;

        self.spawn_timer += delta_secs;
        let spawn_interval = (2.2 - (self.difficulty_level as f32 * 0.2)).max(0.7);
        if self.spawn_timer >= spawn_interval {
            self.spawn_timer = 0.0;
            self.spawn_bubble();
        }

        // Update existing bubbles
        for bubble in &mut self.bubbles {
            if bubble.is_popping {
                bubble.pop_anim += delta_secs * 4.0;
            } else {
                bubble.y += bubble.speed * delta_secs;
            }
        }

        // Check missed bubbles that reached bottom
        let mut lost_lives = 0;
        self.bubbles.retain(|bubble| {
            if !bubble.is_popping && bubble.y >= 0.95 {
                lost_lives += 1;
                false
            } else if bubble.is_popping && bubble.pop_anim >= 1.0 {
                false
            } else {
                true
            }
        });

        if lost_lives > 0 {
            sound_miss = true;
            self.streak = 0;
            if self.lives <= lost_lives {
                self.lives = 0;
                self.is_game_over = true;
            } else {
                self.lives -= lost_lives;
            }
        }

        (sound_pop, sound_miss)
    }

    pub fn handle_char(&mut self, c: char) -> (bool, bool) {
        if self.is_game_over {
            return (false, false);
        }

        let lower = c.to_ascii_lowercase();

        // 1. Try to continue a partially typed bubble first (lowest on screen)
        let mut target_idx = None;
        let mut lowest_y = -1.0;

        for (idx, bubble) in self.bubbles.iter().enumerate() {
            if bubble.is_popping {
                continue;
            }
            if bubble.typed_len > 0 && bubble.typed_len < bubble.text.len() {
                let next_char = bubble.text.chars().nth(bubble.typed_len).unwrap().to_ascii_lowercase();
                if next_char == lower && bubble.y > lowest_y {
                    lowest_y = bubble.y;
                    target_idx = Some(idx);
                }
            }
        }

        // 2. If no partially typed bubble matched, find lowest matching new bubble
        if target_idx.is_none() {
            for (idx, bubble) in self.bubbles.iter().enumerate() {
                if bubble.is_popping || bubble.typed_len > 0 {
                    continue;
                }
                let first_char = bubble.text.chars().next().unwrap().to_ascii_lowercase();
                if first_char == lower && bubble.y > lowest_y {
                    lowest_y = bubble.y;
                    target_idx = Some(idx);
                }
            }
        }

        if let Some(idx) = target_idx {
            let bubble = &mut self.bubbles[idx];
            bubble.typed_len += 1;

            if bubble.typed_len >= bubble.text.len() {
                // Popped!
                bubble.is_popping = true;
                self.streak += 1;
                let combo_bonus = (self.streak / 5) * 20;
                let points = (bubble.text.len() as u32 * 10) + combo_bonus;
                self.score += points;

                if self.score / 150 > self.difficulty_level {
                    self.difficulty_level += 1;
                }
                (true, true) // match + popped
            } else {
                (true, false) // match but not yet popped
            }
        } else {
            self.streak = 0;
            (false, false)
        }
    }

    pub fn restart(&mut self) {
        *self = Self::new();
    }
}
