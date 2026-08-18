#[derive(Clone, Copy, Debug)]
pub struct StoryPreset {
    pub id: &'static str,
    pub title: &'static str,
    pub emoji: &'static str,
    pub difficulty: &'static str,
    pub word_count: usize,
    pub content: &'static str,
}

pub const STORY_PRESETS: &[StoryPreset] = &[
    StoryPreset {
        id: "dragon_books",
        title: "The Dragon Who Loved Books",
        emoji: "🐉",
        difficulty: "Easy",
        word_count: 42,
        content: "Deep in an emerald forest lived a friendly dragon named Barnaby. While other dragons practiced breathing fire, Barnaby spent his afternoons reading ancient fairy tales under the shade of giant whispering willow trees. Everyone in the village brought him new adventure stories.",
    },
    StoryPreset {
        id: "mars_rover",
        title: "Journey Beyond Mars",
        emoji: "🚀",
        difficulty: "Medium",
        word_count: 48,
        content: "A brave little exploration rover rolled across the red dusty hills of Mars. It flashed its solar panels toward the sun, collecting sparkling energy to study mysterious crystals buried deep beneath Olympus Mons. Signals traveled across space back to Earth with wonderful discoveries.",
    },
    StoryPreset {
        id: "treehouse_club",
        title: "The Secret Treehouse Club",
        emoji: "🌲",
        difficulty: "Easy",
        word_count: 44,
        content: "High up in the branches of the grand oak tree stood the secret clubhouse. Maya and Leo pulled up the rope ladder and spread out their treasure map. Today was the day they would decode the mystery of the enchanted golden compass.",
    },
    StoryPreset {
        id: "dolphin_cove",
        title: "Dolphin Island Secret",
        emoji: "🐬",
        difficulty: "Easy",
        word_count: 46,
        content: "Sunlight danced over the crystal clear waves of Coral Bay. A playful pod of blue dolphins leapt through the surf, inviting the seaside children to race along the sandy shoreline. Ocean breezes carried the fresh scent of sea salt and summer excitement.",
    },
    StoryPreset {
        id: "dino_keyboard",
        title: "T-Rex's Tiny Keyboard",
        emoji: "🦕",
        difficulty: "Fun",
        word_count: 45,
        content: "Timmy the T-Rex had enormous feet and sharp teeth, but very tiny arms. Whenever he tried to play typing games, his big claws pressed three keys at once! With patient practice on the home row, Timmy soon became the fastest dinosaur typist in the valley.",
    },
    StoryPreset {
        id: "crafting_legend",
        title: "The Diamond Sword Legend",
        emoji: "⚔️",
        difficulty: "Medium",
        word_count: 49,
        content: "Deep inside the dark cave, glowstone lit up the underground cavern. Alex gathered shiny blue diamonds and obsidian blocks before nightfall. With precise strikes at the crafting table, a glowing sword appeared, ready to protect the village from the midnight creepers.",
    },
];

pub fn get_all_stories() -> Vec<StoryPreset> {
    STORY_PRESETS.to_vec()
}
