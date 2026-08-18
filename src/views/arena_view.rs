use crate::app::TypeStudentView;
use crate::components::hands::{HandFingerState, HandsGuideModel};
use crate::components::keyboard::{get_keyboard_layout, Finger};
use crate::engine::CharStatus;
use gpui::*;

pub fn render_typing_arena(view: &TypeStudentView, _cx: &mut Context<TypeStudentView>) -> AnyElement {
    let session = match &view.state.typing_session {
        Some(s) => s,
        None => return div().child("No active session").into_any_element(),
    };
    let info = view.state.current_exercise_info.as_ref();
    let title = info.map(|i| i.title.clone()).unwrap_or_else(|| "Exercise".to_string());
    let instruction = info.map(|i| i.instruction.clone()).unwrap_or_default();

    let net_wpm = session.net_wpm();
    let accuracy = session.accuracy_percent();
    let errors = session.error_keystrokes;
    let progress = session.progress_ratio() * 100.0;
    let active_char = session.current_char();
    let active_finger = view.state.active_finger();
    let hands_model = HandsGuideModel::for_active_target(active_finger, active_char);

    div()
        .flex()
        .flex_col()
        .gap_3p5()
        .child(
            // Header & HUD Metrics
            div()
                .flex()
                .justify_between()
                .items_center()
                .px_4()
                .py_3()
                .rounded_xl()
                .bg(rgb(0xffffff))
                .border_1()
                .border_color(rgb(0xe2e8f0))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_0p5()
                        .child(
                            div()
                                .text_size(px(16.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x0f172a))
                                .child(title),
                        )
                        .child(
                            div()
                                .text_size(px(11.0))
                                .text_color(rgb(0x64748b))
                                .child(instruction),
                        ),
                )
                .child(
                    div()
                        .flex()
                        .items_center()
                        .gap_3()
                        .child(metric_pill("NET WPM", format!("{:.0}", net_wpm), rgb(0x15803d), rgb(0xf0fdf4), rgb(0xbbf7d0)))
                        .child(metric_pill("ACCURACY", format!("{:.0}%", accuracy), rgb(0x0369a1), rgb(0xf0f9ff), rgb(0xbae6fd)))
                        .child(metric_pill(
                            "ERRORS",
                            format!("{}", errors),
                            if errors > 0 { rgb(0xdc2626) } else { rgb(0x64748b) },
                            if errors > 0 { rgb(0xfef2f2) } else { rgb(0xf8fafc) },
                            if errors > 0 { rgb(0xfecaca) } else { rgb(0xe2e8f0) },
                        )),
                ),
        )
        // Progress Bar
        .child(
            div()
                .w_full()
                .h(px(4.0))
                .rounded_full()
                .bg(rgb(0xe2e8f0))
                .child(
                    div()
                        .h_full()
                        .rounded_full()
                        .bg(rgb(0x0284c7))
                        .w(px((progress * 7.5).max(4.0))),
                ),
        )
        // Typing Text Box
        .child(
            div()
                .p_4()
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
                .children(session.target_chars.iter().enumerate().map(|(idx, &ch)| {
                    let status = session.char_statuses[idx];
                    let is_cursor = idx == session.cursor_idx;
                    let display_char = if ch == ' ' { "␣" } else { &ch.to_string() };

                    let (text_color, bg_color) = match status {
                        CharStatus::Correct => (rgb(0x15803d), rgb(0xdcfce7)),
                        CharStatus::Incorrect(_) => (rgb(0xb91c1c), rgb(0xfee2e2)),
                        CharStatus::Pending => {
                            if is_cursor {
                                (rgb(0xffffff), rgb(0x0284c7))
                            } else {
                                (rgb(0x475569), rgb(0xffffff))
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
                })),
        )
        // Unified Guidance Console (Keyboard + Anatomical Hands)
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
                .gap_3()
                // Guidance Prompt Banner
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .w_full()
                        .px_4()
                        .py_1p5()
                        .rounded_lg()
                        .bg(rgb(0xe0f2fe))
                        .border_1()
                        .border_color(rgb(0xbae6fd))
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .child(div().text_size(px(15.0)).child("👉"))
                                .child(
                                    div()
                                        .text_size(px(12.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x0369a1))
                                        .child(hands_model.active_finger_instruction()),
                                ),
                        )
                        .child(
                            div()
                                .text_size(px(10.0))
                                .font_weight(FontWeight::SEMIBOLD)
                                .text_color(rgb(0x0369a1))
                                .child("Home Row: A S D F (Left) | J K L ; (Right)"),
                        ),
                )
                // Visual Keyboard
                .child(render_keyboard(active_char, active_finger))
                // Visual Hands
                .child(render_hands(&hands_model)),
        )
        .into_any_element()
}

