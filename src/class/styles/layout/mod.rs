use super::ApplyClass;
use bevy::ui::{Interaction, Style};

#[derive(Debug, Clone)]
pub struct AspectRatio(pub Option<f32>);

impl AspectRatio {
    pub fn fract(num: f32, den: f32) -> Self {
        Self(Some(num / den))
    }
}

impl ApplyClass for AspectRatio {
    type Component = Style;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.aspect_ratio = self.0;
    }
}

pub const ASPECT_AUTO: AspectRatio = AspectRatio(None);
pub const ASPECT_SQUARE: AspectRatio = AspectRatio(Some(1.0));
pub const ASPECT_VIDEO: AspectRatio = AspectRatio(Some(16.0 / 9.0));
