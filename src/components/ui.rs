use gpui::*;

/// Standard styled container card with clean borders and soft background
pub fn card() -> Div {
    div()
        .p_5()
        .rounded_xl()
        .bg(rgb(0xffffff))
        .border_1()
        .border_color(rgb(0xe2e8f0))
}

/// Standard page/view header card
pub fn page_header(title: impl Into<SharedString>, subtitle: impl Into<SharedString>) -> Div {
    card()
        .flex()
        .flex_col()
        .gap_1()
        .child(
            div()
                .text_size(px(18.0))
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0x0f172a))
                .child(title.into()),
        )
        .child(
            div()
                .text_size(px(12.0))
                .text_color(rgb(0x64748b))
                .child(subtitle.into()),
        )
}

/// Compact metric tile for HUDs and results summaries
pub fn metric_tile(label: &'static str, value: String, accent: Rgba) -> Div {
    div()
        .flex()
        .flex_col()
        .items_center()
        .px_4()
        .py_2()
        .rounded_xl()
        .bg(rgb(0xf8fafc))
        .border_1()
        .border_color(rgb(0xe2e8f0))
        .child(
            div()
                .text_size(px(16.0))
                .font_weight(FontWeight::BOLD)
                .text_color(accent)
                .child(value),
        )
        .child(
            div()
                .text_size(px(9.5))
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(0x64748b))
                .child(label),
        )
}
