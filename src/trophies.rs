use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TrophyDef {
    pub id: &'static str,
    pub title: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub category: &'static str,
}

pub const ALL_TROPHIES: &[TrophyDef] = &[
    TrophyDef {
        id: "first_steps",
        title: "First Steps",
        description: "Completed your very first touch typing exercise.",
        icon: "🌱",
        category: "Milestones",
    },
    TrophyDef {
        id: "home_row_hero",
        title: "Home Row Hero",
        description: "Passed Lesson 1 (Home Row) with 3 or more stars.",
        icon: "🏠",
        category: "Lessons",
    },
    TrophyDef {
        id: "speed_novice",
        title: "Speedy Fingers",
        description: "Reached 20+ Net WPM on any exercise.",
        icon: "⚡",
        category: "Speed",
    },
    TrophyDef {
        id: "speed_racer",
        title: "Speed Racer",
        description: "Reached 40+ Net WPM on any exercise.",
        icon: "🏎️",
        category: "Speed",
    },
    TrophyDef {
        id: "accuracy_ace",
        title: "Sharpshooter",
        description: "Finished any drill with 100% typing accuracy.",
        icon: "🎯",
        category: "Accuracy",
    },
    TrophyDef {
        id: "bubble_buster",
        title: "Bubble Buster",
        description: "Popped 20 bubbles in the Bubbles mini-game.",
        icon: "🫧",
        category: "Games",
    },
    TrophyDef {
        id: "wordtris_champion",
        title: "WordTris Architect",
        description: "Cleared 5 stacks of falling word beams in WordTris.",
        icon: "🧱",
        category: "Games",
    },
    TrophyDef {
        id: "cloud_chaser",
        title: "Cloud Chaser",
        description: "Scored 200+ points blowing away drifting clouds.",
        icon: "☁️",
        category: "Games",
    },
    TrophyDef {
        id: "abc_sprinter",
        title: "Alphabet Master",
        description: "Completed the ABC Alphabet Sprint in under 15 seconds.",
        icon: "🥇",
        category: "Games",
    },
    TrophyDef {
        id: "diploma_graduate",
        title: "Certified Typist",
        description: "Passed an official timed examination and earned a Diploma.",
        icon: "📜",
        category: "Certificates",
    },
    TrophyDef {
        id: "dedicated_scholar",
        title: "Dedicated Scholar",
        description: "Earned 25 total golden stars across course lessons.",
        icon: "⭐",
        category: "Milestones",
    },
    TrophyDef {
        id: "keyboard_virtuoso",
        title: "Keyboard Virtuoso",
        description: "Passed all 12 touch typing lessons in the course curriculum.",
        icon: "👑",
        category: "Mastery",
    },
];

pub fn get_all_trophies() -> Vec<TrophyDef> {
    ALL_TROPHIES.to_vec()
}
