use crate::audio::{SharedAudio, SoundEffect};
use crate::components::keyboard::Finger;
use crate::course::{get_all_lessons, Exercise, Lesson};
use crate::engine::TypingSession;
use crate::profile::{ProfileStore, TestRecord};
use crate::views::games::{AbcGame, BubblesGame, CloudsGame, WordTrisGame};
use std::time::Duration;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ActiveView {
    CourseOverview,
    TypingArena,
    ExerciseResults,
    SmartReview,
    StoryStudio,
    Trophies,
    TypingTests,
    DiplomaView,
    GamesHub,
    GameBubbles,
    GameWordTris,
    GameClouds,
    GameAbc,
    Statistics,
    Profiles,
    Settings,
}

#[derive(Clone, Debug)]
pub struct DiplomaData {
    pub student_name: String,
    pub test_title: String,
    pub net_wpm: f32,
    pub gross_wpm: f32,
    pub accuracy: f32,
    pub date: String,
    pub duration_label: String,
}

pub struct AppState {
    pub profile_store: ProfileStore,
    pub audio: SharedAudio,
    pub active_view: ActiveView,
    pub lessons: Vec<Lesson>,
    
    // Active typing session
    pub current_lesson_idx: usize,
    pub current_exercise_idx: usize,
    pub typing_session: Option<TypingSession>,
    pub current_exercise_info: Option<Exercise>,
    
    // Active games
    pub bubbles_game: BubblesGame,
    pub wordtris_game: WordTrisGame,
    pub clouds_game: CloudsGame,
    pub abc_game: AbcGame,
    
    // Diploma display
    pub active_diploma: Option<DiplomaData>,
    
    // Review session
    pub review_custom_text: Option<String>,

    // UI state
    pub is_sidebar_open: bool,
    pub is_pre_activity: bool,
}

impl AppState {
    pub fn new() -> Self {
        let profile_store = ProfileStore::load();
        let audio = SharedAudio::new();
        let lessons = get_all_lessons();

        let state = Self {
            profile_store,
            audio,
            active_view: ActiveView::CourseOverview,
            lessons,
            current_lesson_idx: 0,
            current_exercise_idx: 0,
            typing_session: None,
            current_exercise_info: None,
            bubbles_game: BubblesGame::new(),
            wordtris_game: WordTrisGame::new(),
            clouds_game: CloudsGame::new(),
            abc_game: AbcGame::new(),
            active_diploma: None,
            review_custom_text: None,
            is_sidebar_open: true,
            is_pre_activity: false,
        };

        // Apply audio settings from active profile
        if let Some(profile) = state.profile_store.active_profile() {
            state.audio.set_enabled(profile.settings.sound_enabled);
            state.audio.set_volume(profile.settings.sound_volume);
        }

        state
    }

    pub fn toggle_sidebar(&mut self) {
        self.is_sidebar_open = !self.is_sidebar_open;
    }

    pub fn begin_live_activity(&mut self) {
        self.is_pre_activity = false;
        if let Some(session) = &mut self.typing_session {
            session.start_time = None; // Reset timer so it starts with first actual keystroke
        }
    }

    pub fn start_exercise(&mut self, lesson_idx: usize, exercise_idx: usize) {
        if lesson_idx < self.lessons.len() && exercise_idx < self.lessons[lesson_idx].exercises.len() {
            self.current_lesson_idx = lesson_idx;
            self.current_exercise_idx = exercise_idx;
            let ex = self.lessons[lesson_idx].exercises[exercise_idx].clone();
            self.typing_session = Some(TypingSession::new(&ex.text));
            self.current_exercise_info = Some(ex);
            self.active_view = ActiveView::TypingArena;
            self.is_sidebar_open = false;
            self.is_pre_activity = true;
        }
    }

    pub fn start_typing_test(&mut self, title: &str, text: &str, duration_secs: u32) {
        let session = TypingSession::with_time_limit(text, Duration::from_secs(duration_secs as u64));
        self.typing_session = Some(session);
        self.current_exercise_info = Some(Exercise {
            id: "test".to_string(),
            title: title.to_string(),
            exercise_type: crate::course::ExerciseType::LessonTest,
            instruction: format!("Timed Typing Test: {} minutes. Type as quickly and accurately as possible!", duration_secs / 60),
            new_keys: vec![],
            text: text.to_string(),
        });
        self.active_view = ActiveView::TypingArena;
        self.is_sidebar_open = false;
        self.is_pre_activity = true;
    }

