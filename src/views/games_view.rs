use crate::state::ActiveView;

#[derive(Clone, Copy, Debug)]
pub struct GameCard {
    pub view: ActiveView,
    pub title: &'static str,
    pub description: &'static str,
    pub icon: &'static str,
    pub difficulty: &'static str,
    pub key_name: &'static str,
}

pub fn get_game_cards() -> Vec<GameCard> {
    vec![
        GameCard {
            view: ActiveView::GameBubbles,
            title: "Bubbles",
            description: "Pop descending character and word bubbles before they touch the ocean floor! Build up huge streak combos.",
            icon: "🫧",
            difficulty: "Beginner / All Levels",
            key_name: "bubbles",
        },
        GameCard {
            view: ActiveView::GameWordTris,
            title: "WordTris",
            description: "Falling word beams stack in columns. Type the words in time to destroy beams before the columns reach the top!",
            icon: "🧱",
            difficulty: "Intermediate",
            key_name: "wordtris",
        },
        GameCard {
            view: ActiveView::GameClouds,
            title: "Clouds",
            description: "Clouds float across the blue sky. Type each word and hit Space to blow them away! Watch for sunny bonus clouds.",
            icon: "☁️",
            difficulty: "All Levels",
            key_name: "clouds",
        },
        GameCard {
            view: ActiveView::GameAbc,
            title: "ABC Sprint",
            description: "How fast can you type the alphabet from A to Z with zero mistakes? Race against your best personal record.",
            icon: "⚡",
            difficulty: "Speed Reflexes",
            key_name: "abc",
        },
    ]
}
