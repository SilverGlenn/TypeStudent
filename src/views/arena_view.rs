use crate::app::TypeStudentView;
use crate::components::hands::{HandFingerState, HandsGuideModel};
use crate::components::keyboard::{get_keyboard_layout, Finger};
use crate::engine::CharStatus;
use crate::state::ActiveView;
use gpui::*;

pub fn render_typing_arena(view: &TypeStudentView, cx: &mut Context<TypeStudentView>) -> AnyElement {
    let session = match &view.state.typing_session {
        Some(s) => s,
        None => return div().child("No active session").into_any_element(),
    };
    let info = view.state.current_exercise_info.as_ref();
    let title = info.map(|i| i.title.clone()).unwrap_or_else(|| "Exercise".to_string());
    let instruction = info.map(|i| i.instruction.clone()).unwrap_or_default();

    // 1. Pre-Activity Warm-Up & Countdown Screen
    if view.state.is_pre_activity {
        return render_pre_activity_screen(view, &title, &instruction, cx);
    }

    // 2. Live Typing Screen (Minimal HUD, No cluttered banners, spacious)
    let net_wpm = session.net_wpm();
    let accuracy = session.accuracy_percent();
    let errors = session.error_keystrokes;
    let active_char = session.current_char();
    let active_finger = view.state.active_finger();
    let hands_model = HandsGuideModel::for_active_target(active_finger, active_char);

    div()
        .flex()
        .flex_col()
        .gap_4()
        .size_full()
        // Top Minimal Live HUD & Quick Exit
        .child(
            div()
                .flex()
                .justify_between()
                .items_center()
                .px_4()
                .py_2p5()
                .rounded_xl()
                .bg(rgb(0xffffff))
                .border_1()
                .border_color(rgb(0xe2e8f0))
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .child(
                            div()
                                .text_size(px(15.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x0f172a))
                                .child(title),
                        ),
                )
                // 3 Live Indicators only: WPM, Accuracy, Errors
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .child(metric_pill("WPM", format!("{:.0}", net_wpm), rgb(0x15803d), rgb(0xf0fdf4), rgb(0xbbf7d0)))
                        .child(metric_pill("ACCURACY", format!("{:.0}%", accuracy), rgb(0x0369a1), rgb(0xf0f9ff), rgb(0xbae6fd)))
                        .child(metric_pill(
                            "ERRORS",
                            format!("{}", errors),
                            if errors > 0 { rgb(0xdc2626) } else { rgb(0x64748b) },
                            if errors > 0 { rgb(0xfef2f2) } else { rgb(0xf8fafc) },
                            if errors > 0 { rgb(0xfecaca) } else { rgb(0xe2e8f0) },
                        ))
                        // Voice Guidance Toggle button
                        .child({
                            let voice_on = view.state.audio.is_voice_enabled();
                            div()
                                .id("btn_toggle_voice")
                                .px_2p5()
                                .py_1p5()
                                .rounded_lg()
                                .bg(if voice_on { rgb(0xf0fdf4) } else { rgb(0xf8fafc) })
                                .border_1()
                                .border_color(if voice_on { rgb(0x86efac) } else { rgb(0xcbd5e1) })
                                .text_size(px(11.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(if voice_on { rgb(0x15803d) } else { rgb(0x64748b) })
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0xe2e8f0)))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                        this.state.audio.toggle_voice();
                                        cx.notify();
                                    }),
                                )
                                .child(if voice_on { "🔊 Voice ON" } else { "🔈 Voice OFF" })
                        })
                        // Quick Exit button
                        .child(
                            div()
                                .id("btn_exit_arena")
                                .px_3()
                                .py_1p5()
                                .rounded_lg()
                                .bg(rgb(0xf1f5f9))
                                .border_1()
                                .border_color(rgb(0xcbd5e1))
                                .text_size(px(11.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x475569))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0xe2e8f0)))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                        this.state.active_view = ActiveView::CourseOverview;
                                        this.state.is_sidebar_open = true;
                                        cx.notify();
                                    }),
                                )
                                .child("✕ Exit"),
                        ),
                ),
        )
        // Spacious, large-font typing text box
        .child(
            div()
                .p_5()
                .rounded_2xl()
                .bg(rgb(0xffffff))
                .border_2()
                .border_color(rgb(0x38bdf8))
                .flex()
                .flex_wrap()
                .gap_1p5()
                .text_size(px(24.0))
                .font_weight(FontWeight::MEDIUM)
                .line_height(px(36.0))
                .children(session.target_chars.iter().enumerate().map(|(idx, &ch)| {
                    let status = session.char_statuses[idx];
                    let is_cursor = idx == session.cursor_idx;
                    let is_space = ch == ' ';
                    let display_char = if is_space { "␣" } else { &ch.to_string() };

                    let (text_color, bg_color) = match status {
                        CharStatus::Correct => {
                            if is_space {
                                (rgb(0x86efac), rgb(0xf0fdf4))
                            } else {
                                (rgb(0x15803d), rgb(0xdcfce7))
                            }
                        }
                        CharStatus::Incorrect(_) => (rgb(0xb91c1c), rgb(0xfee2e2)),
                        CharStatus::Pending => {
                            if is_cursor {
                                (rgb(0xffffff), rgb(0x0284c7))
                            } else if is_space {
                                (rgb(0xcbd5e1), rgb(0xffffff)) // Faint, subtle space marker
                            } else {
                                (rgb(0x334155), rgb(0xffffff))
                            }
                        }
                    };

                    div()
                        .px_1p5()
                        .py_0p5()
                        .rounded_md()
                        .bg(bg_color)
                        .text_color(text_color)
                        .border_b_2()
                        .border_color(if is_cursor { rgb(0x0369a1) } else { rgb(0xffffff) })
                        .child(display_char.to_string())
                })),
        )
        // Friendly Keyboard & Hands Console (Ages 3-16)
        .child(
            div()
                .p_4()
                .rounded_2xl()
                .bg(rgb(0xffffff))
                .border_1()
                .border_color(rgb(0xe2e8f0))
                .flex()
                .flex_col()
                .items_center()
                .gap_3p5()
                // Visual Keyboard
                .child(render_friendly_keyboard(active_char, active_finger))
                // Visual Hands
                .child(render_friendly_hands(&hands_model)),
        )
        .into_any_element()
}

