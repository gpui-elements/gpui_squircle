use gpui::{Background, Pixels};

use crate::Squircle;

#[derive(Default, Clone, Copy)]
pub enum BorderMode {
    #[default]
    Center,
    Outside,
    Inside
}

#[derive(Default, Clone, Copy)]
pub struct SquircleStyles {
    pub corner_radius: Option<f32>,
    pub top_left_corner_radius: Option<f32>,
    pub top_right_corner_radius: Option<f32>,
    pub bottom_right_corner_radius: Option<f32>,
    pub bottom_left_corner_radius: Option<f32>,
    pub corner_smoothing: Option<f32>,
    pub preserve_smoothing: Option<bool>,
    pub bg: Option<Background>,
    pub border_width: Option<f32>,
    pub border_color: Option<Background>,
    pub border_mode: Option<BorderMode>
}

impl SquircleStyles {
    pub fn apply(&self, mut element: Squircle) -> Squircle {
        if let Some(corner_radius) = self.corner_radius {
            element.squircle_styles.corner_radius = Some(corner_radius);
        }

        if let Some(top_left_corner_radius) = self.top_left_corner_radius {
            element.squircle_styles.top_left_corner_radius = Some(top_left_corner_radius);
        }

        if let Some(top_right_corner_radius) = self.top_right_corner_radius {
            element.squircle_styles.top_right_corner_radius = Some(top_right_corner_radius);
        }

        if let Some(bottom_right_corner_radius) = self.bottom_right_corner_radius {
            element.squircle_styles.bottom_right_corner_radius = Some(bottom_right_corner_radius);
        }

        if let Some(bottom_left_corner_radius) = self.bottom_left_corner_radius {
            element.squircle_styles.bottom_left_corner_radius = Some(bottom_left_corner_radius);
        }

        if let Some(corner_smoothing) = self.corner_smoothing {
            element.squircle_styles.corner_smoothing = Some(corner_smoothing);
        }

        if let Some(preserve_smoothing) = self.preserve_smoothing {
            element.squircle_styles.preserve_smoothing = Some(preserve_smoothing);
        }

        if let Some(bg) = self.bg {
            element.squircle_styles.bg = Some(bg);
        }

        if let Some(border_width) = self.border_width {
            element.squircle_styles.border_width = Some(border_width);
        }

        if let Some(border_color) = self.border_color {
            element.squircle_styles.border_color = Some(border_color);
        }

        if let Some(border_mode) = self.border_mode {
            element.squircle_styles.border_mode = Some(border_mode);
        }

        element
    }
}

pub trait SquircleStylable where Self: Sized {
    fn get_squircle_styles_mut(&mut self) -> &mut SquircleStyles;

    fn rounded_smoothing(mut self, smoothing: f32) -> Self {
        self.get_squircle_styles_mut().corner_smoothing = Some(smoothing.clamp(0., 1.));
        self
    }

    // TODO: support other than pixel.
    fn rounded(mut self, length: Pixels) -> Self {
        self.get_squircle_styles_mut().corner_radius = Some(length.to_f64() as f32);
        self
    }

    // TODO: support other than pixel.
    fn rounded_tl(mut self, length: Pixels) -> Self {
        self.get_squircle_styles_mut().top_left_corner_radius = Some(length.to_f64() as f32);
        self
    }

    // TODO: support other than pixel.
    fn rounded_tr(mut self, length: Pixels) -> Self {
        self.get_squircle_styles_mut().top_right_corner_radius = Some(length.to_f64() as f32);
        self
    }

    // TODO: support other than pixel.
    fn rounded_bl(mut self, length: Pixels) -> Self {
        self.get_squircle_styles_mut().bottom_left_corner_radius = Some(length.to_f64() as f32);
        self
    }

    // TODO: support other than pixel.
    fn rounded_br(mut self, length: Pixels) -> Self {
        self.get_squircle_styles_mut().bottom_right_corner_radius = Some(length.to_f64() as f32);
        self
    }

    fn bg<F>(mut self, f: F) -> Self
    where
        F: Into<Background>,
        Self: Sized,
    {
        self.get_squircle_styles_mut().bg = Some(f.into());
        self
    }

    // TODO: support other than pixel.
    fn border(mut self, length: Pixels) -> Self {
        self.get_squircle_styles_mut().border_width = Some(length.to_f64() as f32);
        self
    }

    fn border_color<F>(mut self, f: F) -> Self
    where
        F: Into<Background>,
        Self: Sized,
    {
        self.get_squircle_styles_mut().border_color = Some(f.into());
        self
    }

    fn border_outside(mut self) -> Self {
        self.get_squircle_styles_mut().border_mode = Some(BorderMode::Outside);
        self
    }

    fn border_inside(mut self) -> Self {
        self.get_squircle_styles_mut().border_mode = Some(BorderMode::Inside);
        self
    }

    fn border_center(mut self) -> Self {
        self.get_squircle_styles_mut().border_mode = Some(BorderMode::Center);
        self
    }

}