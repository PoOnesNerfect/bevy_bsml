use crate::class::{ApplyClass, StyleClass};
use bevy::ui::Style;

#[derive(Debug, Clone, PartialEq)]
pub struct AspectRatio(pub Option<f32>);

impl AspectRatio {
    pub fn fract(num: f32, den: f32) -> StyleClass {
        StyleClass::AspectRatio(Self(Some(num / den)))
    }
}

impl ApplyClass<AspectRatio> for Style {
    fn apply_class(&mut self, class: &AspectRatio) {
        self.aspect_ratio = class.0;
    }
}

pub const ASPECT_AUTO: StyleClass = StyleClass::AspectRatio(AspectRatio(None));
pub const ASPECT_SQUARE: StyleClass = StyleClass::AspectRatio(AspectRatio(Some(1.0)));
pub const ASPECT_VIDEO: StyleClass = StyleClass::AspectRatio(AspectRatio(Some(16.0 / 9.0)));