fn render_pre_activity_screen(
    view: &TypeStudentView,
    title: &str,
    instruction: &str,
    cx: &mut Context<TypeStudentView>,
) -> AnyElement {
    let title_copy = title.to_string();
    let instruction_copy = instruction.to_string();

    // 1. If 3-Second Countdown is active
    if let Some(countdown) = view.state.pre_activity_countdown {
        let (num_display, msg) = match countdown {
            3 => ("3", "Hands on Home Row!"),
            2 => ("2", "Get Ready!"),
            1 => ("1", "Start Typing!"),
            _ => ("Go!", "Go!"),
        };

        return div()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap_6()
            .p_12()
            .size_full()
            .rounded_2xl()
            .bg(rgb(0xffffff))
            .border_1()
            .border_color(rgb(0xe2e8f0))
            .child(
                div()
                    .w(px(140.0))
                    .h(px(140.0))
                    .rounded_full()
                    .bg(rgb(0xe0f2fe))
                    .border_4()
                    .border_color(rgb(0x0284c7))
                    .flex()
                    .items_center()
                    .justify_center()
                    .child(
                        div()
                            .text_size(px(64.0))
                            .font_weight(FontWeight::BOLD)
                            .text_color(rgb(0x0284c7))
                            .child(num_display),
                    ),
            )
            .child(
                div()
                    .text_size(px(24.0))
                    .font_weight(FontWeight::BOLD)
                    .text_color(rgb(0x0f172a))
                    .child(msg),
            )
            .into_any_element();
    }

    // 2. Interactive Key Warm-Up / Test Area
    let keys = &view.state.pre_activity_keys;
    let tested = &view.state.pre_activity_tested;
    let all_tested = !tested.is_empty() && tested.iter().all(|&t| t);

    div()
        .flex()
        .flex_col()
        .items_center()
        .justify_center()
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
                .child(div().text_size(px(40.0)).child("🖐️"))
                .child(
                    div()
                        .text_size(px(22.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x0f172a))
                        .child(title_copy),
                )
                .child(
                    div()
                        .text_size(px(14.0))
                        .text_color(rgb(0x475569))
                        .child(if instruction_copy.is_empty() {
                            "Test each key on your keyboard to warm up before typing:".to_string()
                        } else {
                            instruction_copy
                        }),
                ),
        )
        // Interactive Key Test Cards
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_3()
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x64748b))
                        .child("STRIKE EACH KEY ON YOUR KEYBOARD TO TEST:"),
                )
                .child(
                    div()
                        .flex()
                        .gap_3()
                        .children(keys.iter().enumerate().map(|(idx, &c)| {
                            let is_done = tested.get(idx).copied().unwrap_or(false);

                            let bg_color = if is_done { rgb(0xdcfce7) } else { rgb(0xf0f9ff) };
                            let border_color = if is_done { rgb(0x86efac) } else { rgb(0x38bdf8) };
                            let text_color = if is_done { rgb(0x15803d) } else { rgb(0x0284c7) };

                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .px_5()
                                .py_3()
                                .rounded_2xl()
                                .bg(bg_color)
                                .border_2()
                                .border_color(border_color)
                                .gap_1()
                                .child(
                                    div()
                                        .text_size(px(24.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(text_color)
                                        .child(c.to_uppercase().to_string()),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(text_color)
                                        .child(if is_done { "✓ Tested" } else { "Press Key" }),
                                )
                        })),
                ),
        )
        // Action Buttons
        .child(
            div()
                .flex()
                .items_center()
                .gap_4()
                .mt_2()
                .child(
                    div()
                        .id("btn_start_typing_live")
                        .px_8()
                        .py_3p5()
                        .rounded_xl()
                        .bg(if all_tested { rgb(0x16a34a) } else { rgb(0x0284c7) })
                        .text_size(px(15.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(0x0369a1)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                this.start_pre_activity_countdown(cx);
                            }),
                        )
                        .child(if all_tested {
                            "All Keys Tested! Start (Press Space) ▶"
                        } else {
                            "Start Activity (or Press Space) ▶"
                        }),
                )
                .child(
                    div()
                        .id("btn_cancel_pre_activity")
                        .px_5()
                        .py_3p5()
                        .rounded_xl()
                        .bg(rgb(0xf1f5f9))
                        .border_1()
                        .border_color(rgb(0xcbd5e1))
                        .text_size(px(14.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x475569))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(0xe2e8f0)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                this.state.active_view = ActiveView::CourseOverview;
                                this.state.is_sidebar_open = true;
                                cx.notify();
                            }),
                        )
                        .child("Back to Lessons"),
                ),
        )
        .into_any_element()
}

