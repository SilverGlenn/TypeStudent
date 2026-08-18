use crate::state::{ActiveView, AppState};
use crate::views::arena_view::render_typing_arena;
use crate::views::course_view::render_course_overview;
use crate::views::diploma_view::render_diploma;
use crate::views::games_view::get_game_cards;
use crate::views::profile_view::render_profiles;
use crate::views::results_view::render_exercise_results;
use crate::views::review_view::render_smart_review;
use crate::views::settings_view::render_settings;
use crate::views::sidebar::get_sidebar_items;
use crate::views::stats_view::render_statistics;
use crate::views::stories_view::render_story_studio;
use crate::views::tests_view::render_typing_tests;
use crate::views::trophies_view::render_trophies;
use gpui::*;

pub struct TypeStudentView {
    pub state: AppState,
    pub focus_handle: FocusHandle,
    pub export_status: Option<String>,
}

impl TypeStudentView {
    pub fn new(cx: &mut Context<Self>) -> Self {
        Self {
            state: AppState::new(),
            focus_handle: cx.focus_handle(),
            export_status: None,
        }
    }

    fn parse_keystroke(event: &KeyDownEvent) -> Option<char> {
        let key = &event.keystroke.key;
        let shift = event.keystroke.modifiers.shift;

        match key.as_str() {
            "backspace" => Some('\x08'),
            "space" | " " => Some(' '),
            "enter" | "return" => Some('\n'),
            _ if key.len() == 1 => {
                let c = key.chars().next().unwrap();
                if shift {
                    let shifted = match c {
                        'a'..='z' => c.to_ascii_uppercase(),
                        '1' => '!',
                        '2' => '@',
                        '3' => '#',
                        '4' => '$',
                        '5' => '%',
                        '6' => '^',
                        '7' => '&',
                        '8' => '*',
                        '9' => '(',
                        '0' => ')',
                        '-' => '_',
                        '=' => '+',
                        ';' => ':',
                        '\'' => '"',
                        ',' => '<',
                        '.' => '>',
                        '/' => '?',
                        '`' => '~',
                        '[' => '{',
                        ']' => '}',
                        '\\' => '|',
                        other => other,
                    };
                    Some(shifted)
                } else {
                    Some(c)
                }
            }
            _ => None,
        }
    }

    pub fn start_pre_activity_countdown(&mut self, cx: &mut Context<Self>) {
        if self.state.pre_activity_countdown.is_some() {
            return;
        }
        self.state.start_countdown();
        self.state.audio.play(crate::audio::SoundEffect::ExerciseSuccess);
        cx.notify();

        let weak_entity = cx.entity().downgrade();
        let async_cx = cx.to_async();

        cx.foreground_executor().spawn(async move {
            for _ in 0..3 {
                async_cx.background_executor().timer(std::time::Duration::from_millis(900)).await;
                let is_finished = async_cx.update(|cx| {
                    if let Some(view) = weak_entity.upgrade() {
                        view.update(cx, |this, cx| {
                            if let Some(cnt) = &mut this.state.pre_activity_countdown {
                                if *cnt > 1 {
                                    *cnt -= 1;
                                    this.state.audio.play(crate::audio::SoundEffect::KeyClick);
                                    cx.notify();
                                    false
                                } else {
                                    this.state.begin_live_activity();
                                    this.state.audio.play(crate::audio::SoundEffect::ExerciseSuccess);
                                    cx.notify();
                                    true
                                }
                            } else {
                                true
                            }
                        })
                    } else {
                        true
                    }
                }).unwrap_or(true);

                if is_finished {
                    break;
                }
            }
        }).detach();
    }
}

