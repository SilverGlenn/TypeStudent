use crate::app::TypeStudentView;
use crate::state::ActiveView;
use crate::views::diploma_export::export_diploma_html;
use gpui::*;

pub fn render_diploma(view: &TypeStudentView, cx: &mut Context<TypeStudentView>) -> AnyElement {
    let diploma = match &view.state.active_diploma {
        Some(d) => d,
        None => return div().child("No diploma available").into_any_element(),
    };

    let student_name = diploma.student_name.clone();
    let test_title = diploma.test_title.clone();
    let net_wpm = diploma.net_wpm;
    let accuracy = diploma.accuracy;
    let date = diploma.date.clone();
    let duration = diploma.duration_label.clone();
    let export_msg = view.export_status.clone();

    div()
        .flex()
        .flex_col()
        .items_center()
        .gap_5()
        .p_6()
        .rounded_2xl()
        .bg(rgb(0xfefce8))
        .border_4()
        .border_color(rgb(0xd97706))
        .text_color(rgb(0x78350f))
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_1()
                .child(div().text_size(px(36.0)).child("🏆"))
                .child(
                    div()
                        .text_size(px(24.0))
                        .font_weight(FontWeight::BOLD)
                        .child("Certificate of Touch Typing Achievement"),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .child("This officially certifies that"),
                )
                .child(
                    div()
                        .text_size(px(22.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x92400e))
                        .child(diploma.student_name.clone()),
                )
                .child(
                    div()
                        .text_size(px(13.0))
                        .child(format!("has successfully completed: {}", diploma.test_title)),
                ),
        )
        .child(
            div()
                .flex()
                .gap_5()
                .p_3p5()
                .rounded_xl()
                .bg(rgb(0xfef08a))
                .border_1()
                .border_color(rgb(0xfacc15))
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .child(
                            div()
                                .text_size(px(20.0))
                                .font_weight(FontWeight::BOLD)
                                .child(format!("{:.1} WPM", diploma.net_wpm)),
                        )
                        .child(div().text_size(px(10.0)).child("Net Speed")),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .child(
                            div()
                                .text_size(px(20.0))
                                .font_weight(FontWeight::BOLD)
                                .child(format!("{:.1}%", diploma.accuracy)),
                        )
                        .child(div().text_size(px(10.0)).child("Accuracy")),
                )
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .items_center()
                        .child(
                            div()
                                .text_size(px(20.0))
                                .font_weight(FontWeight::BOLD)
                                .child(diploma.duration_label.clone()),
                        )
                        .child(div().text_size(px(10.0)).child("Duration")),
                ),
        )
        .child(
            div()
                .flex()
                .justify_between()
                .w_full()
                .px_8()
                .mt_1()
                .child(div().text_size(px(11.0)).child(format!("Date Issued: {}", diploma.date)))
                .child(div().text_size(px(11.0)).child("TypeStudent Examination Board 📜")),
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
                        .gap_3()
                        .child(
                            div()
                                .id("btn_export_html_diploma")
                                .px_5()
                                .py_2()
                                .rounded_lg()
                                .bg(rgb(0x0284c7))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0x0369a1)))
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                        if let Some(path) = export_diploma_html(
                                            &student_name,
                                            &test_title,
                                            net_wpm,
                                            accuracy,
                                            &date,
                                            &duration,
                                        ) {
                                            this.export_status = Some(format!("Saved: {}", path.display()));
                                        }
                                        cx.notify();
                                    }),
                                )
                                .child("🖨️ Export & Print Certificate (HTML)"),
                        )
                        .child(
                            div()
                                .id("btn_back_from_diploma")
                                .px_5()
                                .py_2()
                                .rounded_lg()
                                .bg(rgb(0xd97706))
                                .text_size(px(12.0))
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xffffff))
                                .cursor_pointer()
                                .on_mouse_down(
                                    MouseButton::Left,
                                    cx.listener(|this, _event: &MouseDownEvent, _window, cx| {
                                        this.state.active_view = ActiveView::TypingTests;
                                        cx.notify();
                                    }),
                                )
                                .child("Back to Tests"),
                        ),
                )
                .children(export_msg.map(|msg| {
                    div()
                        .text_size(px(11.0))
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x15803d))
                        .child(format!("✓ Certificate exported to {}", msg))
                })),
        )
        .into_any_element()
}