    pub fn start_smart_review(&mut self, drill_text: &str) {
        self.typing_session = Some(TypingSession::new(drill_text));
        self.current_exercise_info = Some(Exercise {
            id: "smart_review".to_string(),
            title: "Smart Review Drill".to_string(),
            exercise_type: crate::course::ExerciseType::KeyDrill,
            instruction: "Targeted drill for your most difficult keys. Focus on accuracy over raw speed!".to_string(),
            new_keys: vec![],
            text: drill_text.to_string(),
        });
        self.active_view = ActiveView::TypingArena;
        self.is_sidebar_open = false;
        self.is_pre_activity = true;
    }

    pub fn start_story_practice(&mut self, title: &str, story_text: &str) {
        self.typing_session = Some(TypingSession::new(story_text));
        self.current_exercise_info = Some(Exercise {
            id: "story_practice".to_string(),
            title: title.to_string(),
            exercise_type: crate::course::ExerciseType::SentenceDrill,
            instruction: "Read and type the story passage smoothly.".to_string(),
            new_keys: vec![],
            text: story_text.to_string(),
        });
        self.active_view = ActiveView::TypingArena;
        self.is_sidebar_open = false;
        self.is_pre_activity = true;
    }

    pub fn on_keystroke(&mut self, c: char) {
        match self.active_view {
            ActiveView::TypingArena => {
                if self.is_pre_activity {
                    self.begin_live_activity();
                    if c == ' ' || c == '\n' {
                        return;
                    }
                }

                if let Some(session) = &mut self.typing_session {
                    let is_correct = session.handle_char_input(c);
                    let latency = session.key_timings.last().map(|k| k.latency_ms).unwrap_or(0);
                    
                    if is_correct {
                        self.audio.play(SoundEffect::KeyClick);
                    } else {
                        self.audio.play(SoundEffect::KeyError);
                    }

                    // Record to active user profile
                    if let Some(profile) = self.profile_store.active_profile_mut() {
                        profile.record_keystroke(c, is_correct, latency);
                    }

                    if session.is_completed {
                        self.audio.play(SoundEffect::ExerciseSuccess);
                        self.finish_current_session();
                    }
                }
            }
            ActiveView::GameBubbles => {
                let (matched, popped) = self.bubbles_game.handle_char(c);
                if popped {
                    self.audio.play(SoundEffect::GamePop);
                } else if matched {
                    self.audio.play(SoundEffect::KeyClick);
                } else {
                    self.audio.play(SoundEffect::KeyError);
                }
            }
            ActiveView::GameWordTris => {
                let matched = self.wordtris_game.handle_char(c);
                if matched {
                    self.audio.play(SoundEffect::KeyClick);
                } else {
                    self.audio.play(SoundEffect::KeyError);
                }
            }
            ActiveView::GameClouds => {
                let cleared = self.clouds_game.handle_char(c);
                if cleared {
                    self.audio.play(SoundEffect::WordComplete);
                } else {
                    self.audio.play(SoundEffect::KeyClick);
                }
            }
            ActiveView::GameAbc => {
                let (correct, finished) = self.abc_game.handle_char(c);
                if finished {
                    self.audio.play(SoundEffect::ExerciseSuccess);
                } else if correct {
                    self.audio.play(SoundEffect::KeyClick);
                } else {
                    self.audio.play(SoundEffect::KeyError);
                }
            }
            _ => {}
        }
    }

    pub fn on_backspace(&mut self) {
        if self.active_view == ActiveView::TypingArena {
            if let Some(session) = &mut self.typing_session {
                if session.handle_backspace() {
                    self.audio.play(SoundEffect::KeyClick);
                }
            }
        } else if self.active_view == ActiveView::GameClouds {
            self.clouds_game.handle_char('\x08');
        }
    }