impl Render for TypeStudentView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        window.focus(&self.focus_handle);

        let active_view = self.state.active_view;
        let is_typing_view = active_view == ActiveView::TypingArena;
        let is_sidebar_open = self.state.is_sidebar_open;
        let active_profile = self.state.profile_store.active_profile().cloned();
        let current_stars = active_profile.as_ref().map(|p| p.total_stars()).unwrap_or(0);
        let student_name = active_profile.as_ref().map(|p| p.name.clone()).unwrap_or_else(|| "Student".to_string());
        let avatar = active_profile.as_ref().map(|p| p.avatar_emoji.clone()).unwrap_or_else(|| "🎓".to_string());

        div()
            .id("root_container")
            .track_focus(&self.focus_handle)
            .flex()
            .flex_row()
            .size_full()
            .bg(rgb(0xf8fafc))
            .text_color(rgb(0x0f172a))
            .font_family(".SystemUIFont")
            .key_context("type_student_view")
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                if let Some(ch) = Self::parse_keystroke(event) {
                    if ch == '\x08' {
                        this.state.on_backspace();
                    } else {
                        let was_counting = this.state.pre_activity_countdown.is_some();
                        this.state.on_keystroke(ch);
                        if !was_counting && this.state.pre_activity_countdown.is_some() {
                            this.state.pre_activity_countdown = None; // Reset so helper starts clean async task
                            this.start_pre_activity_countdown(cx);
                        }
                    }
                    cx.notify();
                }
            }))
            // 1. Optional / Toggleable Sidebar Navigation
            .children(if is_sidebar_open {
                Some(
                    div()
                        .w(px(250.0))
                        .h_full()
                        .bg(rgb(0xffffff))
                        .border_r_1()
                        .border_color(rgb(0xe2e8f0))
                        .flex()
                        .flex_col()
                        .justify_between()
                        .p_4()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_4()
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_2()
                                                .child(div().text_size(px(22.0)).child("⌨️"))
                                                .child(
                                                    div()
                                                        .text_size(px(17.0))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(0x0369a1))
                                                        .child("TypeStudent"),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .id("btn_collapse_sidebar")
                                                .px_2()
                                                .py_1()
                                                .rounded_md()
                                                .bg(rgb(0xf1f5f9))
                                                .cursor_pointer()
                                                .hover(|s| s.bg(rgb(0xe2e8f0)))
                                                .text_size(px(12.0))
                                                .text_color(rgb(0x64748b))
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                                        this.state.toggle_sidebar();
                                                        cx.notify();
                                                    }),
                                                )
                                                .child("◀"),
                                        ),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_1()
                                        .children(get_sidebar_items().into_iter().enumerate().map(|(idx, item)| {
                                            let is_active = self.state.active_view == item.view;
                                            let target_view = item.view;
                                            let nav_id = format!("sidebar_nav_{}", idx);

                                            div()
                                                .id(ElementId::Name(nav_id.into()))
                                                .flex()
                                                .items_center()
                                                .justify_between()
                                                .px_3()
                                                .py_2()
                                                .rounded_md()
                                                .cursor_pointer()
                                                .bg(if is_active { rgb(0xe0f2fe) } else { rgb(0xffffff) })
                                                .border_1()
                                                .border_color(if is_active { rgb(0xbae6fd) } else { rgb(0xffffff) })
                                                .hover(|s| if !is_active { s.bg(rgb(0xf1f5f9)) } else { s })
                                                .on_mouse_down(
                                                    MouseButton::Left,
                                                    cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                                        this.state.active_view = target_view;
                                                        cx.notify();
                                                    }),
                                                )
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_3()
                                                        .child(div().text_size(px(16.0)).child(item.icon))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .font_weight(if is_active {
                                                                    FontWeight::BOLD
                                                                } else {
                                                                    FontWeight::MEDIUM
                                                                })
                                                                .text_color(if is_active {
                                                                    rgb(0x0369a1)
                                                                } else {
                                                                    rgb(0x334155)
                                                                })
                                                                .child(item.title),
                                                        ),
                                                )
                                                .children(item.badge.map(|b| {
                                                    div()
                                                        .px_2()
                                                        .py_0p5()
                                                        .rounded_full()
                                                        .bg(if is_active { rgb(0x0284c7) } else { rgb(0xf1f5f9) })
                                                        .text_size(px(10.0))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(if is_active { rgb(0xffffff) } else { rgb(0x475569) })
                                                        .child(b)
                                                }))
                                        })),
                                ),
                        )
                        .child(
                            div()
                                .id("profile_pill_bottom")
                                .p_3()
                                .rounded_lg()
                                .bg(rgb(0xf8fafc))
                                .border_1()
                                .border_color(rgb(0xe2e8f0))
                                .flex()
                                .items_center()
                                .justify_between()
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0xf1f5f9)))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                        this.state.active_view = ActiveView::Profiles;
                                        cx.notify();
                                    }),
                                )
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_3()
                                        .child(div().text_size(px(22.0)).child(avatar))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .text_size(px(13.0))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(0x0f172a))
                                                        .child(student_name),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(11.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(rgb(0xd97706))
                                                        .child(format!("⭐ {} Stars", current_stars)),
                                                ),
                                        ),
                                )
                                .child(div().text_size(px(12.0)).text_color(rgb(0x94a3b8)).child("⇄")),
                        ),
                )
            } else {
                None
            })
            // 2. Main Viewport
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .flex_col()
                    .bg(rgb(0xf8fafc))
                    // Header Bar (Hidden during typing activities to eliminate clutter)
                    .children(if !is_typing_view {
                        Some(
                            div()
                                .h(px(50.0))
                                .px_6()
                                .bg(rgb(0xffffff))
                                .border_b_1()
                                .border_color(rgb(0xe2e8f0))
                                .flex()
                                .items_center()
                                .justify_between()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_3()
                                        .children(if !is_sidebar_open {
                                            Some(
                                                div()
                                                    .id("btn_expand_sidebar")
                                                    .px_2p5()
                                                    .py_1()
                                                    .rounded_md()
                                                    .bg(rgb(0xf1f5f9))
                                                    .border_1()
                                                    .border_color(rgb(0xe2e8f0))
                                                    .cursor_pointer()
                                                    .hover(|s| s.bg(rgb(0xe2e8f0)))
                                                    .text_size(px(12.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0x0369a1))
                                                    .on_mouse_down(
                                                        MouseButton::Left,
                                                        cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                                            this.state.toggle_sidebar();
                                                            cx.notify();
                                                        }),
                                                    )
                                                    .child("☰ Menu"),
                                            )
                                        } else {
                                            None
                                        })
                                        .child(
                                            div()
                                                .text_size(px(15.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0x0369a1))
                                                .child(match active_view {
                                                    ActiveView::CourseOverview => "Course Curriculum",
                                                    ActiveView::TypingArena => "Typing Arena",
                                                    ActiveView::ExerciseResults => "Exercise Results",
                                                    ActiveView::SmartReview => "Smart Review",
                                                    ActiveView::StoryStudio => "Story Studio",
                                                    ActiveView::Trophies => "Trophy Room",
                                                    ActiveView::TypingTests => "Typing Tests",
                                                    ActiveView::DiplomaView => "Certificate Diploma",
                                                    ActiveView::GamesHub => "Arcade Mini-Games",
                                                    ActiveView::GameBubbles => "Game: Bubbles",
                                                    ActiveView::GameWordTris => "Game: WordTris",
                                                    ActiveView::GameClouds => "Game: Clouds",
                                                    ActiveView::GameAbc => "Game: ABC Sprint",
                                                    ActiveView::Statistics => "Statistics & Heatmap",
                                                    ActiveView::Profiles => "Student Profiles",
                                                    ActiveView::Settings => "Settings",
                                                }),
                                        ),
                                )
                                .child(
                                    div()
                                        .px_3()
                                        .py_1()
                                        .rounded_md()
                                        .bg(rgb(0xf1f5f9))
                                        .border_1()
                                        .border_color(rgb(0xe2e8f0))
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(0x475569))
                                        .child("Offline 🔌"),
                                ),
                        )
                    } else {
                        None
                    })
                    // Main View Content
                    .child(
                        div()
                            .flex_1()
                            .p_5()
                            .overflow_hidden()
                            .child(self.render_active_view(cx)),
                    ),
            )
    }
}

