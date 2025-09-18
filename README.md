# GPUI Squircle

> A squircle component for gpui.

![rounded rect vs squircle](rounded_rect_vs_squircle.png)

## Installation

gpui is still in development so you need to add the dependencies via git.

```sh
gpui = { git = "https://github.com/zed-industries/zed.git" }
gpui_squircle = { git = "https://github.com/gpui-elements/gpui_squircle.git" }
```

## Usage

```rs
use gpui::{ParentElement, Styled, div, px};
use gpui_squircle::{squircle, SquircleStylable};

fn squircle_div() -> impl gpui::IntoElement {
    div()
        .size(px(200.))

        .child(
            // To use a squircle simply parent it to an element.
            // It automatically fills the parent's entire size
            // whilst also ignoring padding. 
            squircle()
                .rounded(px(25.))
                .bg(gpui::red())
                .border(px(15.))
                .border_color(gpui::blue())
                .border_outside()
        )
}
```

## Examples

Examples can be found [here](/examples)