fn metric_pill(label: &'static str, value: String, text_color: Rgba, bg: Rgba, border: Rgba) -> Div {
    div()
        .flex()
        .flex_col()
        .items_center()
        .px_3p5()
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

fn render_keyboard(active_char: Option<char>, _active_finger: Option<Finger>) -> impl IntoElement {
    let layout = get_keyboard_layout();
    let target_lower = active_char.map(|c| c.to_ascii_lowercase());

    div()
        .flex()
        .flex_col()
        .gap_1()
        .children(layout.into_iter().map(|row| {
            div()
                .flex()
                .gap_1()
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

                    let width_px = key.width_units * 38.0;
                    let is_home_bump = key.label == "F" || key.label == "J";

                    div()
                        .w(px(width_px))
                        .h(px(34.0))
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
                                .text_size(px(11.0))
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
                                    .w(px(4.0))
                                    .h(px(1.5))
                                    .rounded_full()
                                    .bg(rgb(0x64748b)),
                            )
                        } else {
                            None
                        })
                }))
        }))
}

fn render_hands(hands: &HandsGuideModel) -> impl IntoElement {
    div()
        .flex()
        .justify_around()
        .items_end()
        .w_full()
        .px_6()
        .child(
            // Left Hand
            div()
                .flex()
                .flex_col()
                .items_center()
                .child(
                    div()
                        .flex()
                        .items_end()
                        .gap_1p5()
                        .child(render_finger(hands.left_pinky, hands.target_char))
                        .child(render_finger(hands.left_ring, hands.target_char))
                        .child(render_finger(hands.left_middle, hands.target_char))
                        .child(render_finger(hands.left_index, hands.target_char))
                        .child(render_finger(hands.left_thumb, hands.target_char)),
                )
                .child(
                    div()
                        .w(px(165.0))
                        .h(px(38.0))
                        .rounded_b_2xl()
                        .bg(rgb(0xf1f5f9))
                        .border_1()
                        .border_color(rgb(0xcbd5e1))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_size(px(10.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x64748b))
                                .child("LEFT HAND"),
                        ),
                ),
        )
        .child(
            // Right Hand
            div()
                .flex()
                .flex_col()
                .items_center()
                .child(
                    div()
                        .flex()
                        .items_end()
                        .gap_1p5()
                        .child(render_finger(hands.right_thumb, hands.target_char))
                        .child(render_finger(hands.right_index, hands.target_char))
                        .child(render_finger(hands.right_middle, hands.target_char))
                        .child(render_finger(hands.right_ring, hands.target_char))
                        .child(render_finger(hands.right_pinky, hands.target_char)),
                )
                .child(
                    div()
                        .w(px(165.0))
                        .h(px(38.0))
                        .rounded_b_2xl()
                        .bg(rgb(0xf1f5f9))
                        .border_1()
                        .border_color(rgb(0xcbd5e1))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(
                            div()
                                .text_size(px(10.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x64748b))
                                .child("RIGHT HAND"),
                        ),
                ),
        )
}

fn render_finger(finger: HandFingerState, target_char: Option<char>) -> impl IntoElement {
    let height = if finger.is_active { finger.active_height } else { finger.normal_height };
    let (f_r, f_g, f_b) = finger.finger.rgb();
    let finger_color = rgb((f_r as u32) << 16 | (f_g as u32) << 8 | (f_b as u32));

    let bg_color = if finger.is_active { finger_color } else { rgb(0xf8fafc) };
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
        .w(px(finger.width))
        .h(px(height))
        .rounded_t_full()
        .bg(bg_color)
        .border_2()
        .border_color(border_color)
        .flex()
        .flex_col()
        .items_center()
        .justify_between()
        .py_1()
        .child(
            div()
                .w(px(20.0))
                .h(px(20.0))
                .rounded_full()
                .bg(if finger.is_active { rgb(0xffffff) } else { rgb(0xe2e8f0) })
                .border_1()
                .border_color(if finger.is_active { rgb(0x0f172a) } else { rgb(0x94a3b8) })
                .flex()
                .items_center()
                .justify_center()
                .child(
                    div()
                        .text_size(px(10.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(if finger.is_active { finger_color } else { rgb(0x334155) })
                        .child(tip_label),
                ),
        )
        .child(
            div()
                .text_size(px(8.5))
                .font_weight(FontWeight::BOLD)
                .text_color(if finger.is_active { rgb(0xffffff) } else { rgb(0x64748b) })
                .child(finger.label),
        )
}
