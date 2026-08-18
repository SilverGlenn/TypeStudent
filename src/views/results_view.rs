use crate::app::TypeStudentView;
use crate::components::ui::metric_tile;
use gpui::*;

pub fn render_exercise_results(view: &TypeStudentView, cx: &mut Context<TypeStudentView>) -> AnyElement {
    let session = match &view.state.typing_session {
        Some(s) => s,
        None => return div().child("No result available").into_any_element(),
    };
    let info = view.state.current_exercise_info.as_ref();
    let title = info.map(|i| i.title.clone()).unwrap_or_else(|| "Exercise Completed".to_string());
    let target_wpm = view.state.profile_store.active_profile().map(|p| p.settings.target_wpm).unwrap_or(35);
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
                .child(div().text_size(px(40.0)).child("🎉"))
                .child(
                    div()
                        .text_size(px(20.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x0f172a))
                        .child(title),
                )
                .child(
                    div()
                        .text_size(px(16.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xd97706))
                        .child(stars_visual),
                ),
        )
        // Metrics Grid
        .child(
            div()
                .flex()
                .gap_3()
                .child(metric_tile("NET SPEED", format!("{:.1} WPM", net_wpm), rgb(0x0284c7)))
                .child(metric_tile("GROSS SPEED", format!("{:.1} WPM", gross_wpm), rgb(0x64748b)))
                .child(metric_tile("ACCURACY", format!("{:.1}%", accuracy), rgb(0x15803d)))
                .child(metric_tile(
                    "ERRORS",
                    format!("{}", errors),
                    if errors > 0 { rgb(0xdc2626) } else { rgb(0x15803d) },
                ))
                .child(metric_tile("TIME", format!("{:.0}s", elapsed), rgb(0xd97706))),
        )
        // Actions
        .child(
            div()
                .flex()
                .gap_3()
                .child(
                    div()
                        .id("btn_next_exercise")
                        .px_5()
                        .py_2p5()
                        .rounded_lg()
                        .bg(rgb(0x0284c7))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(0x0369a1)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                this.state.next_exercise();
                                cx.notify();
                            }),
                        )
                        .child("Next Exercise ▶"),
                )
                .child(
                    div()
                        .id("btn_repeat_exercise")
                        .px_5()
                        .py_2p5()
                        .rounded_lg()
                        .bg(rgb(0xf1f5f9))
                        .border_1()
                        .border_color(rgb(0xcbd5e1))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x334155))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(0xe2e8f0)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                this.state.restart_current_exercise();
                                cx.notify();
                            }),
                        )
                        .child("Repeat Drill ↺"),
                ),
        )
        .into_any_element()
}
