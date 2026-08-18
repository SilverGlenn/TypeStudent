mod app;

use app::TypeStudentView;
use gpui::*;

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1280.0), px(880.0)), cx);
        let options = WindowOptions {
            window_bounds: Some(WindowBounds::Windowed(bounds)),
            titlebar: Some(TitlebarOptions {
                title: Some("TypeStudent Pro - Touch Typing Tutor".into()),
                ..Default::default()
            }),
            ..Default::default()
        };

        cx.open_window(options, |window, cx| {
            cx.new(|cx| {
                let view = TypeStudentView::new(cx);
                window.focus(&view.focus_handle);
                view
            })
        })
        .unwrap();
    });
}
