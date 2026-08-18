use crate::app::TypeStudentView;
use crate::components::ui::page_header;
use gpui::*;

pub fn render_typing_tests(_view: &TypeStudentView, cx: &mut Context<TypeStudentView>) -> AnyElement {
    let tests = vec![
        (
            "Aesop's Fables: The Fox and Grapes",
            120,
            "One warm summer day, a Fox was strolling through an orchard until he came to a bunch of Grapes just ripening on a vine...",
        ),
        (
            "The History of the Airplane",
            180,
            "For centuries, humans watched birds soar freely across the skies and dreamed of building wings to fly...",
        ),
        (
            "Albert Einstein and Curiosity",
            300,
            "Albert Einstein once stated that he had no special talents, but was only passionately curious about the universe...",
        ),
        (
            "Speed Sprint (1 Minute)",
            60,
            "Touch typing is an essential 21st century skill that unlocks high productivity, creativity, and rapid programming speed.",
        ),
    ];

    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(page_header(
            "⏱️ Standardized Typing Tests",
            "Timed typing evaluations from 1 to 5 minutes. Passing an exam generates a Certificate Diploma.",
        ))
        .child(
            div()
                .flex()
                .flex_col()
                .gap_2p5()
                .children(tests.into_iter().enumerate().map(|(idx, (name, duration, text))| {
                    let minutes = duration / 60;
                    let text_copy = text.to_string();
                    let name_copy = name.to_string();
                    let test_btn_id = format!("test_btn_{}", idx);

                    div()
                        .flex()
                        .justify_between()
                        .items_center()
                        .p_3p5()
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
                                        .text_size(px(15.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x0f172a))
                                        .child(name),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(0x64748b))
                                        .child(format!("Duration: {} min | Standardized Text", minutes)),
                                ),
                        )
                        .child(
                            div()
                                .id(ElementId::Name(test_btn_id.into()))
                                .px_3p5()
                                .py_1p5()
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
                                        this.state.start_typing_test(&name_copy, &text_copy, duration);
                                        cx.notify();
                                    }),
                                )
                                .child("Begin Test ▶"),
                        )
                })),
        )
        .into_any_element()
}
