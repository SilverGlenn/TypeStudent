use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyStat {
    pub hits: u32,
    pub misses: u32,
    pub total_latency_ms: u64,
}

impl Default for KeyStat {
    fn default() -> Self {
        Self {
            hits: 0,
            misses: 0,
            total_latency_ms: 0,
        }
    }
}

impl KeyStat {
    pub fn accuracy(&self) -> f32 {
        let total = self.hits + self.misses;
        if total == 0 {
            100.0
        } else {
            (self.hits as f32 / total as f32) * 100.0
        }
    }

    pub fn avg_latency_ms(&self) -> u64 {
        if self.hits == 0 {
            0
        } else {
            self.total_latency_ms / self.hits as u64
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct LessonRecord {
    pub stars: u8,
    pub best_wpm: f32,
    pub best_accuracy: f32,
    pub completed_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TestRecord {
    pub test_name: String,
    pub duration_seconds: u32,
    pub gross_wpm: f32,
    pub net_wpm: f32,
    pub accuracy: f32,
    pub errors: usize,
    pub completed_at: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserSettings {
    pub sound_volume: f32,
    pub sound_enabled: bool,
    pub theme: String, // "classic", "dark", "warm_retro"
    pub show_hands: bool,
    pub show_keyboard: bool,
    pub target_wpm: u32,
}

impl Default for UserSettings {
    fn default() -> Self {
        Self {
            sound_volume: 0.7,
            sound_enabled: true,
            theme: "classic".to_string(),
            show_hands: true,
            show_keyboard: true,
            target_wpm: 35,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub name: String,
    pub avatar_emoji: String,
    pub created_at: String,
    pub last_active: String,
    pub settings: UserSettings,
    pub key_stats: HashMap<char, KeyStat>,
    pub lesson_records: HashMap<String, LessonRecord>, // key: "lesson_id/exercise_id"
    pub test_history: Vec<TestRecord>,
    pub game_high_scores: HashMap<String, u32>, // "bubbles", "wordtris", "clouds", "abc"
    pub unlocked_trophies: HashMap<String, String>, // key: trophy_id, value: timestamp
}

impl UserProfile {
    pub fn new(id: String, name: String, avatar: String) -> Self {
        let now = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
        Self {
            id,
            name,
            avatar_emoji: avatar,
            created_at: now.clone(),
            last_active: now,
            settings: UserSettings::default(),
            key_stats: HashMap::new(),
            lesson_records: HashMap::new(),
            test_history: Vec::new(),
            game_high_scores: HashMap::new(),
            unlocked_trophies: HashMap::new(),
        }
    }

    pub fn unlock_trophy(&mut self, trophy_id: &str) -> bool {
        if !self.unlocked_trophies.contains_key(trophy_id) {
            let now = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
            self.unlocked_trophies.insert(trophy_id.to_string(), now);
            true
        } else {
            false
        }
    }

    pub fn has_trophy(&self, trophy_id: &str) -> bool {
        self.unlocked_trophies.contains_key(trophy_id)
    }

    pub fn record_keystroke(&mut self, ch: char, is_correct: bool, latency_ms: u64) {
        let stat = self.key_stats.entry(ch.to_ascii_lowercase()).or_default();
        if is_correct {
            stat.hits += 1;
            stat.total_latency_ms += latency_ms;
        } else {
            stat.misses += 1;
        }
    }

    pub fn get_difficult_keys(&self) -> Vec<(char, f32)> {
        let mut keys: Vec<(char, f32)> = self
            .key_stats
            .iter()
            .filter(|(ch, stat)| ch.is_alphabetic() && (stat.hits + stat.misses) >= 5)
            .map(|(&ch, stat)| (ch, stat.accuracy()))
            .filter(|(_, acc)| *acc < 90.0)
            .collect();
        keys.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));
        keys
    }

    pub fn total_lessons_passed(&self) -> usize {
        self.lesson_records.len()
    }

    pub fn total_stars(&self) -> u32 {
        self.lesson_records.values().map(|r| r.stars as u32).sum()
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ProfileStore {
    pub active_profile_id: String,
    pub profiles: Vec<UserProfile>,
}

impl Default for ProfileStore {
    fn default() -> Self {
        let default_profile = UserProfile::new("student_1".to_string(), "Student 1".to_string(), "🎓".to_string());
        Self {
            active_profile_id: default_profile.id.clone(),
            profiles: vec![default_profile],
        }
    }
}

impl ProfileStore {
    pub fn file_path() -> PathBuf {
        PathBuf::from("profiles_data.json")
    }

    pub fn load() -> Self {
        let path = Self::file_path();
        if path.exists() {
            if let Ok(data) = fs::read_to_string(&path) {
                if let Ok(store) = serde_json::from_str::<ProfileStore>(&data) {
                    if !store.profiles.is_empty() {
                        return store;
                    }
                }
            }
        }
        let store = Self::default();
        store.save();
        store
    }

    pub fn save(&self) {
        let path = Self::file_path();
        if let Ok(data) = serde_json::to_string_pretty(self) {
            let _ = fs::write(path, data);
        }
    }

    pub fn active_profile(&self) -> Option<&UserProfile> {
        self.profiles.iter().find(|p| p.id == self.active_profile_id)
    }

    pub fn active_profile_mut(&mut self) -> Option<&mut UserProfile> {
        self.profiles.iter_mut().find(|p| p.id == self.active_profile_id)
    }

    pub fn add_profile(&mut self, name: String, avatar: String) -> String {
        let id = format!("student_{}", self.profiles.len() + 1);
        let profile = UserProfile::new(id.clone(), name, avatar);
        self.profiles.push(profile);
        self.active_profile_id = id.clone();
        self.save();
        id
    }

    pub fn switch_profile(&mut self, id: &str) {
        if self.profiles.iter().any(|p| p.id == id) {
            self.active_profile_id = id.to_string();
            self.save();
        }
    }

    pub fn delete_profile(&mut self, id: &str) {
        if self.profiles.len() > 1 {
            self.profiles.retain(|p| p.id != id);
            if self.active_profile_id == id {
                self.active_profile_id = self.profiles[0].id.clone();
            }
            self.save();
        }
    }
}
