use crate::app::TypeStudentView;
use crate::components::ui::card;
use crate::trophies::get_all_trophies;
use gpui::*;

pub fn render_trophies(view: &TypeStudentView, _cx: &mut Context<TypeStudentView>) -> AnyElement {
    let trophies = get_all_trophies();
    let profile = view.state.profile_store.active_profile();
    let unlocked_count = trophies
        .iter()
        .filter(|t| profile.map_or(false, |p| p.has_trophy(t.id)))
        .count();

    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(
            card()
                .flex()
                .justify_between()
                .items_center()
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
                                .child("🏆 Trophy Room & Achievement Badges"),
                        )
                        .child(
                            div()
                                .text_size(px(12.0))
                                .text_color(rgb(0x64748b))
                                .child("Milestones tracking speed thresholds, accuracy streaks, and lesson completions."),
                        ),
                )
                .child(
                    div()
                        .px_3p5()
                        .py_1p5()
                        .rounded_lg()
                        .bg(rgb(0xfef3c7))
                        .border_1()
                        .border_color(rgb(0xfde68a))
                        .text_size(px(13.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x92400e))
                        .child(format!("Unlocked: {} / {} Badges", unlocked_count, trophies.len())),
                ),
        )
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_3p5()
                .children(trophies.into_iter().map(|trophy| {
                    let is_unlocked = profile.map_or(false, |p| p.has_trophy(trophy.id));
                    let unlocked_date = profile.and_then(|p| p.unlocked_trophies.get(trophy.id)).cloned();

                    let bg_color = if is_unlocked { rgb(0xffffff) } else { rgb(0xf8fafc) };
                    let border_color = if is_unlocked { rgb(0xf59e0b) } else { rgb(0xe2e8f0) };

                    div()
                        .flex()
                        .items_center()
                        .w(px(310.0))
                        .p_3p5()
                        .rounded_xl()
                        .bg(bg_color)
                        .border_1()
                        .border_color(border_color)
                        .gap_3()
                        .child(
                            div()
                                .w(px(46.0))
                                .h(px(46.0))
                                .rounded_full()
                                .bg(if is_unlocked { rgb(0xfef3c7) } else { rgb(0xe2e8f0) })
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_size(px(24.0))
                                .child(if is_unlocked { trophy.icon } else { "🔒" }),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_0p5()
                                .child(
                                    div()
                                        .text_size(px(13.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if is_unlocked {
                                            rgb(0x0f172a)
                                        } else {
                                            rgb(0x64748b)
                                        })
                                        .child(trophy.title),
                                )
                                .child(
                                    div()
                                        .text_size(px(10.5))
                                        .text_color(rgb(0x64748b))
                                        .line_height(px(14.0))
                                        .child(trophy.description),
                                )
                                .child(
                                    div()
                                        .text_size(px(9.5))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(if is_unlocked {
                                            rgb(0x15803d)
                                        } else {
                                            rgb(0x94a3b8)
                                        })
                                        .child(if is_unlocked {
                                            format!("Unlocked ✓ {}", unlocked_date.as_deref().unwrap_or(""))
                                        } else {
                                            "Locked 🔒".to_string()
                                        }),
                                ),
                        )
                })),
        )
        .into_any_element()
}
