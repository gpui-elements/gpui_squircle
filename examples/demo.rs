use gpui::{App, Application, Bounds, ParentElement, Render, Styled, WindowBounds, WindowOptions, div, prelude::*, px, rems, size};
use gpui_squircle::{squircle, SquircleStylable};

fn rounded_rect_div() -> impl gpui::IntoElement {
    div()
        .size(px(200.))
        .rounded(px(50.))
        .bg(gpui::red())
        .p(px(35.))
        .flex()
        .justify_center()
        .items_center()
        .text_size(rems(1.5))

        .child("rounded rect")
}

fn squircle_div() -> impl gpui::IntoElement {
    div()
        .size(px(200.))
        .p(px(35.))
        .flex()
        .justify_center()
        .items_center()
        .text_size(rems(1.5))

        .child(
            // To use a squircle simply parent it to an element.
            // It automatically fills the parent's entire size
            // whilst also ignoring padding. 
            squircle()
                .rounded(px(50.))
                .bg(gpui::red())
        )

        .child("squircle")
}

struct MyGpuiApp;

impl Render for MyGpuiApp {
    fn render(&mut self, _window: &mut gpui::Window, _cx: &mut gpui::Context<Self>) -> impl gpui::IntoElement {
        div()
            .size_full()
            .flex()
            .gap(px(25.))
            .justify_center()
            .items_center()

            .child(rounded_rect_div())
            .child(squircle_div())
    }
}

fn main() {
    Application::new().run(|cx: &mut App| {
        let bounds =
            Bounds::centered(None, size(px(475.), px(250.0)), cx);

        cx
            .open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |_, cx| {
                    cx.new(|_| MyGpuiApp)
                },
            )
            .unwrap();

        cx.activate(true);
    })
}