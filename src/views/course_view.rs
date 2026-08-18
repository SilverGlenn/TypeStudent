use crate::app::TypeStudentView;
use crate::components::ui::page_header;
use gpui::*;

pub fn render_course_overview(view: &TypeStudentView, cx: &mut Context<TypeStudentView>) -> AnyElement {
    let lessons = view.state.lessons.clone();
    let profile = view.state.profile_store.active_profile();

    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(page_header(
            "12-Lesson Touch Typing Course",
            "Progressive muscle memory training. Learn every key with structured finger placement.",
        ))
        .child(
            div()
                .flex()
                .flex_col()
                .gap_3()
                .children(lessons.into_iter().enumerate().map(|(l_idx, lesson)| {
                    let keys_str = lesson
                        .keys_introduced
                        .iter()
                        .map(|c| c.to_uppercase().to_string())
                        .collect::<Vec<_>>()
                        .join(" ");

                    div()
                        .flex()
                        .flex_col()
                        .p_4()
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
                                                .w(px(28.0))
                                                .h(px(28.0))
                                                .rounded_full()
                                                .bg(rgb(0x0284c7))
                                                .flex()
                                                .items_center()
                                                .justify_center()
                                                .font_weight(FontWeight::BOLD)
                                                .text_size(px(13.0))
                                                .text_color(rgb(0xffffff))
                                                .child(format!("{}", lesson.number)),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .child(
                                                    div()
                                                        .text_size(px(15.0))
                                                        .font_weight(FontWeight::BOLD)
                                                        .text_color(rgb(0x0f172a))
                                                        .child(format!("Lesson {}: {}", lesson.number, lesson.title)),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(11.0))
                                                        .text_color(rgb(0x64748b))
                                                        .child(lesson.subtitle.clone()),
                                                ),
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
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x0369a1))
                                        .child(format!("Keys: {}", keys_str)),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_wrap()
                                .gap_2()
                                .children(lesson.exercises.into_iter().enumerate().map(|(e_idx, ex)| {
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
                                        .w(px(240.0))
                                        .p_2p5()
                                        .rounded_lg()
                                        .bg(rgb(0xf8fafc))
                                        .border_1()
                                        .border_color(if record.is_some() {
                                            rgb(0x38bdf8)
                                        } else {
                                            rgb(0xe2e8f0)
                                        })
                                        .cursor_pointer()
                                        .hover(|s| s.bg(rgb(0xe0f2fe)))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                                this.state.start_exercise(l_idx, e_idx);
                                                cx.notify();
                                            }),
                                        )
                                        .child(
                                            div()
                                                .flex()
                                                .flex_col()
                                                .gap_0p5()
                                                .child(
                                                    div()
                                                        .flex()
                                                        .items_center()
                                                        .gap_1p5()
                                                        .child(
                                                            div().text_size(px(12.0)).child(ex.exercise_type.icon()),
                                                        )
                                                        .child(
                                                            div()
                                                                .text_size(px(12.0))
                                                                .font_weight(FontWeight::SEMIBOLD)
                                                                .text_color(rgb(0x0f172a))
                                                                .child(ex.title),
                                                        ),
                                                )
                                                .child(
                                                    div()
                                                        .text_size(px(10.0))
                                                        .text_color(rgb(0xd97706))
                                                        .child(stars_str),
                                                ),
                                        )
                                        .child(
                                            div()
                                                .px_2()
                                                .py_1()
                                                .rounded_md()
                                                .bg(rgb(0x0284c7))
                                                .text_size(px(10.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0xffffff))
                                                .child("Start ▶"),
                                        )
                                })),
                        )
                })),
        )
        .into_any_element()
}
