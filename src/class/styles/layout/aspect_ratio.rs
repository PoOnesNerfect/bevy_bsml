use crate::class::ApplyClass;
use bevy_ui::Style;

#[derive(Debug, Clone, PartialEq)]
pub struct AspectRatio(pub Option<f32>);

impl AspectRatio {
    pub fn fract(num: f32, den: f32) -> AspectRatio {
        Self(Some(num / den))
    }
}

impl ApplyClass<AspectRatio> for Style {
    fn apply_class(&mut self, class: &AspectRatio) {
        self.aspect_ratio = class.0;
    }
}

pub const ASPECT_AUTO: AspectRatio = AspectRatio(None);
pub const ASPECT_SQUARE: AspectRatio = AspectRatio(Some(1.0));
pub const ASPECT_VIDEO: AspectRatio = AspectRatio(Some(16.0 / 9.0));
