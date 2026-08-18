use rand::Rng;

#[derive(Clone, Debug)]
pub struct CloudItem {
    pub text: String,
    pub typed_input: String,
    pub x: f32, // 1.0 (right side) drifting to -0.3 (left side)
    pub y: f32, // vertical altitude
    pub speed: f32,
    pub is_sunny: bool,
    pub is_cleared: bool,
}

#[derive(Clone, Debug)]
pub struct CloudsGame {
    pub clouds: Vec<CloudItem>,
    pub score: u32,
    pub misses: u32,
    pub max_misses: u32,
    pub is_game_over: bool,
    pub spawn_timer: f32,
    pub word_bank: Vec<&'static str>,
}

impl CloudsGame {
    pub fn new() -> Self {
        Self {
            clouds: Vec::new(),
            score: 0,
            misses: 0,
            max_misses: 5,
            is_game_over: false,
            spawn_timer: 0.0,
            word_bank: vec![
                "sky", "cloud", "wind", "breeze", "rain", "sunshine", "silver", "flying",
                "eagle", "plane", "storm", "thunder", "whisper", "horizon", "soar", "glider",
                "castle", "rainbow", "crystal", "floating", "feather", "zephyr", "glory",
            ],
        }
    }

    pub fn spawn_cloud(&mut self) {
        let mut rng = rand::thread_rng();
        let word = self.word_bank[rng.gen_range(0..self.word_bank.len())];
        let y = rng.gen_range(0.12..0.75);
        let speed = rng.gen_range(0.06..0.12);
        let is_sunny = rng.gen_bool(0.3);

        self.clouds.push(CloudItem {
            text: word.to_string(),
            typed_input: String::new(),
            x: 1.05,
            y,
            speed,
            is_sunny,
            is_cleared: false,
        });
    }

    pub fn update(&mut self, delta_secs: f32) -> (bool, bool) {
        if self.is_game_over {
            return (false, false);
        }

        let cleared_sound = false;
        let mut miss_sound = false;

        self.spawn_timer += delta_secs;
        if self.spawn_timer >= 2.8 {
            self.spawn_timer = 0.0;
            self.spawn_cloud();
        }

        for cloud in &mut self.clouds {
            cloud.x -= cloud.speed * delta_secs;
        }

        // Check escaped clouds
        let mut escaped = 0;
        self.clouds.retain(|cloud| {
            if cloud.is_cleared {
                false
            } else if cloud.x <= -0.2 {
                escaped += 1;
                false
            } else {
                true
            }
        });

        if escaped > 0 {
            miss_sound = true;
            self.misses += escaped;
            if self.misses >= self.max_misses {
                self.is_game_over = true;
            }
        }

        (cleared_sound, miss_sound)
    }

    pub fn handle_char(&mut self, c: char) -> bool {
        if self.is_game_over {
            return false;
        }

        if c == ' ' || c == '\n' {
            // Try to submit/clear matching cloud
            let mut cleared_idx = None;
            for (idx, cloud) in self.clouds.iter().enumerate() {
                if cloud.typed_input.eq_ignore_ascii_case(&cloud.text) {
                    cleared_idx = Some(idx);
                    break;
                }
            }

            if let Some(idx) = cleared_idx {
                let cloud = &mut self.clouds[idx];
                cloud.is_cleared = true;
                let multiplier = if cloud.is_sunny { 2 } else { 1 };
                self.score += (cloud.text.len() as u32 * 20) * multiplier;
                return true;
            }
        } else if c == '\x08' {
            // Backspace on closest cloud
            if let Some(cloud) = self.clouds.iter_mut().min_by(|a, b| a.x.partial_cmp(&b.x).unwrap()) {
                cloud.typed_input.pop();
                return true;
            }
        } else {
            // Append char to closest cloud with matching prefix or closest cloud
            if let Some(cloud) = self.clouds.iter_mut().min_by(|a, b| a.x.partial_cmp(&b.x).unwrap()) {
                cloud.typed_input.push(c);
                return true;
            }
        }

        false
    }

    pub fn restart(&mut self) {
        *self = Self::new();
    }
}
