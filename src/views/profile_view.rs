use crate::app::TypeStudentView;
use crate::components::ui::page_header;
use gpui::*;

pub fn render_profiles(view: &mut TypeStudentView, cx: &mut Context<TypeStudentView>) -> AnyElement {
    let profiles = view.state.profile_store.profiles.clone();
    let active_id = view.state.profile_store.active_profile_id.clone();

    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(page_header(
            "👥 Student Profiles",
            "Multiple student accounts for independent progress and lesson history.",
        ))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_3p5()
                .children(profiles.into_iter().enumerate().map(|(idx, p)| {
                    let is_active = p.id == active_id;
                    let p_id = p.id.clone();
                    let prof_btn_id = format!("prof_btn_{}", idx);

                    div()
                        .flex()
                        .justify_between()
                        .items_center()
                        .w(px(310.0))
                        .p_3p5()
                        .rounded_xl()
                        .bg(rgb(0xffffff))
                        .border_1()
                        .border_color(if is_active {
                            rgb(0x0284c7)
                        } else {
                            rgb(0xe2e8f0)
                        })
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2p5()
                                .child(div().text_size(px(26.0)).child(p.avatar_emoji.clone()))
                                .child(
                                    div()
                                        .flex()
                                        .flex_col()
                                        .child(
                                            div()
                                                .text_size(px(14.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0x0f172a))
                                                .child(p.name.clone()),
                                        )
                                        .child(
                                            div()
                                                .text_size(px(10.5))
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(rgb(0xd97706))
                                                .child(format!(
                                                    "⭐ {} Stars | {} Lessons",
                                                    p.total_stars(),
                                                    p.total_lessons_passed()
                                                )),
                                        ),
                                ),
                        )
                        .child(if is_active {
                            div()
                                .px_3()
                                .py_1()
                                .rounded_md()
                                .bg(rgb(0x0284c7))
                                .text_size(px(10.5))
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
                                .text_size(px(10.5))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x334155))
                                .cursor_pointer()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                        this.state.profile_store.switch_profile(&p_id);
                                        cx.notify();
                                    }),
                                )
                                .child("Select")
                                .into_any_element()
                        })
                })),
        )
        .into_any_element()
}