    pub fn check_trophies(&mut self) {
        let profile = match self.profile_store.active_profile_mut() {
            Some(p) => p,
            None => return,
        };

        let mut unlocked_any = false;

        // 1. First steps
        if !profile.lesson_records.is_empty() && profile.unlock_trophy("first_steps") {
            unlocked_any = true;
        }

        // 2. Home row hero (Lesson 1 completed with >= 3 stars)
        let has_home_row = profile.lesson_records.iter().any(|(k, v)| k.starts_with("lesson_1/") && v.stars >= 3);
        if has_home_row && profile.unlock_trophy("home_row_hero") {
            unlocked_any = true;
        }

        // 3. Dedicated scholar (25+ stars)
        if profile.total_stars() >= 25 && profile.unlock_trophy("dedicated_scholar") {
            unlocked_any = true;
        }

        // 4. Keyboard Virtuoso (all 12 lessons)
        let unique_lessons: std::collections::HashSet<usize> = profile
            .lesson_records
            .keys()
            .filter_map(|k| k.strip_prefix("lesson_")?.split('/').next()?.parse().ok())
            .collect();
        if unique_lessons.len() >= 12 && profile.unlock_trophy("keyboard_virtuoso") {
            unlocked_any = true;
        }

        // 5. Certified Typist
        if !profile.test_history.is_empty() && profile.unlock_trophy("diploma_graduate") {
            unlocked_any = true;
        }

        if unlocked_any {
            self.profile_store.save();
            self.audio.play(SoundEffect::ExerciseSuccess);
        }
    }

    pub fn finish_current_session(&mut self) {
        if let (Some(session), Some(info)) = (&self.typing_session, &self.current_exercise_info) {
            let target_wpm = self.profile_store.active_profile().map(|p| p.settings.target_wpm).unwrap_or(35);
            let stars = session.calculate_stars(target_wpm);
            let gross = session.gross_wpm();
            let net = session.net_wpm();
            let acc = session.accuracy_percent();
            let errors = session.error_keystrokes;

            let is_test = info.id == "test";
            if is_test {
                let test_title = info.title.clone();
                let now = chrono::Local::now().format("%B %d, %Y").to_string();
                let dur = session.elapsed_seconds() as u32;
                
                // Record test result
                if let Some(profile) = self.profile_store.active_profile_mut() {
                    profile.test_history.push(TestRecord {
                        test_name: test_title.clone(),
                        duration_seconds: dur,
                        gross_wpm: gross,
                        net_wpm: net,
                        accuracy: acc,
                        errors,
                        completed_at: now.clone(),
                    });
                }
                self.profile_store.save();

                // Generate diploma
                let student_name = self.profile_store.active_profile().map(|p| p.name.clone()).unwrap_or_else(|| "Student".to_string());
                self.active_diploma = Some(DiplomaData {
                    student_name,
                    test_title,
                    net_wpm: net,
                    gross_wpm: gross,
                    accuracy: acc,
                    date: now,
                    duration_label: format!("{} Minutes", (dur / 60).max(1)),
                });
                self.check_trophies();
                self.active_view = ActiveView::DiplomaView;
            } else {
                // Record exercise completion
                let record_key = format!("lesson_{}/{}", self.current_lesson_idx + 1, info.id);
                let now = chrono::Local::now().format("%Y-%m-%d %H:%M").to_string();
                if let Some(profile) = self.profile_store.active_profile_mut() {
                    profile.lesson_records.insert(
                        record_key,
                        crate::profile::LessonRecord {
                            stars,
                            best_wpm: net,
                            best_accuracy: acc,
                            completed_at: now,
                        },
                    );

                    // Check instant trophies (Speed, Accuracy)
                    if net >= 40.0 {
                        profile.unlock_trophy("speed_racer");
                    } else if net >= 20.0 {
                        profile.unlock_trophy("speed_novice");
                    }
                    if acc >= 100.0 && session.target_chars.len() >= 20 {
                        profile.unlock_trophy("accuracy_ace");
                    }
                }
                self.profile_store.save();
                self.check_trophies();
                self.active_view = ActiveView::ExerciseResults;
            }
        }
    }

    pub fn next_exercise(&mut self) {
        let next_ex = self.current_exercise_idx + 1;
        if next_ex < self.lessons[self.current_lesson_idx].exercises.len() {
            self.start_exercise(self.current_lesson_idx, next_ex);
        } else {
            let next_lesson = self.current_lesson_idx + 1;
            if next_lesson < self.lessons.len() {
                self.start_exercise(next_lesson, 0);
            } else {
                self.active_view = ActiveView::CourseOverview;
            }
        }
    }

    pub fn restart_current_exercise(&mut self) {
        if let Some(session) = &mut self.typing_session {
            session.reset();
            self.active_view = ActiveView::TypingArena;
        }
    }

    pub fn active_finger(&self) -> Option<Finger> {
        match self.active_view {
            ActiveView::TypingArena => {
                self.typing_session.as_ref().and_then(|s| s.current_char()).map(crate::components::keyboard::get_finger_for_char)
            }
            _ => None,
        }
    }
}
