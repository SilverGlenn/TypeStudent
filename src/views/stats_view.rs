use crate::app::TypeStudentView;
use crate::components::ui::page_header;
use gpui::*;

pub fn render_statistics(view: &TypeStudentView, _cx: &mut Context<TypeStudentView>) -> AnyElement {
    let profile = view.state.profile_store.active_profile();

    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(page_header(
            "📊 Touch Typing Statistics & Heatmap",
            "Per-key accuracy diagnostics and cumulative typing performance metrics.",
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
                        .child("Alphabet Accuracy Heatmap:"),
                )
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_2()
                        .children(('a'..='z').map(|c| {
                            let stat = profile.and_then(|p| p.key_stats.get(&c));
                            let acc = stat.map(|s| s.accuracy()).unwrap_or(100.0);
                            let total = stat.map(|s| s.hits + s.misses).unwrap_or(0);

                            let (bg_color, border_color, text_color) = if total == 0 {
                                (rgb(0xf8fafc), rgb(0xe2e8f0), rgb(0x94a3b8))
                            } else if acc >= 95.0 {
                                (rgb(0xf0fdf4), rgb(0x86efac), rgb(0x15803d))
                            } else if acc >= 85.0 {
                                (rgb(0xfefce8), rgb(0xfde047), rgb(0xa16207))
                            } else {
                                (rgb(0xfef2f2), rgb(0xfca5a5), rgb(0xdc2626))
                            };

                            div()
                                .flex()
                                .flex_col()
                                .items_center()
                                .w(px(48.0))
                                .py_1p5()
                                .rounded_lg()
                                .bg(bg_color)
                                .border_1()
                                .border_color(border_color)
                                .child(
                                    div()
                                        .text_size(px(15.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(text_color)
                                        .child(c.to_uppercase().to_string()),
                                )
                                .child(
                                    div()
                                        .text_size(px(9.5))
                                        .text_color(text_color)
                                        .child(if total == 0 {
                                            "-".to_string()
                                        } else {
                                            format!("{:.0}%", acc)
                                        }),
                                )
                        })),
                ),
        )
        .into_any_element()
}
