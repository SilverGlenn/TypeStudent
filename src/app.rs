use type_student::components::keyboard::{get_keyboard_layout, Finger};
use type_student::components::hands::{HandFingerState, HandsGuideModel};
use type_student::engine::CharStatus;
use type_student::state::{ActiveView, AppState};
use type_student::trophies::get_all_trophies;
use type_student::views::diploma_export::export_diploma_html;
use type_student::views::games_view::get_game_cards;
use type_student::views::sidebar::get_sidebar_items;
use type_student::views::stories_data::get_all_stories;
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

        if key == "backspace" {
            return Some('\x08');
        }
        if key == "space" || key == " " {
            return Some(' ');
        }
        if key == "enter" || key == "return" {
            return Some('\n');
        }
        if key.len() == 1 {
            let c = key.chars().next().unwrap();
            if shift {
                if c.is_ascii_lowercase() {
                    return Some(c.to_ascii_uppercase());
                }
                let shifted = match c {
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
                return Some(shifted);
            } else {
                return Some(c);
            }
        }
        None
    }
}

impl Render for TypeStudentView {
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // Ensure this view retains focus for keyboard events
        window.focus(&self.focus_handle);

        let active_view = self.state.active_view;
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
            .bg(rgb(0xf8fafc)) // Clean, soothing soft slate white
            .text_color(rgb(0x0f172a)) // Crisp readable charcoal
            .font_family(".SystemUIFont")
            .key_context("type_student_view")
            .on_key_down(cx.listener(|this, event: &KeyDownEvent, _window, cx| {
                if let Some(ch) = Self::parse_keystroke(event) {
                    if ch == '\x08' {
                        this.state.on_backspace();
                    } else {
                        this.state.on_keystroke(ch);
                    }
                    cx.notify();
                }
            }))
            .child(
                // 1. Sidebar (Clean soft light blue-gray)
                div()
                    .w(px(260.0))
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
                            // App Branding / Logo
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .child(
                                        div()
                                            .flex()
                                            .items_center()
                                            .gap_2()
                                            .child(div().text_size(px(24.0)).child("⌨️"))
                                            .child(
                                                div()
                                                    .text_size(px(18.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0x0369a1))
                                                    .child("TypeStudent Pro")
                                            )
                                    )
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .text_color(rgb(0x64748b))
                                            .child("Touch Typing Tutor & Educational Games")
                                    )
                            )
                            // Navigation Links
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .gap_1()
                                    .children(
                                        get_sidebar_items().into_iter().enumerate().map(|(idx, item)| {
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
                                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                                    this.state.active_view = target_view;
                                                    cx.notify();
                                                }))
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_3()
                                                        .child(div().text_size(px(16.0)).child(item.icon))
                                                        .child(
                                                            div()
                                                                .text_size(px(13.0))
                                                                .font_weight(if is_active { FontWeight::BOLD } else { FontWeight::MEDIUM })
                                                                .text_color(if is_active { rgb(0x0369a1) } else { rgb(0x334155) })
                                                                .child(item.title)
                                                        )
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
                                        })
                                    )
                            )
                    )
                    // Profile card in bottom sidebar
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
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                this.state.active_view = ActiveView::Profiles;
                                cx.notify();
                            }))
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_3()
                                    .child(div().text_size(px(22.0)).child(avatar.clone()))
                                    .child(
                                        div()
                                            .flex()
                                            .flex_col()
                                            .child(
                                                div()
                                                    .text_size(px(13.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0x0f172a))
                                                    .child(student_name.clone())
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.0))
                                                    .font_weight(FontWeight::SEMIBOLD)
                                                    .text_color(rgb(0xd97706))
                                                    .child(format!("⭐ {} Stars", current_stars))
                                            )
                                    )
                            )
                            .child(div().text_size(px(12.0)).text_color(rgb(0x94a3b8)).child("⇄"))
                    )
            )
            .child(
                // 2. Main Content Area
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .flex_col()
                    .bg(rgb(0xf8fafc))
                    // Top Status Bar
                    .child(
                        div()
                            .h(px(52.0))
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
                                    .child(
                                        div()
                                            .text_size(px(14.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x0369a1))
                                            .child(match active_view {
                                                ActiveView::CourseOverview => "Touch Typing Course Curriculum",
                                                ActiveView::TypingArena => "Typing Arena & Finger Guide",
                                                ActiveView::ExerciseResults => "Exercise Results & Performance",
                                                ActiveView::SmartReview => "Smart Review - Weak Keys Practice",
                                                ActiveView::StoryStudio => "Story Studio - Creative Reading & Typing",
                                                ActiveView::Trophies => "Trophy Room & Achievement Badges",
                                                ActiveView::TypingTests => "Typing Tests & Official Diplomas",
                                                ActiveView::DiplomaView => "Certificate of Achievement",
                                                ActiveView::GamesHub => "Typing Mini Games",
                                                ActiveView::GameBubbles => "Game: Bubbles",
                                                ActiveView::GameWordTris => "Game: WordTris",
                                                ActiveView::GameClouds => "Game: Clouds",
                                                ActiveView::GameAbc => "Game: ABC Sprint",
                                                ActiveView::Statistics => "Student Statistics & Heatmap",
                                                ActiveView::Profiles => "Student Profiles Switcher",
                                                ActiveView::Settings => "Settings & Sound",
                                            })
                                    )
                            )
                            .child(
                                div()
                                    .flex()
                                    .items_center()
                                    .gap_4()
                                    .child(
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded_md()
                                            .bg(rgb(0xf1f5f9))
                                            .border_1()
                                            .border_color(rgb(0xe2e8f0))
                                            .text_size(px(12.0))
                                            .text_color(rgb(0x475569))
                                            .child("Offline Mode Active 🔌")
                                    )
                            )
                    )
                    // View Body
                    .child(
                        div()
                            .flex_1()
                            .p_6()
                            .overflow_hidden()
                            .child(self.render_active_view(cx))
                    )
            )
    }
}

impl TypeStudentView {
    fn render_active_view(&mut self, cx: &mut Context<Self>) -> AnyElement {
        match self.state.active_view {
            ActiveView::CourseOverview => self.render_course_overview(cx),
            ActiveView::TypingArena => self.render_typing_arena(cx),
            ActiveView::ExerciseResults => self.render_exercise_results(cx),
            ActiveView::SmartReview => self.render_smart_review(cx),
            ActiveView::StoryStudio => self.render_story_studio(cx),
            ActiveView::Trophies => self.render_trophies(cx),
            ActiveView::TypingTests => self.render_typing_tests(cx),
            ActiveView::DiplomaView => self.render_diploma(cx),
            ActiveView::GamesHub => self.render_games_hub(cx),
            ActiveView::GameBubbles => self.render_game_bubbles(cx),
            ActiveView::GameWordTris => self.render_game_wordtris(cx),
            ActiveView::GameClouds => self.render_game_clouds(cx),
            ActiveView::GameAbc => self.render_game_abc(cx),
            ActiveView::Statistics => self.render_statistics(cx),
            ActiveView::Profiles => self.render_profiles(cx),
            ActiveView::Settings => self.render_settings(cx),
        }
    }