fn metric_pill(label: &'static str, value: String, text_color: Rgba, bg: Rgba, border: Rgba) -> Div {
    div()
        .flex()
        .flex_col()
        .items_center()
        .px_4()
        .py_1()
        .rounded_lg()
        .bg(bg)
        .border_1()
        .border_color(border)
        .child(
            div()
                .text_size(px(16.0))
                .font_weight(FontWeight::BOLD)
                .text_color(text_color)
                .child(value),
        )
        .child(
            div()
                .text_size(px(9.0))
                .font_weight(FontWeight::BOLD)
                .text_color(text_color)
                .child(label),
        )
}

fn render_friendly_keyboard(active_char: Option<char>, _active_finger: Option<Finger>) -> impl IntoElement {
    let layout = get_keyboard_layout();
    let target_lower = active_char.map(|c| c.to_ascii_lowercase());

    div()
        .flex()
        .flex_col()
        .gap_1p5()
        .children(layout.into_iter().map(|row| {
            div()
                .flex()
                .gap_1p5()
                .justify_center()
                .children(row.into_iter().map(|key| {
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

                    let width_px = key.width_units * 42.0;
                    let is_home_bump = key.label == "F" || key.label == "J";

                    div()
                        .w(px(width_px))
                        .h(px(38.0))
                        .rounded_lg()
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
                                .text_size(px(13.0))
                                .font_weight(if is_active {
                                    FontWeight::BOLD
                                } else {
                                    FontWeight::SEMIBOLD
                                })
                                .text_color(text_color)
                                .child(key.label),
                        )
                        .children(if is_home_bump && !is_active {
                            Some(
                                div()
                                    .w(px(5.0))
                                    .h(px(2.0))
                                    .rounded_full()
                                    .bg(rgb(0x64748b)),
                            )
                        } else {
                            None
                        })
                }))
        }))
}

