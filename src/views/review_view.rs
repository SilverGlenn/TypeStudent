use crate::app::TypeStudentView;
use crate::components::ui::page_header;
use gpui::*;

pub fn render_smart_review(view: &TypeStudentView, cx: &mut Context<TypeStudentView>) -> AnyElement {
    let profile = view.state.profile_store.active_profile();
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
        .gap_5()
        .child(page_header(
            "🎯 Smart Weak Key Review",
            "Automatic latency and error frequency analysis. Targeted remedial drills to eliminate weak spots.",
        ))
        .child(
            div()
                .p_5()
                .rounded_xl()
                .bg(rgb(0xffffff))
                .border_1()
                .border_color(rgb(0xe2e8f0))
                .flex()
                .flex_col()
                .gap_3()
                .child(
                    div()
                        .text_size(px(15.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x0f172a))
                        .child("Current Difficult Keys:"),
                )
                .child(if difficult_keys.is_empty() {
                    div()
                        .text_size(px(12.0))
                        .text_color(rgb(0x15803d))
                        .child("✨ No weak keys detected. High accuracy across all evaluated keys.")
                        .into_any_element()
                } else {
                    div()
                        .flex()
                        .gap_2p5()
                        .children(difficult_keys.into_iter().map(|(c, acc)| {
                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .px_3()
                                .py_1p5()
                                .rounded_lg()
                                .bg(rgb(0xfef2f2))
                                .border_1()
                                .border_color(rgb(0xfecaca))
                                .child(
                                    div()
                                        .text_size(px(16.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xdc2626))
                                        .child(c.to_uppercase().to_string()),
                                )
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .text_color(rgb(0x64748b))
                                        .child(format!("{:.0}% Acc", acc)),
                                )
                        }))
                        .into_any_element()
                })
                .child(
                    div()
                        .id("btn_start_smart_review")
                        .mt_2()
                        .px_5()
                        .py_2p5()
                        .w(px(200.0))
                        .rounded_lg()
                        .bg(rgb(0x0284c7))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xffffff))
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(0x0369a1)))
                        .on_mouse_down(
                            MouseButton::Left,
                            cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                this.state.start_smart_review(&review_text);
                                cx.notify();
                            }),
                        )
                        .child("Start Targeted Drill ▶"),
                ),
        )
        .into_any_element()
}
