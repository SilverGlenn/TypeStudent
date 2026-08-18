use crate::app::TypeStudentView;
use crate::components::ui::page_header;
use gpui::*;

pub fn render_settings(view: &TypeStudentView, cx: &mut Context<TypeStudentView>) -> AnyElement {
    let profile = view.state.profile_store.active_profile();
    let sound_enabled = profile.map(|p| p.settings.sound_enabled).unwrap_or(true);
    let target_wpm = profile.map(|p| p.settings.target_wpm).unwrap_or(35);

    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(page_header(
            "⚙️ Settings & Configuration",
            "Configure procedural audio effects, benchmark speed goals, and accessibility.",
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
                .gap_3p5()
                .child(
                    div()
                        .flex()
                        .justify_between()
                        .items_center()
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_size(px(13.5))
                                        .font_weight(FontWeight::BOLD)
                                        .child("Procedural Sound Effects"),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(0x64748b))
                                        .child("Mechanical keyclicks, chimes, and mistake audio indicators"),
                                ),
                        )
                        .child(
                            div()
                                .id("toggle_sound_btn")
                                .px_3p5()
                                .py_1p5()
                                .rounded_lg()
                                .bg(if sound_enabled {
                                    rgb(0x0284c7)
                                } else {
                                    rgb(0x94a3b8)
                                })
                                .text_size(px(11.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .cursor_pointer()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                        if let Some(p) = this.state.profile_store.active_profile_mut() {
                                            p.settings.sound_enabled = !p.settings.sound_enabled;
                                            let en = p.settings.sound_enabled;
                                            this.state.audio.set_enabled(en);
                                        }
                                        this.state.profile_store.save();
                                        cx.notify();
                                    }),
                                )
                                .child(if sound_enabled {
                                    "Enabled 🔊"
                                } else {
                                    "Muted 🔇"
                                }),
                        ),
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
                                .child(
                                    div()
                                        .text_size(px(13.5))
                                        .font_weight(FontWeight::BOLD)
                                        .child("Target Speed Goal"),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(0x64748b))
                                        .child("Benchmark speed used for 5-star calculations"),
                                ),
                        )
                        .child(
                            div()
                                .px_3p5()
                                .py_1p5()
                                .rounded_lg()
                                .bg(rgb(0xf8fafc))
                                .border_1()
                                .border_color(rgb(0xbae6fd))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0x0369a1))
                                .child(format!("{} WPM", target_wpm)),
                        ),
                ),
        )
        .into_any_element()
}
