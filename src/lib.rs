use gpui::{App, Background, Bounds, PathBuilder, Pixels, Size, Window, canvas, point, prelude::*, px};
use lyon::{extra::parser::{ParserOptions, PathParser, Source}, path::Path as LyonPath};
use figma_squircle::{FigmaSquircleParams, get_svg_path};

mod style_trait;
pub use style_trait::*;

struct BuildAndPaintOptions {
    builder: PathBuilder,
    background: Background
}

impl BuildAndPaintOptions {
    fn fill(background: Background) -> Self {
        Self {
            builder: PathBuilder::fill(),
            background
        }
    }

    fn stroke(background: Background, border_width: f32) -> Self {
        Self {
            builder: PathBuilder::stroke(px(border_width)),
            background
        }
    }
}

#[derive(IntoElement)]
pub struct Squircle {
    squircle_styles: SquircleStyles
}

impl SquircleStylable for Squircle {
    fn get_squircle_styles_mut(&mut self) -> &mut SquircleStyles {
        &mut self.squircle_styles
    }
}

impl Squircle {
    fn new() -> Self {
        Self {
            squircle_styles: SquircleStyles::default()
        }
    }

    fn to_params(&self, width: f32, height: f32, border_offset: f32) -> FigmaSquircleParams {
        let squircle_styles = &self.squircle_styles;

        FigmaSquircleParams {
            corner_radius: squircle_styles.corner_radius
                .map(|x| x - border_offset),
            top_left_corner_radius: squircle_styles.top_left_corner_radius
                .map(|x| x - border_offset),
            top_right_corner_radius: squircle_styles.top_right_corner_radius
                .map(|x| x - border_offset),
            bottom_right_corner_radius: squircle_styles.bottom_right_corner_radius
                .map(|x| x - border_offset),
            bottom_left_corner_radius: squircle_styles.bottom_left_corner_radius
                .map(|x| x - border_offset),
            corner_smoothing: squircle_styles.corner_smoothing.unwrap_or(1.),
            width, height,
            preserve_smoothing: squircle_styles.preserve_smoothing.unwrap_or(true)
        }
    }

    fn build_lyon_path(&self, size: Size<Pixels>, border_offset: f32) -> Option<LyonPath> {
        let svg_path = get_svg_path(self.to_params(
            size.width.to_f64() as f32,
            size.height.to_f64() as f32,
            border_offset
        ));

        let mut lyon_builder = LyonPath::builder();
        
        let parsed = PathParser::new()
            .parse(
                &ParserOptions::DEFAULT,
                &mut Source::new(svg_path.chars()),
                &mut lyon_builder
            );

        if parsed.is_err() {
            return None
        }
        
        Some(lyon_builder.build())
    }

    fn build_and_paint_paths<'a, const N: usize>(
        &self,
        window: &mut Window,
        Bounds { origin, size }: Bounds<Pixels>,
        border_offset: f32,
        mut options: [BuildAndPaintOptions; N]
    ) {
        let border_offset_px = px(border_offset);

        let size = size - gpui::size(
            border_offset_px,
            border_offset_px
        );

        // If the path doesn't exist then the svg is malformed.
        // TODO: fallback to regular rounded rectangle if this case is met.
        let Some(path) = self.build_lyon_path(size, border_offset) else { println!("malformed svg"); return };

        let (Pixels(origin_x), Pixels(origin_y)) = (
            origin.x + border_offset_px / 2.,
            origin.y + border_offset_px / 2.
        );

        for event in path.iter() {
            match event {
                lyon::path::Event::Begin { at } => {
                    let at = point(px(origin_x + at.x), px(origin_y + at.y));

                    for BuildAndPaintOptions {
                        builder, ..
                    } in options.as_mut() {
                        builder.move_to(at)
                    }
                }

                lyon::path::Event::Line { from:_, to } => {
                    let to = point(px(origin_x + to.x), px(origin_y + to.y));

                    for BuildAndPaintOptions {
                        builder, ..
                    } in options.as_mut() {
                        builder.line_to(to)
                    }
                }

                lyon::path::Event::Quadratic { from:_, ctrl, to } => {
                    let to = point(px(origin_x + to.x), px(origin_y + to.y));
                    let ctrl = point(px(origin_x + ctrl.x), px(origin_y + ctrl.y));

                    for BuildAndPaintOptions {
                        builder, ..
                    } in options.as_mut() {
                        builder.curve_to(to, ctrl);
                    }
                }

                lyon::path::Event::Cubic { from:_, ctrl1, ctrl2, to } => {
                    let to = point(px(origin_x + to.x), px(origin_y + to.y));
                    let ctrl1 = point(px(origin_x + ctrl1.x), px(origin_y + ctrl1.y));
                    let ctrl2 = point(px(origin_x + ctrl2.x), px(origin_y + ctrl2.y));

                    for BuildAndPaintOptions {
                        builder, ..
                    } in options.as_mut() {
                        builder.cubic_bezier_to(to, ctrl1, ctrl2)
                    }
                }

                lyon::path::Event::End { close, .. } => {
                    if close {
                        for BuildAndPaintOptions { builder, .. } in options.as_mut() {
                            builder.close()
                        }
                    }
                }
            }
        }

        for BuildAndPaintOptions { builder, background, .. } in options {
            let Ok(path) = builder.build() else { continue };
            window.paint_path(path, background);
        }
    }

    #[inline(always)]
    fn get_border_offset(&self, border_width: f32) -> f32 {
        match self.squircle_styles.border_mode.unwrap_or_default() {
            BorderMode::Outside => -border_width,
            BorderMode::Inside => border_width,
            BorderMode::Center => 0.
        }
    }
}

impl RenderOnce for Squircle {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        canvas(
            move |_, _, _| {},

            move |bounds, _, window, _| {
                /*if self.squircle_styles.corner_radius == Some(8.) {
                    println!("{}", bounds.size);
                }*/

                let squircle_styles = &self.squircle_styles;

                match (
                    squircle_styles.bg,
                    squircle_styles.border_width
                        .zip(self.squircle_styles.border_color)
                ) {
                    (Some(bg), None) => {
                        self.build_and_paint_paths(window, bounds, 0., [
                            BuildAndPaintOptions::fill(bg)
                        ]);
                    },

                    (Some(bg), Some((border_width, border_color))) => {
                        let border_offset = self.get_border_offset(border_width);

                        if border_offset == 0. {
                            // We can generate the same path for both the fill and the stroke.

                            self.build_and_paint_paths(window, bounds, 0., [
                                BuildAndPaintOptions::fill(bg),
                                BuildAndPaintOptions::stroke(border_color, border_width)
                            ]);

                        } else {
                            // We need to generate differen't paths for the fill and
                            // stroke as they have different corner radius's and size's.

                            self.build_and_paint_paths(window, bounds, 0., [
                                BuildAndPaintOptions::fill(bg),
                            ]);

                            self.build_and_paint_paths(window, bounds, border_offset, [
                                BuildAndPaintOptions::stroke(border_color, border_width)
                            ]);
                        }
                    },

                    (None, None) => (),

                    (None, Some((border_width, border_color))) => {
                        self.build_and_paint_paths(
                            window, bounds, self.get_border_offset(border_width),
                            [ BuildAndPaintOptions::stroke(border_color, border_width) ]   
                        );
                    },
                };
            },
        )
            .size_full()
            .absolute()
            .top_0()
            .bottom_0()
            .left_0()
            .right_0()
    }
}

pub fn squircle() -> Squircle {
    Squircle::new()
}