impl TypeStudentView {
    fn render_active_view(&mut self, cx: &mut Context<Self>) -> AnyElement {
        match self.state.active_view {
            ActiveView::CourseOverview => render_course_overview(self, cx),
            ActiveView::TypingArena => render_typing_arena(self, cx),
            ActiveView::ExerciseResults => render_exercise_results(self, cx),
            ActiveView::SmartReview => render_smart_review(self, cx),
            ActiveView::StoryStudio => render_story_studio(self, cx),
            ActiveView::Trophies => render_trophies(self, cx),
            ActiveView::TypingTests => render_typing_tests(self, cx),
            ActiveView::DiplomaView => render_diploma(self, cx),
            ActiveView::GamesHub => self.render_games_hub(cx),
            ActiveView::GameBubbles => self.render_game_bubbles(cx),
            ActiveView::GameWordTris => self.render_game_wordtris(cx),
            ActiveView::GameClouds => self.render_game_clouds(cx),
            ActiveView::GameAbc => self.render_game_abc(cx),
            ActiveView::Statistics => render_statistics(self, cx),
            ActiveView::Profiles => render_profiles(self, cx),
            ActiveView::Settings => render_settings(self, cx),
        }
    }

    fn render_games_hub(&self, cx: &mut Context<Self>) -> AnyElement {
        let cards = get_game_cards();

        div()
            .flex()
            .flex_col()
            .gap_5()
            .child(
                div()
                    .p_5()
                    .rounded_xl()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe2e8f0))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .gap_1()
                            .child(
                                div()
                                    .text_size(px(18.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0f172a))
                                    .child("🎮 The 4 Classic TypingMaster Games"),
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x64748b))
                                    .child("Reinforce typing muscle memory and reflexes through arcade mini games."),
                            ),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_3p5()
                    .children(cards.into_iter().enumerate().map(|(idx, card)| {
                        let target_view = card.view;
                        let play_btn_id = format!("play_game_btn_{}", idx);

                        div()
                            .flex()
                            .flex_col()
                            .justify_between()
                            .w(px(310.0))
                            .p_4()
                            .rounded_xl()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(rgb(0xe2e8f0))
                            .hover(|s| s.bg(rgb(0xf8fafc)).border_color(rgb(0x38bdf8)))
                            .gap_3()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1p5()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2p5()
                                            .child(div().text_size(px(28.0)).child(card.icon))
                                            .child(
                                                div()
                                                    .text_size(px(16.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0x0f172a))
                                                    .child(card.title),
                                            ),
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(rgb(0x64748b))
                                            .child(card.description),
                                    ),
                            )
                            .child(
                                div()
                                    .flex()
                                    .justify_between()
                                    .items_center()
                                    .child(
                                        div()
                                            .text_size(px(10.0))
                                            .font_weight(FontWeight::SEMIBOLD)
                                            .text_color(rgb(0xd97706))
                                            .child(card.difficulty),
                                    )
                                    .child(
                                        div()
                                            .id(ElementId::Name(play_btn_id.into()))
                                            .px_3p5()
                                            .py_1()
                                            .rounded_lg()
                                            .bg(rgb(0x0284c7))
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xffffff))
                                            .cursor_pointer()
                                            .on_mouse_down(
                                                MouseButton::Left,
                                                cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                                    this.state.active_view = target_view;
                                                    cx.notify();
                                                }),
                                            )
                                            .child("Play ▶"),
                                    ),
                            )
                    })),
            )
            .into_any_element()
    }

    fn render_game_bubbles(&self, cx: &mut Context<Self>) -> AnyElement {
        let game = &self.state.bubbles_game;

        div()
            .flex()
            .flex_col()
            .gap_3p5()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p_3p5()
                    .rounded_xl()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe2e8f0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2p5()
                            .child(div().text_size(px(22.0)).child("🫧"))
                            .child(div().text_size(px(16.0)).font_weight(FontWeight::BOLD).child("Bubbles")),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xd97706))
                                    .child(format!("Score: {}", game.score)),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0284c7))
                                    .child(format!("Streak: {}", game.streak)),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0xdc2626))
                                    .child(format!("Lives: {}", "❤️".repeat(game.lives as usize))),
                            ),
                    ),
            )
            .child(
                div()
                    .h(px(380.0))
                    .rounded_2xl()
                    .bg(rgb(0xe0f2fe))
                    .border_2()
                    .border_color(rgb(0x38bdf8))
                    .relative()
                    .overflow_hidden()
                    .children(game.bubbles.iter().map(|b| {
                        let left_px = b.x * 650.0;
                        let top_px = b.y * 340.0;

                        div()
                            .absolute()
                            .left(px(left_px))
                            .top(px(top_px))
                            .px_3()
                            .py_1()
                            .rounded_full()
                            .bg(rgb(0x0284c7))
                            .border_2()
                            .border_color(rgb(0xffffff))
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .child(b.text.clone())
                    })),
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x64748b))
                            .child("Type the letter or word in the bubble to pop it before it reaches the floor."),
                    )
                    .child(
                        div()
                            .id("btn_restart_bubbles")
                            .px_3p5()
                            .py_1()
                            .rounded_md()
                            .bg(rgb(0xf1f5f9))
                            .border_1()
                            .border_color(rgb(0xcbd5e1))
                            .text_size(px(11.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x334155))
                            .cursor_pointer()
                            .on_mouse_down(
                                MouseButton::Left,
                                cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                    this.state.bubbles_game.restart();
                                    cx.notify();
                                }),
                            )
                            .child("Restart Game ↺"),
                    ),
            )
            .into_any_element()
    }

    fn render_game_wordtris(&self, _cx: &mut Context<Self>) -> AnyElement {
        let game = &self.state.wordtris_game;

        div()
            .flex()
            .flex_col()
            .gap_3p5()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p_3p5()
                    .rounded_xl()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe2e8f0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2p5()
                            .child(div().text_size(px(22.0)).child("🧱"))
                            .child(div().text_size(px(16.0)).font_weight(FontWeight::BOLD).child("WordTris")),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xd97706))
                                    .child(format!("Score: {}", game.score)),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0284c7))
                                    .child(format!("Cleared: {}", game.lines_cleared)),
                            ),
                    ),
            )
            .child(
                div()
                    .h(px(350.0))
                    .rounded_2xl()
                    .bg(rgb(0xf8fafc))
                    .border_2()
                    .border_color(rgb(0xe2e8f0))
                    .flex()
                    .justify_around()
                    .p_3()
                    .children((0..game.columns).map(|col_idx| {
                        let height = game.stack_heights[col_idx];
                        let danger = height >= 4;

                        div()
                            .flex()
                            .flex_col_reverse()
                            .w(px(100.0))
                            .h_full()
                            .rounded_lg()
                            .bg(rgb(0xffffff))
                            .border_1()
                            .border_color(if danger {
                                rgb(0xf87171)
                            } else {
                                rgb(0xcbd5e1)
                            })
                            .p_1p5()
                            .gap_1p5()
                            .children((0..height).map(|_| {
                                div()
                                    .w_full()
                                    .h(px(26.0))
                                    .rounded_md()
                                    .bg(if danger {
                                        rgb(0xf87171)
                                    } else {
                                        rgb(0x0284c7)
                                    })
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .text_size(px(10.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffffff))
                                    .child("🧱 BEAM")
                            }))
                    })),
            )
            .into_any_element()
    }

    fn render_game_clouds(&self, _cx: &mut Context<Self>) -> AnyElement {
        let game = &self.state.clouds_game;

        div()
            .flex()
            .flex_col()
            .gap_3p5()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p_3p5()
                    .rounded_xl()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe2e8f0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2p5()
                            .child(div().text_size(px(22.0)).child("☁️"))
                            .child(div().text_size(px(16.0)).font_weight(FontWeight::BOLD).child("Clouds")),
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xd97706))
                                    .child(format!("Score: {}", game.score)),
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xdc2626))
                                    .child(format!("Misses: {}/5", game.misses)),
                            ),
                    ),
            )
            .child(
                div()
                    .h(px(350.0))
                    .rounded_2xl()
                    .bg(rgb(0xbae6fd))
                    .border_2()
                    .border_color(rgb(0x7dd3fc))
                    .relative()
                    .overflow_hidden()
                    .children(game.clouds.iter().map(|c| {
                        let left_px = c.x * 650.0;
                        let top_px = c.y * 280.0;

                        div()
                            .absolute()
                            .left(px(left_px))
                            .top(px(top_px))
                            .px_3p5()
                            .py_1p5()
                            .rounded_full()
                            .bg(if c.is_sunny {
                                rgb(0xfef08a)
                            } else {
                                rgb(0xffffff)
                            })
                            .text_color(rgb(0x0f172a))
                            .text_size(px(13.0))
                            .font_weight(FontWeight::BOLD)
                            .child(format!("☁️ {}", c.text))
                    })),
            )
            .into_any_element()
    }

    fn render_game_abc(&self, cx: &mut Context<Self>) -> AnyElement {
        let game = &self.state.abc_game;
        let current_target = game
            .current_char()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_else(|| "Done".to_string());
        let elapsed = game.elapsed_secs();

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_5()
            .p_6()
            .rounded_2xl()
            .bg(rgb(0xffffff))
            .border_1()
            .border_color(rgb(0xe2e8f0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_1()
                    .child(div().text_size(px(36.0)).child("⚡"))
                    .child(div().text_size(px(20.0)).font_weight(FontWeight::BOLD).child("ABC Alphabet Sprint"))
                    .child(
                        div()
                            .text_size(px(12.0))
                            .text_color(rgb(0x64748b))
                            .child("Type A to Z in sequence."),
                    ),
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .px_8()
                    .py_4()
                    .rounded_2xl()
                    .bg(rgb(0xf8fafc))
                    .border_2()
                    .border_color(rgb(0x38bdf8))
                    .gap_1()
                    .child(div().text_size(px(12.0)).text_color(rgb(0x64748b)).child("NEXT LETTER"))
                    .child(
                        div()
                            .text_size(px(54.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x0284c7))
                            .child(current_target),
                    ),
            )
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(metric_tile("TIME", format!("{:.2}s", elapsed), rgb(0xd97706)))
                    .child(metric_tile("MISTAKES", format!("{}", game.mistakes), rgb(0xdc2626)))
                    .child(metric_tile("PROGRESS", format!("{}/26", game.current_idx), rgb(0x15803d))),
            )
            .child(
                div()
                    .id("btn_restart_abc")
                    .px_5()
                    .py_2()
                    .rounded_lg()
                    .bg(rgb(0x0284c7))
                    .text_size(px(12.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .cursor_pointer()
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                            this.state.abc_game.restart();
                            cx.notify();
                        }),
                    )
                    .child("Restart Sprint ↺"),
            )
            .into_any_element()
    }
}

fn metric_tile(label: &'static str, value: String, accent: Rgba) -> Div {
    div()
        .flex()
        .flex_col()
        .items_center()
        .px_4()
        .py_2()
        .rounded_xl()
        .bg(rgb(0xf8fafc))
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .child(
            div()
                .text_size(px(18.0))
                .font_weight(FontWeight::BOLD)
                .text_color(accent)
                .child(value),
        )
        .child(
            div()
                .text_size(px(10.0))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(0x64748b))
                .child(label),
        )
}