fn render_friendly_hands(hands: &HandsGuideModel) -> impl IntoElement {
    let left_active_label = if hands.left_pinky.is_active { "Pinky" }
    else if hands.left_ring.is_active { "Ring" }
    else if hands.left_middle.is_active { "Middle" }
    else if hands.left_index.is_active { "Index" }
    else if hands.left_thumb.is_active { "Thumb" }
    else { "Left Hand" };

    let right_active_label = if hands.right_pinky.is_active { "Pinky" }
    else if hands.right_ring.is_active { "Ring" }
    else if hands.right_middle.is_active { "Middle" }
    else if hands.right_index.is_active { "Index" }
    else if hands.right_thumb.is_active { "Thumb" }
    else { "Right Hand" };

    let is_left_active = hands.left_pinky.is_active || hands.left_ring.is_active || hands.left_middle.is_active || hands.left_index.is_active || hands.left_thumb.is_active;
    let is_right_active = hands.right_pinky.is_active || hands.right_ring.is_active || hands.right_middle.is_active || hands.right_index.is_active || hands.right_thumb.is_active;

    div()
        .flex()
        .justify_around()
        .items_end()
        .w_full()
        .px_8()
        .child(
            // Left Hand
            div()
                .flex()
                .items_end()
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .child(
                            div()
                                .flex()
                                .items_end()
                                .gap_1()
                                .child(render_friendly_finger(hands.left_pinky, hands.target_char, false))
                                .child(render_friendly_finger(hands.left_ring, hands.target_char, false))
                                .child(render_friendly_finger(hands.left_middle, hands.target_char, false))
                                .child(render_friendly_finger(hands.left_index, hands.target_char, false)),
                        )
                        .child(
                            div()
                                .w(px(160.0))
                                .h(px(110.0))
                                .bg(rgb(0xffffff))
                                .border_2()
                                .border_color(if is_left_active { rgb(0x94a3b8) } else { rgb(0xcbd5e1) })
                                .rounded_b_3xl()
                                .rounded_t_sm()
                                .mt_1()
                                .flex()
                                .items_center()
                                .justify_center()
                                .shadow_sm()
                                .child(
                                    div()
                                        .text_size(px(15.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if is_left_active { rgb(0x0f172a) } else { rgb(0x94a3b8) })
                                        .child(left_active_label),
                                ),
                        ),
                )
                .child(
                    // Left Thumb
                    div()
                        .mb_6()
                        .ml_1()
                        .child(render_friendly_finger(hands.left_thumb, hands.target_char, true)),
                ),
        )
        .child(
            // Right Hand
            div()
                .flex()
                .items_end()
                .child(
                    // Right Thumb
                    div()
                        .mb_6()
                        .mr_1()
                        .child(render_friendly_finger(hands.right_thumb, hands.target_char, true)),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .child(
                            div()
                                .flex()
                                .items_end()
                                .gap_1()
                                .child(render_friendly_finger(hands.right_index, hands.target_char, false))
                                .child(render_friendly_finger(hands.right_middle, hands.target_char, false))
                                .child(render_friendly_finger(hands.right_ring, hands.target_char, false))
                                .child(render_friendly_finger(hands.right_pinky, hands.target_char, false)),
                        )
                        .child(
                            div()
                                .w(px(160.0))
                                .h(px(110.0))
                                .bg(rgb(0xffffff))
                                .border_2()
                                .border_color(if is_right_active { rgb(0x94a3b8) } else { rgb(0xcbd5e1) })
                                .rounded_b_3xl()
                                .rounded_t_sm()
                                .mt_1()
                                .flex()
                                .items_center()
                                .justify_center()
                                .shadow_sm()
                                .child(
                                    div()
                                        .text_size(px(15.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if is_right_active { rgb(0x0f172a) } else { rgb(0x94a3b8) })
                                        .child(right_active_label),
                                ),
                        ),
                ),
        )
}

fn render_friendly_finger(finger: HandFingerState, target_char: Option<char>, is_thumb: bool) -> impl IntoElement {
    let height = if finger.is_active { finger.active_height } else { finger.normal_height };
    let width = if is_thumb { finger.width + 8.0 } else { finger.width + 2.0 };

    let (f_r, f_g, f_b) = finger.finger.rgb();
    let finger_color = rgb((f_r as u32) << 16 | (f_g as u32) << 8 | (f_b as u32));

    let border_color = if finger.is_active { rgb(0x0f172a) } else { rgb(0xcbd5e1) };

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
        .bg(finger_color)
        .border_2()
        .border_color(border_color)
        .flex()
        .flex_col()
        .items_center()
        .justify_start()
        .pt_2()
        // Resting / Target Keycap on Fingertip
        .child(
            div()
                .w(px(24.0))
                .h(px(24.0))
                .rounded_full()
                .bg(rgb(0xffffff))
                .border_1()
                .border_color(if finger.is_active { rgb(0x0f172a) } else { rgb(0x94a3b8) })
                .flex()
                .items_center()
                .justify_center()
                .shadow_sm()
                .child(
                    div()
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(if finger.is_active { rgb(0x0f172a) } else { rgb(0x64748b) })
                        .child(tip_label),
                ),
        )
}