    // 1. Course Overview View
    fn render_course_overview(&self, cx: &mut Context<Self>) -> AnyElement {
        let lessons = self.state.lessons.clone();
        let profile = self.state.profile_store.active_profile();

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
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
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0f172a))
                                    .child("12-Lesson Touch Typing Course")
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x64748b))
                                    .child("Step-by-step muscle memory training. Learn every key with correct finger placement!")
                            )
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_4()
                    .children(
                        lessons.into_iter().enumerate().map(|(l_idx, lesson)| {
                            let keys_str: String = lesson.keys_introduced.iter().map(|c| c.to_uppercase().to_string()).collect::<Vec<_>>().join(" ");

                            div()
                                .flex()
                                .flex_col()
                                .p_5()
                                .rounded_xl()
                                .bg(rgb(0xffffff))
                                .border_1()
                                .border_color(rgb(0xe2e8f0))
                                .gap_3()
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_3()
                                                .child(
                                                    div()
                                                        .w(px(32.0))
                                                        .h(px(32.0))
                                                        .rounded_full()
                                                        .bg(rgb(0x0284c7))
                                                        .flex()
                                                        .items_center()
                                                        .justify_center()
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_size(px(14.0))
                                                        .text_color(rgb(0xffffff))
                                                        .child(format!("{}", lesson.number))
                                                )
                                                .child(
                                                    div()
                                                        .flex()
                                                        .flex_col()
                                                        .child(
                                                            div()
                                                                .text_size(px(16.0))
                                                                .font_weight(FontWeight::BOLD)
                                                                .text_color(rgb(0x0f172a))
                                                                .child(format!("Lesson {}: {}", lesson.number, lesson.title))
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(px(12.0))
                                                                .text_color(rgb(0x64748b))
                                                                .child(lesson.subtitle.clone())
                                                        )
                                                )
                                        )
                                        .child(
                                            div()
                                                .px_3()
                                                .py_1()
                                                .rounded_md()
                                                .bg(rgb(0xf1f5f9))
                                                .border_1()
                                                .border_color(rgb(0xe2e8f0))
                                                .text_size(px(12.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0x0369a1))
                                                .child(format!("Keys: {}", keys_str))
                                        )
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_wrap()
                                        .gap_2()
                                        .children(
                                            lesson.exercises.into_iter().enumerate().map(|(e_idx, ex)| {
                                                let record_key = format!("lesson_{}/{}", l_idx + 1, ex.id);
                                                let record = profile.and_then(|p| p.lesson_records.get(&record_key));
                                                let stars_str = match record.map(|r| r.stars).unwrap_or(0) {
                                                    5 => "⭐⭐⭐⭐⭐",
                                                    4 => "⭐⭐⭐⭐",
                                                    3 => "⭐⭐⭐",
                                                    2 => "⭐⭐",
                                                    1 => "⭐",
                                                    _ => "⚪⚪⚪⚪⚪",
                                                };
                                                let ex_btn_id = format!("ex_btn_{}_{}", l_idx, e_idx);

                                                div()
                                                    .id(ElementId::Name(ex_btn_id.into()))
                                                    .flex()
                                                    .items_center()
                                                    .justify_between()
                                                    .w(px(260.0))
                                                    .p_3()
                                                    .rounded_lg()
                                                    .bg(rgb(0xf8fafc))
                                                    .border_1()
                                                    .border_color(if record.is_some() { rgb(0x38bdf8) } else { rgb(0xe2e8f0) })
                                                    .cursor_pointer()
                                                    .hover(|s| s.bg(rgb(0xe0f2fe)))
                                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                                        this.state.start_exercise(l_idx, e_idx);
                                                        cx.notify();
                                                    }))
                                                    .child(
                                                        div()
                                                            .flex()
                                                            .flex_col()
                                                            .gap_1()
                                                            .child(
                                                                div()
                                                                    .flex()
                                                                    .items_center()
                                                                    .gap_1p5()
                                                                    .child(div().text_size(px(13.0)).child(ex.exercise_type.icon()))
                                                                    .child(
                                                                        div()
                                                                            .text_size(px(13.0))
                                                                            .font_weight(FontWeight::SEMIBOLD)
                                                                            .text_color(rgb(0x0f172a))
                                                                            .child(ex.title)
                                                                    )
                                                            )
                                                            .child(
                                                                div()
                                                                    .text_size(px(11.0))
                                                                    .text_color(rgb(0xd97706))
                                                                    .child(stars_str)
                                                            )
                                                    )
                                                    .child(
                                                        div()
                                                            .px_2()
                                                            .py_1()
                                                            .rounded_md()
                                                            .bg(rgb(0x0284c7))
                                                            .text_size(px(11.0))
                                                            .font_weight(FontWeight::BOLD)
                                                            .text_color(rgb(0xffffff))
                                                            .child("Start ▶")
                                                    )
                                            })
                                        )
                                )
                        })
                    )
            )
            .into_any_element()
    }

    // 2. Live Interactive Typing Arena with Keyboard & Hands Finger Guide
    fn render_typing_arena(&self, _cx: &mut Context<Self>) -> AnyElement {
        let session = match &self.state.typing_session {
            Some(s) => s,
            None => return div().child("No active session").into_any_element(),
        };
        let info = self.state.current_exercise_info.as_ref();
        let title = info.map(|i| i.title.clone()).unwrap_or_else(|| "Exercise".to_string());
        let instruction = info.map(|i| i.instruction.clone()).unwrap_or_default();
        
        let net_wpm = session.net_wpm();
        let accuracy = session.accuracy_percent();
        let errors = session.error_keystrokes;
        let progress = session.progress_ratio() * 100.0;
        let active_char = session.current_char();
        let active_finger = self.state.active_finger();
        let hands_model = HandsGuideModel::for_active_target(active_finger, active_char);

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                // Exercise Header & HUD
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p_4()
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
                                    .child(title)
                            )
                            .child(
                                div()
                                    .text_size(px(12.0))
                                    .text_color(rgb(0x64748b))
                                    .child(instruction)
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            // Net WPM Gauge
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .px_4()
                                    .py_1p5()
                                    .rounded_lg()
                                    .bg(rgb(0xf0fdf4))
                                    .border_1()
                                    .border_color(rgb(0xbbf7d0))
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x15803d))
                                            .child(format!("{:.0}", net_wpm))
                                    )
                                    .child(
                                        div()
                                            .text_size(px(10.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x166534))
                                            .child("NET WPM")
                                    )
                            )
                            // Accuracy Gauge
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .px_4()
                                    .py_1p5()
                                    .rounded_lg()
                                    .bg(rgb(0xf0f9ff))
                                    .border_1()
                                    .border_color(rgb(0xbae6fd))
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x0369a1))
                                            .child(format!("{:.0}%", accuracy))
                                    )
                                    .child(
                                        div()
                                            .text_size(px(10.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x075985))
                                            .child("ACCURACY")
                                    )
                            )
                            // Errors Counter
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .px_3()
                                    .py_1p5()
                                    .rounded_lg()
                                    .bg(if errors > 0 { rgb(0xfef2f2) } else { rgb(0xf8fafc) })
                                    .border_1()
                                    .border_color(if errors > 0 { rgb(0xfecaca) } else { rgb(0xe2e8f0) })
                                    .child(
                                        div()
                                            .text_size(px(18.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(if errors > 0 { rgb(0xdc2626) } else { rgb(0x64748b) })
                                            .child(format!("{}", errors))
                                    )
                                    .child(
                                        div()
                                            .text_size(px(10.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x64748b))
                                            .child("ERRORS")
                                    )
                            )
                    )
            )
            // Progress Bar
            .child(
                div()
                    .w_full()
                    .h(px(5.0))
                    .rounded_full()
                    .bg(rgb(0xe2e8f0))
                    .child(
                        div()
                            .h_full()
                            .rounded_full()
                            .bg(rgb(0x0284c7))
                            .w(px((progress * 7.5).max(4.0)))
                    )
            )
            // Typing Text Box Area (Clean white card with soothing contrast)
            .child(
                div()
                    .p_5()
                    .rounded_xl()
                    .bg(rgb(0xffffff))
                    .border_2()
                    .border_color(rgb(0x38bdf8))
                    .flex()
                    .flex_wrap()
                    .gap_1()
                    .text_size(px(22.0))
                    .font_weight(FontWeight::MEDIUM)
                    .line_height(px(32.0))
                    .children(
                        session.target_chars.iter().enumerate().map(|(idx, &ch)| {
                            let status = session.char_statuses[idx];
                            let is_cursor = idx == session.cursor_idx;

                            let display_char = if ch == ' ' { "␣" } else { &ch.to_string() };

                            let (text_color, bg_color) = match status {
                                CharStatus::Correct => (rgb(0x15803d), rgb(0xdcfce7)), // Soothing soft green
                                CharStatus::Incorrect(_) => (rgb(0xb91c1c), rgb(0xfee2e2)), // Soft red
                                CharStatus::Pending => {
                                    if is_cursor {
                                        (rgb(0xffffff), rgb(0x0284c7)) // Focus blue cursor
                                    } else {
                                        (rgb(0x475569), rgb(0xffffff)) // Readable slate
                                    }
                                }
                            };

                            div()
                                .px_1()
                                .rounded_sm()
                                .bg(bg_color)
                                .text_color(text_color)
                                .border_b_2()
                                .border_color(if is_cursor { rgb(0x0369a1) } else { rgb(0xffffff) })
                                .child(display_char.to_string())
                        })
                    )
            )
            // Live Finger Guidance Prompt Banner
            .child(
                div()
                    .flex()
                    .items_center()
                    .justify_between()
                    .px_4()
                    .py_2()
                    .rounded_lg()
                    .bg(rgb(0xe0f2fe))
                    .border_1()
                    .border_color(rgb(0xbae6fd))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .child(div().text_size(px(16.0)).child("👉"))
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0369a1))
                                    .child(hands_model.active_finger_instruction())
                            )
                    )
                    .child(
                        div()
                            .text_size(px(11.0))
                            .text_color(rgb(0x0369a1))
                            .child("Keep eyes on screen • Fingers on Home Row (ASDF - JKL;)")
                    )
            )
            // Interactive Visual Keyboard (Color-Coded Finger Zones)
            .child(
                self.render_visual_keyboard(active_char, active_finger)
            )
            // Realistic Visual Hands (Anatomical Left & Right Hands with Animated Active Finger)
            .child(
                self.render_visual_hands(&hands_model)
            )
            .into_any_element()
    }

    // Visual Keyboard Component (Color-Coded Matching Finger Zones)
    fn render_visual_keyboard(&self, active_char: Option<char>, _active_finger: Option<Finger>) -> impl IntoElement {
        let layout = get_keyboard_layout();
        let target_lower = active_char.map(|c| c.to_ascii_lowercase());

        div()
            .p_3()
            .rounded_xl()
            .bg(rgb(0xffffff))
            .border_1()
            .border_color(rgb(0xe2e8f0))
            .flex()
            .flex_col()
            .gap_1()
            .children(
                layout.into_iter().map(|row| {
                    div()
                        .flex()
                        .gap_1()
                        .justify_center()
                        .children(
                            row.into_iter().map(|key| {
                                let is_active = target_lower.map_or(false, |t| {
                                    key.char_val == t || (t == ' ' && key.char_val == ' ')
                                });

                                let (f_r, f_g, f_b) = key.finger.rgb();
                                let finger_color = rgb((f_r as u32) << 16 | (f_g as u32) << 8 | (f_b as u32));

                                let (bg_color, text_color, border_bottom) = if is_active {
                                    (finger_color, rgb(0xffffff), rgb(0x0f172a))
                                } else {
                                    (rgb(0xf8fafc), rgb(0x1e293b), finger_color)
                                };

                                let width_px = key.width_units * 39.0;

                                div()
                                    .w(px(width_px))
                                    .h(px(36.0))
                                    .rounded_md()
                                    .bg(bg_color)
                                    .border_1()
                                    .border_color(if is_active { rgb(0x0f172a) } else { rgb(0xcbd5e1) })
                                    .border_b_3()
                                    .border_color(border_bottom)
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_size(px(12.0))
                                            .font_weight(if is_active { FontWeight::BOLD } else { FontWeight::SEMIBOLD })
                                            .text_color(text_color)
                                            .child(key.label)
                                    )
                            })
                        )
                })
            )
    }

    // Authentic Hand Shapes & Animated Finger Guides (Classic TypingMaster Style)
    fn render_visual_hands(&self, hands: &HandsGuideModel) -> impl IntoElement {
        div()
            .p_4()
            .rounded_xl()
            .bg(rgb(0xffffff))
            .border_1()
            .border_color(rgb(0xe2e8f0))
            .flex()
            .justify_around()
            .items_end()
            .child(
                // Left Hand Container
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x0369a1))
                            .child("Left Hand (Home: A S D F)")
                    )
                    .child(
                        // Anatomical Hand Structure
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            // Rising Fingers Row
                            .child(
                                div()
                                    .flex()
                                    .items_end()
                                    .gap_1p5()
                                    .child(self.render_finger_pillar(hands.left_pinky, hands.target_char))
                                    .child(self.render_finger_pillar(hands.left_ring, hands.target_char))
                                    .child(self.render_finger_pillar(hands.left_middle, hands.target_char))
                                    .child(self.render_finger_pillar(hands.left_index, hands.target_char))
                                    .child(self.render_finger_pillar(hands.left_thumb, hands.target_char))
                            )
                            // Palm Base
                            .child(
                                div()
                                    .w(px(175.0))
                                    .h(px(45.0))
                                    .rounded_b_2xl()
                                    .bg(rgb(0xf1f5f9))
                                    .border_1()
                                    .border_color(rgb(0xcbd5e1))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x64748b))
                                            .child("LEFT PALM")
                                    )
                            )
                    )
            )
            .child(
                // Right Hand Container
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_1()
                    .child(
                        div()
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x0369a1))
                            .child("Right Hand (Home: J K L ;)")
                    )
                    .child(
                        // Anatomical Hand Structure
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            // Rising Fingers Row
                            .child(
                                div()
                                    .flex()
                                    .items_end()
                                    .gap_1p5()
                                    .child(self.render_finger_pillar(hands.right_thumb, hands.target_char))
                                    .child(self.render_finger_pillar(hands.right_index, hands.target_char))
                                    .child(self.render_finger_pillar(hands.right_middle, hands.target_char))
                                    .child(self.render_finger_pillar(hands.right_ring, hands.target_char))
                                    .child(self.render_finger_pillar(hands.right_pinky, hands.target_char))
                            )
                            // Palm Base
                            .child(
                                div()
                                    .w(px(175.0))
                                    .h(px(45.0))
                                    .rounded_b_2xl()
                                    .bg(rgb(0xf1f5f9))
                                    .border_1()
                                    .border_color(rgb(0xcbd5e1))
                                    .flex()
                                    .items_center()
                                    .justify_center()
                                    .child(
                                        div()
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x64748b))
                                            .child("RIGHT PALM")
                                    )
                            )
                    )
            )
    }

    // Individual Finger Pillar with Animated Extension and Glowing Fingertip
    fn render_finger_pillar(&self, finger: HandFingerState, target_char: Option<char>) -> impl IntoElement {
        let height = if finger.is_active { finger.active_height } else { finger.normal_height };
        let width = finger.width;

        let (f_r, f_g, f_b) = finger.finger.rgb();
        let finger_color = rgb((f_r as u32) << 16 | (f_g as u32) << 8 | (f_b as u32));

        let bg_color = if finger.is_active {
            finger_color
        } else {
            rgb(0xf8fafc)
        };

        let border_color = if finger.is_active {
            rgb(0x0f172a)
        } else {
            rgb(0xcbd5e1)
        };

        let tip_label = if finger.is_active {
            match target_char {
                Some(' ') => "␣".to_string(),
                Some(c) => c.to_uppercase().to_string(),
                None => finger.home_key.to_string(),
            }
        } else {
            finger.home_key.to_string()
        };

        div()
            .w(px(width))
            .h(px(height))
            .rounded_t_full()
            .bg(bg_color)
            .border_2()
            .border_color(border_color)
            .flex()
            .flex_col()
            .items_center()
            .justify_between()
            .py_1p5()
            // Fingertip Cap (Resting Key or Target Key)
            .child(
                div()
                    .w(px(22.0))
                    .h(px(22.0))
                    .rounded_full()
                    .bg(if finger.is_active { rgb(0xffffff) } else { rgb(0xe2e8f0) })
                    .border_1()
                    .border_color(if finger.is_active { rgb(0x0f172a) } else { rgb(0x94a3b8) })
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(11.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(if finger.is_active { finger_color } else { rgb(0x334155) })
                            .child(tip_label)
                    )
            )
            // Finger Name Label
            .child(
                div()
                    .text_size(px(9.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(if finger.is_active { rgb(0xffffff) } else { rgb(0x64748b) })
                    .child(finger.label)
            )
    }

    // 3. Exercise Results Screen
    fn render_exercise_results(&self, cx: &mut Context<Self>) -> AnyElement {
        let session = match &self.state.typing_session {
            Some(s) => s,
            None => return div().child("No result available").into_any_element(),
        };
        let info = self.state.current_exercise_info.as_ref();
        let title = info.map(|i| i.title.clone()).unwrap_or_else(|| "Exercise Completed".to_string());
        let target_wpm = self.state.profile_store.active_profile().map(|p| p.settings.target_wpm).unwrap_or(35);
        let stars = session.calculate_stars(target_wpm);
        let net_wpm = session.net_wpm();
        let gross_wpm = session.gross_wpm();
        let accuracy = session.accuracy_percent();
        let errors = session.error_keystrokes;
        let elapsed = session.elapsed_seconds();

        let stars_visual = match stars {
            5 => "⭐⭐⭐⭐⭐ (Outstanding!)",
            4 => "⭐⭐⭐⭐ (Great Job!)",
            3 => "⭐⭐⭐ (Good Effort!)",
            2 => "⭐⭐ (Keep Practicing)",
            _ => "⭐ (Try Again)",
        };

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_6()
            .p_8()
            .rounded_2xl()
            .bg(rgb(0xffffff))
            .border_1()
            .border_color(rgb(0xe2e8f0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(div().text_size(px(48.0)).child("🎉"))
                    .child(
                        div()
                            .text_size(px(24.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x0f172a))
                            .child(title)
                    )
                    .child(
                        div()
                            .text_size(px(18.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xd97706))
                            .child(stars_visual)
                    )
            )
            // Performance Metrics Grid
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(self.render_stat_box("NET SPEED", &format!("{:.1} WPM", net_wpm), rgb(0x0284c7)))
                    .child(self.render_stat_box("GROSS SPEED", &format!("{:.1} WPM", gross_wpm), rgb(0x64748b)))
                    .child(self.render_stat_box("ACCURACY", &format!("{:.1}%", accuracy), rgb(0x15803d)))
                    .child(self.render_stat_box("ERRORS", &format!("{}", errors), if errors > 0 { rgb(0xdc2626) } else { rgb(0x15803d) }))
                    .child(self.render_stat_box("TIME", &format!("{:.0}s", elapsed), rgb(0xd97706)))
            )
            // Action Buttons
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(
                        div()
                            .id("btn_next_exercise")
                            .px_6()
                            .py_3()
                            .rounded_lg()
                            .bg(rgb(0x0284c7))
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x0369a1)))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                this.state.next_exercise();
                                cx.notify();
                            }))
                            .child("Next Exercise ▶")
                    )
                    .child(
                        div()
                            .id("btn_repeat_exercise")
                            .px_6()
                            .py_3()
                            .rounded_lg()
                            .bg(rgb(0xf1f5f9))
                            .border_1()
                            .border_color(rgb(0xcbd5e1))
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x334155))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0xe2e8f0)))
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                this.state.restart_current_exercise();
                                cx.notify();
                            }))
                            .child("Repeat Drill ↺")
                    )
            )
            .into_any_element()
    }

    fn render_stat_box(&self, label: &'static str, value: &str, accent: Rgba) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .px_5()
            .py_3()
            .rounded_xl()
            .bg(rgb(0xf8fafc))
            .border_1()
            .border_color(rgb(0xe2e8f0))
            .child(
                div()
                    .text_size(px(20.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(accent)
                    .child(value.to_string())
            )
            .child(
                div()
                    .text_size(px(11.0))
                    .font_weight(FontWeight::SEMIBOLD)
                    .text_color(rgb(0x64748b))
                    .child(label)
            )
    }

    // 4. Smart Review View
    fn render_smart_review(&self, cx: &mut Context<Self>) -> AnyElement {
        let profile = self.state.profile_store.active_profile();
        let difficult_keys = profile.map(|p| p.get_difficult_keys()).unwrap_or_default();

        let review_text = if difficult_keys.is_empty() {
            "the quick brown fox jumps over the lazy dog repeatedly and smoothly".to_string()
        } else {
            let keys_str: String = difficult_keys.iter().map(|(c, _)| *c).collect();
            format!("{} {} {} practice strengthens difficult keys with patience", keys_str, keys_str, keys_str)
        };

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .p_6()
                    .rounded_xl()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe2e8f0))
                    .flex()
                    .flex_col()
                    .gap_2()
                    .child(
                        div()
                            .text_size(px(20.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x0f172a))
                            .child("🎯 Smart Weak Key Review")
                    )
                    .child(
                        div()
                            .text_size(px(13.0))
                            .text_color(rgb(0x64748b))
                            .child("TypeStudent automatically analyzes your keystrokes to detect missed letters and slow reactions. Practice targeted drills to eliminate weak spots!")
                    )
            )
            .child(
                div()
                    .p_6()
                    .rounded_xl()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe2e8f0))
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x0f172a))
                            .child("Current Difficult Keys:")
                    )
                    .child(
                        if difficult_keys.is_empty() {
                            div().text_size(px(13.0)).text_color(rgb(0x15803d)).child("✨ No weak keys detected! Your accuracy is great across all tested keys.")
                        } else {
                            div()
                                .flex()
                                .gap_3()
                                .children(
                                    difficult_keys.into_iter().map(|(c, acc)| {
                                        div()
                                            .flex()
                                            .flex_col()
                                            .items_center()
                                            .px_4()
                                            .py_2()
                                            .rounded_lg()
                                            .bg(rgb(0xfef2f2))
                                            .border_1()
                                            .border_color(rgb(0xfecaca))
                                            .child(
                                                div()
                                                    .text_size(px(18.0))
                                                    .font_weight(FontWeight::BOLD)
                                                    .text_color(rgb(0xdc2626))
                                                    .child(c.to_uppercase().to_string())
                                            )
                                            .child(
                                                div()
                                                    .text_size(px(11.0))
                                                    .text_color(rgb(0x64748b))
                                                    .child(format!("{:.0}% Acc", acc))
                                            )
                                    })
                                )
                        }
                    )
                    .child(
                        div()
                            .id("btn_start_smart_review")
                            .mt_4()
                            .px_6()
                            .py_3()
                            .w(px(220.0))
                            .rounded_lg()
                            .bg(rgb(0x0284c7))
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0xffffff))
                            .cursor_pointer()
                            .hover(|s| s.bg(rgb(0x0369a1)))
                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                this.state.start_smart_review(&review_text);
                                cx.notify();
                            }))
                            .child("Start Targeted Drill ▶")
                    )
            )
            .into_any_element()
    }

    // 5. Story Studio View (Kid-friendly reading and typing stories)
    fn render_story_studio(&self, cx: &mut Context<Self>) -> AnyElement {
        let stories = get_all_stories();

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .p_6()
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
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0f172a))
                                    .child("📖 Story Studio & Creative Typing")
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x64748b))
                                    .child("Practice typing full sentences by reading exciting stories about dragons, space exploration, and prehistoric adventures!")
                            )
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_4()
                    .children(
                        stories.into_iter().enumerate().map(|(idx, story)| {
                            let title = story.title.to_string();
                            let content = story.content.to_string();
                            let story_btn_id = format!("story_btn_{}", idx);

                            div()
                                .flex()
                                .flex_col()
                                .justify_between()
                                .w(px(320.0))
                                .p_5()
                                .rounded_xl()
                                .bg(rgb(0xffffff))
                                .border_1()
                                .border_color(rgb(0xe2e8f0))
                                .gap_3()
                                .hover(|s| s.bg(rgb(0xf8fafc)).border_color(rgb(0x38bdf8)))
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_2()
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_2p5()
                                                .child(div().text_size(px(28.0)).child(story.emoji))
                                                .child(
                                                    div()
                                                        .text_size(px(16.0))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(0x0f172a))
                                                        .child(story.title)
                                                )
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.0))
                                                .text_color(rgb(0x64748b))
                                                .line_height(px(18.0))
                                                .child(format!("{}...", &story.content[..story.content.len().min(85)]))
                                        )
                                )
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            div()
                                                .text_size(px(11.0))
                                                .text_color(rgb(0x0284c7))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .child(format!("{} Words | {}", story.word_count, story.difficulty))
                                        )
                                        .child(
                                            div()
                                                .id(ElementId::Name(story_btn_id.into()))
                                                .px_3p5()
                                                .py_1p5()
                                                .rounded_lg()
                                                .bg(rgb(0x0284c7))
                                                .text_size(px(12.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0xffffff))
                                                .cursor_pointer()
                                                .hover(|s| s.bg(rgb(0x0369a1)))
                                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                                    this.state.start_story_practice(&title, &content);
                                                    cx.notify();
                                                }))
                                                .child("Read & Type ▶")
                                        )
                                )
                        })
                    )
            )
            .into_any_element()
    }

    // 6. Trophy Room & Achievement Badges View
    fn render_trophies(&self, _cx: &mut Context<Self>) -> AnyElement {
        let trophies = get_all_trophies();
        let profile = self.state.profile_store.active_profile();
        let unlocked_count = trophies.iter().filter(|t| profile.map_or(false, |p| p.has_trophy(t.id))).count();

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p_6()
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
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0f172a))
                                    .child("🏆 Trophy Room & Achievement Badges")
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x64748b))
                                    .child("Unlock milestones as you improve speed, master lessons, and play typing games!")
                            )
                    )
                    .child(
                        div()
                            .px_4()
                            .py_2()
                            .rounded_lg()
                            .bg(rgb(0xfef3c7))
                            .border_1()
                            .border_color(rgb(0xfde68a))
                            .text_size(px(14.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x92400e))
                            .child(format!("Unlocked: {} / {} Badges", unlocked_count, trophies.len()))
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_4()
                    .children(
                        trophies.into_iter().map(|trophy| {
                            let is_unlocked = profile.map_or(false, |p| p.has_trophy(trophy.id));
                            let unlocked_date = profile.and_then(|p| p.unlocked_trophies.get(trophy.id)).cloned();

                            let bg_color = if is_unlocked { rgb(0xffffff) } else { rgb(0xf8fafc) };
                            let border_color = if is_unlocked { rgb(0xf59e0b) } else { rgb(0xe2e8f0) };

                            div()
                                .flex()
                                .items_center()
                                .w(px(320.0))
                                .p_4()
                                .rounded_xl()
                                .bg(bg_color)
                                .border_1()
                                .border_color(border_color)
                                .gap_3()
                                .child(
                                    div()
                                        .w(px(52.0))
                                        .h(px(52.0))
                                        .rounded_full()
                                        .bg(if is_unlocked { rgb(0xfef3c7) } else { rgb(0xe2e8f0) })
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_size(px(26.0))
                                        .child(if is_unlocked { trophy.icon } else { "🔒" })
                                )
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_1()
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(if is_unlocked { rgb(0x0f172a) } else { rgb(0x64748b) })
                                                .child(trophy.title)
                                        )
                                        .child(
                                            div()
                                                .text_size(px(11.0))
                                                .text_color(rgb(0x64748b))
                                                .line_height(px(15.0))
                                                .child(trophy.description)
                                        )
                                        .child(
                                            div()
                                                .text_size(px(10.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(if is_unlocked { rgb(0x15803d) } else { rgb(0x94a3b8) })
                                                .child(if is_unlocked {
                                                    format!("Unlocked ✓ {}", unlocked_date.as_deref().unwrap_or(""))
                                                } else {
                                                    "Locked 🔒".to_string()
                                                })
                                        )
                                )
                        })
                    )
            )
            .into_any_element()
    }

    // 7. Typing Tests View
    fn render_typing_tests(&self, cx: &mut Context<Self>) -> AnyElement {
        let tests = vec![
            ("Aesop's Fables: The Fox and Grapes", 120, "One warm summer day, a Fox was strolling through an orchard until he came to a bunch of Grapes just ripening on a vine..."),
            ("The History of the Airplane", 180, "For centuries, humans watched birds soar freely across the skies and dreamed of building wings to fly..."),
            ("Albert Einstein and Curiosity", 300, "Albert Einstein once stated that he had no special talents, but was only passionately curious about the universe..."),
            ("Speed Sprint (1 Minute)", 60, "Touch typing is an essential 21st century skill that unlocks high productivity, creativity, and rapid programming speed."),
        ];

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .p_6()
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
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0f172a))
                                    .child("⏱️ Official Timed Typing Tests")
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x64748b))
                                    .child("Take a standardized timed test. Completing a test generates an authentic Certificate Diploma!")
                            )
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_3()
                    .children(
                        tests.into_iter().enumerate().map(|(idx, (name, duration, text))| {
                            let minutes = duration / 60;
                            let text_copy = text.to_string();
                            let name_copy = name.to_string();
                            let test_btn_id = format!("test_btn_{}", idx);

                            div()
                                .flex()
                                .justify_between()
                                .items_center()
                                .p_4()
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
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0x0f172a))
                                                .child(name)
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.0))
                                                .text_color(rgb(0x64748b))
                                                .child(format!("Duration: {} min | Standardized Text", minutes))
                                        )
                                )
                                .child(
                                    div()
                                        .id(ElementId::Name(test_btn_id.into()))
                                        .px_4()
                                        .py_2()
                                        .rounded_lg()
                                        .bg(rgb(0x0284c7))
                                        .text_size(px(13.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xffffff))
                                        .cursor_pointer()
                                        .hover(|s| s.bg(rgb(0x0369a1)))
                                        .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                            this.state.start_typing_test(&name_copy, &text_copy, duration);
                                            cx.notify();
                                        }))
                                        .child("Begin Test ▶")
                                )
                        })
                    )
            )
            .into_any_element()
    }

    // 8. Diploma / Certificate View
    fn render_diploma(&self, cx: &mut Context<Self>) -> AnyElement {
        let diploma = match &self.state.active_diploma {
            Some(d) => d,
            None => return div().child("No diploma available").into_any_element(),
        };

        let student_name = diploma.student_name.clone();
        let test_title = diploma.test_title.clone();
        let net_wpm = diploma.net_wpm;
        let accuracy = diploma.accuracy;
        let date = diploma.date.clone();
        let duration = diploma.duration_label.clone();
        let export_msg = self.export_status.clone();

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_6()
            .p_8()
            .rounded_2xl()
            .bg(rgb(0xfefce8)) // Warm cream white
            .border_4()
            .border_color(rgb(0xd97706)) // Classic gold/amber border
            .text_color(rgb(0x78350f))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(div().text_size(px(40.0)).child("🏆"))
                    .child(
                        div()
                            .text_size(px(28.0))
                            .font_weight(FontWeight::BOLD)
                            .child("Certificate of Touch Typing Achievement")
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .child("This officially certifies that")
                    )
                    .child(
                        div()
                            .text_size(px(26.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x92400e))
                            .child(diploma.student_name.clone())
                    )
                    .child(
                        div()
                            .text_size(px(14.0))
                            .child(format!("has successfully completed the timed typing examination: {}", diploma.test_title))
                    )
            )
            .child(
                div()
                    .flex()
                    .gap_6()
                    .p_4()
                    .rounded_xl()
                    .bg(rgb(0xfef08a))
                    .border_1()
                    .border_color(rgb(0xfacc15))
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(div().text_size(px(22.0)).font_weight(FontWeight::BOLD).child(format!("{:.1} WPM", diploma.net_wpm)))
                            .child(div().text_size(px(11.0)).child("Net Speed"))
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(div().text_size(px(22.0)).font_weight(FontWeight::BOLD).child(format!("{:.1}%", diploma.accuracy)))
                            .child(div().text_size(px(11.0)).child("Accuracy"))
                    )
                    .child(
                        div()
                            .flex()
                            .flex_col()
                            .items_center()
                            .child(div().text_size(px(22.0)).font_weight(FontWeight::BOLD).child(diploma.duration_label.clone()))
                            .child(div().text_size(px(11.0)).child("Duration"))
                    )
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .w_full()
                    .px_10()
                    .mt_2()
                    .child(div().text_size(px(12.0)).child(format!("Date Issued: {}", diploma.date)))
                    .child(div().text_size(px(12.0)).child("TypeStudent Examination Board 📜"))
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(
                        div()
                            .flex()
                            .gap_4()
                            .child(
                                div()
                                    .id("btn_export_html_diploma")
                                    .px_6()
                                    .py_2p5()
                                    .rounded_lg()
                                    .bg(rgb(0x0284c7))
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0x0369a1)))
                                    .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                        if let Some(path) = export_diploma_html(&student_name, &test_title, net_wpm, accuracy, &date, &duration) {
                                            this.export_status = Some(format!("Saved: {}", path.display()));
                                        }
                                        cx.notify();
                                    }))
                                    .child("🖨️ Export & Print Certificate (HTML)")
                            )
                            .child(
                                div()
                                    .id("btn_back_from_diploma")
                                    .px_6()
                                    .py_2p5()
                                    .rounded_lg()
                                    .bg(rgb(0xd97706))
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                        this.state.active_view = ActiveView::TypingTests;
                                        cx.notify();
                                    }))
                                    .child("Back to Tests")
                            )
                    )
                    .children(export_msg.map(|msg| {
                        div().text_size(px(12.0)).font_weight(FontWeight::BOLD).text_color(rgb(0x15803d)).child(format!("✓ Certificate exported to {}", msg))
                    }))
            )
            .into_any_element()
    }

    // 9. Games Hub View
    fn render_games_hub(&self, cx: &mut Context<Self>) -> AnyElement {
        let cards = get_game_cards();

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .p_6()
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
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0f172a))
                                    .child("🎮 The 4 Classic TypingMaster Games")
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x64748b))
                                    .child("Reinforce typing muscle memory and reflexes through arcade mini games!")
                            )
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_4()
                    .children(
                        cards.into_iter().enumerate().map(|(idx, card)| {
                            let target_view = card.view;
                            let play_btn_id = format!("play_game_btn_{}", idx);

                            div()
                                .flex()
                                .flex_col()
                                .justify_between()
                                .w(px(320.0))
                                .p_5()
                                .rounded_xl()
                                .bg(rgb(0xffffff))
                                .border_1()
                                .border_color(rgb(0xe2e8f0))
                                .hover(|s| s.bg(rgb(0xf8fafc)).border_color(rgb(0x38bdf8)))
                                .gap_4()
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .gap_2()
                                        .child(
                                            div()
                                                .flex()
                                                .items_center()
                                                .gap_3()
                                                .child(div().text_size(px(32.0)).child(card.icon))
                                                .child(
                                                    div()
                                                        .text_size(px(18.0))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(0x0f172a))
                                                        .child(card.title)
                                                )
                                        )
                                        .child(
                                            div()
                                                .text_size(px(12.0))
                                                .text_color(rgb(0x64748b))
                                                .child(card.description)
                                        )
                                )
                                .child(
                                    div()
                                        .flex()
                                        .justify_between()
                                        .items_center()
                                        .child(
                                            div()
                                                .text_size(px(11.0))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(rgb(0xd97706))
                                                .child(card.difficulty)
                                        )
                                        .child(
                                            div()
                                                .id(ElementId::Name(play_btn_id.into()))
                                                .px_4()
                                                .py_1p5()
                                                .rounded_lg()
                                                .bg(rgb(0x0284c7))
                                                .text_size(px(12.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0xffffff))
                                                .cursor_pointer()
                                                .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                                    this.state.active_view = target_view;
                                                    cx.notify();
                                                }))
                                                .child("Play ▶")
                                        )
                                )
                        })
                    )
            )
            .into_any_element()
    }

    // Game 1: Bubbles
    fn render_game_bubbles(&self, cx: &mut Context<Self>) -> AnyElement {
        let game = &self.state.bubbles_game;

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p_4()
                    .rounded_xl()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe2e8f0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(div().text_size(px(24.0)).child("🫧"))
                            .child(div().text_size(px(18.0)).font_weight(FontWeight::BOLD).child("Bubbles"))
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(div().text_size(px(14.0)).font_weight(FontWeight::BOLD).text_color(rgb(0xd97706)).child(format!("Score: {}", game.score)))
                            .child(div().text_size(px(14.0)).font_weight(FontWeight::BOLD).text_color(rgb(0x0284c7)).child(format!("Streak: {}", game.streak)))
                            .child(div().text_size(px(14.0)).text_color(rgb(0xdc2626)).child(format!("Lives: {}", "❤️".repeat(game.lives as usize))))
                    )
            )
            .child(
                div()
                    .h(px(420.0))
                    .rounded_2xl()
                    .bg(rgb(0xe0f2fe)) // Soothing soft water blue
                    .border_2()
                    .border_color(rgb(0x38bdf8))
                    .relative()
                    .overflow_hidden()
                    .children(
                        game.bubbles.iter().map(|b| {
                            let left_px = b.x * 650.0;
                            let top_px = b.y * 380.0;

                            div()
                                .absolute()
                                .left(px(left_px))
                                .top(px(top_px))
                                .px_3()
                                .py_1p5()
                                .rounded_full()
                                .bg(rgb(0x0284c7))
                                .border_2()
                                .border_color(rgb(0xffffff))
                                .text_size(px(14.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .child(b.text.clone())
                        })
                    )
            )
            .child(
                div()
                    .flex()
                    .justify_between()
                    .child(
                        div().text_size(px(12.0)).text_color(rgb(0x64748b)).child("Type the letter or word in the bubble to pop it before it reaches the floor!")
                    )
                    .child(
                        div()
                            .id("btn_restart_bubbles")
                            .px_4()
                            .py_1p5()
                            .rounded_md()
                            .bg(rgb(0xf1f5f9))
                            .border_1()
                            .border_color(rgb(0xcbd5e1))
                            .text_size(px(12.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x334155))
                            .cursor_pointer()
                            .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                this.state.bubbles_game.restart();
                                cx.notify();
                            }))
                            .child("Restart Game ↺")
                    )
            )
            .into_any_element()
    }

    // Game 2: WordTris
    fn render_game_wordtris(&self, _cx: &mut Context<Self>) -> AnyElement {
        let game = &self.state.wordtris_game;

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p_4()
                    .rounded_xl()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe2e8f0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(div().text_size(px(24.0)).child("🧱"))
                            .child(div().text_size(px(18.0)).font_weight(FontWeight::BOLD).child("WordTris"))
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(div().text_size(px(14.0)).font_weight(FontWeight::BOLD).text_color(rgb(0xd97706)).child(format!("Score: {}", game.score)))
                            .child(div().text_size(px(14.0)).font_weight(FontWeight::BOLD).text_color(rgb(0x0284c7)).child(format!("Cleared: {}", game.lines_cleared)))
                    )
            )
            .child(
                div()
                    .h(px(380.0))
                    .rounded_2xl()
                    .bg(rgb(0xf8fafc))
                    .border_2()
                    .border_color(rgb(0xe2e8f0))
                    .flex()
                    .justify_around()
                    .p_4()
                    .children(
                        (0..game.columns).map(|col_idx| {
                            let height = game.stack_heights[col_idx];
                            let danger = height >= 4;

                            div()
                                .flex()
                                .flex_col_reverse()
                                .w(px(110.0))
                                .h_full()
                                .rounded_lg()
                                .bg(rgb(0xffffff))
                                .border_1()
                                .border_color(if danger { rgb(0xf87171) } else { rgb(0xcbd5e1) })
                                .p_2()
                                .gap_2()
                                .children(
                                    (0..height).map(|_| {
                                        div()
                                            .w_full()
                                            .h(px(28.0))
                                            .rounded_md()
                                            .bg(if danger { rgb(0xf87171) } else { rgb(0x0284c7) })
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xffffff))
                                            .child("🧱 BEAM")
                                    })
                                )
                        })
                    )
            )
            .into_any_element()
    }

    // Game 3: Clouds
    fn render_game_clouds(&self, _cx: &mut Context<Self>) -> AnyElement {
        let game = &self.state.clouds_game;

        div()
            .flex()
            .flex_col()
            .gap_4()
            .child(
                div()
                    .flex()
                    .justify_between()
                    .items_center()
                    .p_4()
                    .rounded_xl()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe2e8f0))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_3()
                            .child(div().text_size(px(24.0)).child("☁️"))
                            .child(div().text_size(px(18.0)).font_weight(FontWeight::BOLD).child("Clouds"))
                    )
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_4()
                            .child(div().text_size(px(14.0)).font_weight(FontWeight::BOLD).text_color(rgb(0xd97706)).child(format!("Score: {}", game.score)))
                            .child(div().text_size(px(14.0)).font_weight(FontWeight::BOLD).text_color(rgb(0xdc2626)).child(format!("Misses: {}/5", game.misses)))
                    )
            )
            .child(
                div()
                    .h(px(380.0))
                    .rounded_2xl()
                    .bg(rgb(0xbae6fd)) // Soft Pastel Sky
                    .border_2()
                    .border_color(rgb(0x7dd3fc))
                    .relative()
                    .overflow_hidden()
                    .children(
                        game.clouds.iter().map(|c| {
                            let left_px = c.x * 650.0;
                            let top_px = c.y * 300.0;

                            div()
                                .absolute()
                                .left(px(left_px))
                                .top(px(top_px))
                                .px_4()
                                .py_2()
                                .rounded_full()
                                .bg(if c.is_sunny { rgb(0xfef08a) } else { rgb(0xffffff) })
                                .text_color(rgb(0x0f172a))
                                .text_size(px(14.0))
                                .font_weight(FontWeight::BOLD)
                                .child(format!("☁️ {}", c.text))
                        })
                    )
            )
            .into_any_element()
    }

    // Game 4: ABC Sprint
    fn render_game_abc(&self, cx: &mut Context<Self>) -> AnyElement {
        let game = &self.state.abc_game;
        let current_target = game.current_char().map(|c| c.to_uppercase().to_string()).unwrap_or_else(|| "Done".to_string());
        let elapsed = game.elapsed_secs();

        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_6()
            .p_8()
            .rounded_2xl()
            .bg(rgb(0xffffff))
            .border_1()
            .border_color(rgb(0xe2e8f0))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .gap_2()
                    .child(div().text_size(px(40.0)).child("⚡"))
                    .child(div().text_size(px(24.0)).font_weight(FontWeight::BOLD).child("ABC Alphabet Sprint"))
                    .child(div().text_size(px(13.0)).text_color(rgb(0x64748b)).child("Type A to Z as fast as humanly possible!"))
            )
            .child(
                div()
                    .flex()
                    .flex_col()
                    .items_center()
                    .px_10()
                    .py_6()
                    .rounded_2xl()
                    .bg(rgb(0xf8fafc))
                    .border_2()
                    .border_color(rgb(0x38bdf8))
                    .gap_2()
                    .child(div().text_size(px(14.0)).text_color(rgb(0x64748b)).child("STRIKE NEXT LETTER"))
                    .child(
                        div()
                            .text_size(px(64.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x0284c7))
                            .child(current_target)
                    )
            )
            .child(
                div()
                    .flex()
                    .gap_6()
                    .child(self.render_stat_box("TIME", &format!("{:.2}s", elapsed), rgb(0xd97706)))
                    .child(self.render_stat_box("MISTAKES", &format!("{}", game.mistakes), rgb(0xdc2626)))
                    .child(self.render_stat_box("PROGRESS", &format!("{}/26", game.current_idx), rgb(0x15803d)))
            )
            .child(
                div()
                    .id("btn_restart_abc")
                    .px_6()
                    .py_2p5()
                    .rounded_lg()
                    .bg(rgb(0x0284c7))
                    .text_size(px(13.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0xffffff))
                    .cursor_pointer()
                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                        this.state.abc_game.restart();
                        cx.notify();
                    }))
                    .child("Restart Sprint ↺")
            )
            .into_any_element()
    }

    // 10. Statistics & Heatmap View
    fn render_statistics(&self, _cx: &mut Context<Self>) -> AnyElement {
        let profile = self.state.profile_store.active_profile();

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .p_6()
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
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0f172a))
                                    .child("📊 Touch Typing Statistics & Heatmap")
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x64748b))
                                    .child("Per-key accuracy analysis and cumulative typing metrics.")
                            )
                    )
            )
            .child(
                div()
                    .p_6()
                    .rounded_xl()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe2e8f0))
                    .flex()
                    .flex_col()
                    .gap_4()
                    .child(
                        div()
                            .text_size(px(16.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x0f172a))
                            .child("Alphabet Accuracy Heatmap:")
                    )
                    .child(
                        div()
                            .flex()
                            .flex_wrap()
                            .gap_2()
                            .children(
                                ('a'..='z').map(|c| {
                                    let stat = profile.and_then(|p| p.key_stats.get(&c));
                                    let acc = stat.map(|s| s.accuracy()).unwrap_or(100.0);
                                    let total = stat.map(|s| s.hits + s.misses).unwrap_or(0);

                                    let (bg_color, border_color, text_color) = if total == 0 {
                                        (rgb(0xf8fafc), rgb(0xe2e8f0), rgb(0x94a3b8)) // Unused
                                    } else if acc >= 95.0 {
                                        (rgb(0xf0fdf4), rgb(0x86efac), rgb(0x15803d)) // Green
                                    } else if acc >= 85.0 {
                                        (rgb(0xfefce8), rgb(0xfde047), rgb(0xa16207)) // Yellow/Amber
                                    } else {
                                        (rgb(0xfef2f2), rgb(0xfca5a5), rgb(0xdc2626)) // Red
                                    };

                                    div()
                                        .flex()
                                        .flex_col()
                                        .items_center()
                                        .w(px(52.0))
                                        .py_2()
                                        .rounded_lg()
                                        .bg(bg_color)
                                        .border_1()
                                        .border_color(border_color)
                                        .child(
                                            div()
                                                .text_size(px(16.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(text_color)
                                                .child(c.to_uppercase().to_string())
                                        )
                                        .child(
                                            div()
                                                .text_size(px(10.0))
                                                .text_color(text_color)
                                                .child(if total == 0 { "-".to_string() } else { format!("{:.0}%", acc) })
                                        )
                                })
                            )
                    )
            )
            .into_any_element()
    }

    // 11. Profiles View
    fn render_profiles(&mut self, cx: &mut Context<Self>) -> AnyElement {
        let profiles = self.state.profile_store.profiles.clone();
        let active_id = self.state.profile_store.active_profile_id.clone();

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .p_6()
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
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0f172a))
                                    .child("👥 Student Profiles")
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x64748b))
                                    .child("Multiple student accounts for siblings, nieces, and nephews to practice independently.")
                            )
                    )
            )
            .child(
                div()
                    .flex()
                    .flex_wrap()
                    .gap_4()
                    .children(
                        profiles.into_iter().enumerate().map(|(idx, p)| {
                            let is_active = p.id == active_id;
                            let p_id = p.id.clone();
                            let prof_btn_id = format!("prof_btn_{}", idx);

                            div()
                                .flex()
                                .justify_between()
                                .items_center()
                                .w(px(320.0))
                                .p_4()
                                .rounded_xl()
                                .bg(rgb(0xffffff))
                                .border_1()
                                .border_color(if is_active { rgb(0x0284c7) } else { rgb(0xe2e8f0) })
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_3()
                                        .child(div().text_size(px(28.0)).child(p.avatar_emoji.clone()))
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .text_size(px(15.0))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(0x0f172a))
                                                        .child(p.name.clone())
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(11.0))
                                                        .font_weight(FontWeight::SEMIBOLD)
                                                        .text_color(rgb(0xd97706))
                                                        .child(format!("⭐ {} Stars | {} Lessons", p.total_stars(), p.total_lessons_passed()))
                                                )
                                        )
                                )
                                .child(
                                    if is_active {
                                        div()
                                            .px_3()
                                            .py_1()
                                            .rounded_md()
                                            .bg(rgb(0x0284c7))
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0xffffff))
                                            .child("Active ✓")
                                            .into_any_element()
                                    } else {
                                        div()
                                            .id(ElementId::Name(prof_btn_id.into()))
                                            .px_3()
                                            .py_1()
                                            .rounded_md()
                                            .bg(rgb(0xf1f5f9))
                                            .border_1()
                                            .border_color(rgb(0xcbd5e1))
                                            .text_size(px(11.0))
                                            .font_weight(FontWeight::BOLD)
                                            .text_color(rgb(0x334155))
                                            .cursor_pointer()
                                            .on_mouse_down(MouseButton::Left, cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                                this.state.profile_store.switch_profile(&p_id);
                                                cx.notify();
                                            }))
                                            .child("Select")
                                            .into_any_element()
                                    }
                                )
                        })
                    )
            )
            .into_any_element()
    }

    // 12. Settings View
    fn render_settings(&self, cx: &mut Context<Self>) -> AnyElement {
        let profile = self.state.profile_store.active_profile();
        let sound_enabled = profile.map(|p| p.settings.sound_enabled).unwrap_or(true);
        let target_wpm = profile.map(|p| p.settings.target_wpm).unwrap_or(35);

        div()
            .flex()
            .flex_col()
            .gap_6()
            .child(
                div()
                    .p_6()
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
                                    .text_size(px(20.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0f172a))
                                    .child("⚙️ Settings & Options")
                            )
                            .child(
                                div()
                                    .text_size(px(13.0))
                                    .text_color(rgb(0x64748b))
                                    .child("Configure sound effects, speed goals, and accessibility.")
                            )
                    )
            )
            .child(
                div()
                    .p_6()
                    .rounded_xl()
                    .bg(rgb(0xffffff))
                    .border_1()
                    .border_color(rgb(0xe2e8f0))
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
                                    .flex_col()
                                    .child(div().text_size(px(14.0)).font_weight(FontWeight::BOLD).child("Procedural Sound Effects"))
                                    .child(div().text_size(px(12.0)).text_color(rgb(0x64748b)).child("Mechanical keyclicks, chimes, and mistake thuds"))
                            )
                            .child(
                                div()
                                    .id("toggle_sound_btn")
                                    .px_4()
                                    .py_1p5()
                                    .rounded_lg()
                                    .bg(if sound_enabled { rgb(0x0284c7) } else { rgb(0x94a3b8) })
                                    .text_size(px(12.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0xffffff))
                                    .cursor_pointer()
                                    .on_mouse_down(MouseButton::Left, cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                        if let Some(p) = this.state.profile_store.active_profile_mut() {
                                            p.settings.sound_enabled = !p.settings.sound_enabled;
                                            let en = p.settings.sound_enabled;
                                            this.state.audio.set_enabled(en);
                                        }
                                        this.state.profile_store.save();
                                        cx.notify();
                                    }))
                                    .child(if sound_enabled { "Enabled 🔊" } else { "Muted 🔇" })
                            )
                    )
                    .child(
                        div()
                            .flex()
                            .justify_between()
                            .items_center()
                            .child(
                                div()
                                    .flex()
                                    .flex_col()
                                    .child(div().text_size(px(14.0)).font_weight(FontWeight::BOLD).child("Target Speed Goal"))
                                    .child(div().text_size(px(12.0)).text_color(rgb(0x64748b)).child("Benchmark speed used for 5-star calculations"))
                            )
                            .child(
                                div()
                                    .px_4()
                                    .py_1p5()
                                    .rounded_lg()
                                    .bg(rgb(0xf8fafc))
                                    .border_1()
                                    .border_color(rgb(0xbae6fd))
                                    .text_size(px(13.0))
                                    .font_weight(FontWeight::BOLD)
                                    .text_color(rgb(0x0369a1))
                                    .child(format!("{} WPM", target_wpm))
                            )
                    )
            )
            .into_any_element()
    }
}
