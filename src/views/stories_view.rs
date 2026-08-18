use crate::app::TypeStudentView;
use crate::components::ui::page_header;
use crate::views::stories_data::get_all_stories;
use gpui::*;

pub fn render_story_studio(_view: &TypeStudentView, cx: &mut Context<TypeStudentView>) -> AnyElement {
    let stories = get_all_stories();

    div()
        .flex()
        .flex_col()
        .gap_5()
        .child(page_header(
            "📖 Story Studio & Creative Typing",
            "Sentence-level touch typing exercises across varied fiction and educational passages.",
        ))
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_3p5()
                .children(stories.into_iter().enumerate().map(|(idx, story)| {
                    let title = story.title.to_string();
                    let content = story.content.to_string();
                    let story_btn_id = format!("story_btn_{}", idx);

                    div()
                        .flex()
                        .flex_col()
                        .justify_between()
                        .w(px(310.0))
                        .p_4()
                        .rounded_xl()
                        .bg(rgb(0xffffff))
                        .border_1()
                        .border_color(rgb(0xe2e8f0))
                        .gap_3()
                        .hover(|s| s.bg(rgb(0xf8fafc)).border_color(rgb(0x38bdf8)))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .gap_1p5()
                                .child(
                                    div()
                                        .flex()
                                        .items_center()
                                        .gap_2()
                                        .child(div().text_size(px(24.0)).child(story.emoji))
                                        .child(
                                            div()
                                                .text_size(px(15.0))
                                                .font_weight(FontWeight::BOLD)
                                                .text_color(rgb(0x0f172a))
                                                .child(story.title),
                                        ),
                                )
                                .child(
                                    div()
                                        .text_size(px(11.0))
                                        .text_color(rgb(0x64748b))
                                        .line_height(px(16.0))
                                        .child(format!(
                                            "{}...",
                                            &story.content[..story.content.len().min(80)]
                                        )),
                                ),
                        )
                        .child(
                            div()
                                .flex()
                                .justify_between()
                                .items_center()
                                .child(
                                    div()
                                        .text_size(px(10.0))
                                        .text_color(rgb(0x0284c7))
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .child(format!("{} Words | {}", story.word_count, story.difficulty)),
                                )
                                .child(
                                    div()
                                        .id(ElementId::Name(story_btn_id.into()))
                                        .px_3()
                                        .py_1()
                                        .rounded_lg()
                                        .bg(rgb(0x0284c7))
                                        .text_size(px(11.0))
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xffffff))
                                        .cursor_pointer()
                                        .hover(|s| s.bg(rgb(0x0369a1)))
                                        .on_mouse_down(
                                            MouseButton::Left,
                                            cx.listener(move |this, _event: &MouseDownEvent, _window, cx| {
                                                this.state.start_story_practice(&title, &content);
                                                cx.notify();
                                            }),
                                        )
                                        .child("Read & Type ▶"),
                                ),
                        )
                })),
        )
        .into_any_element()
}
