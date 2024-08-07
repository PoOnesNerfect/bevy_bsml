use crate::class::ApplyClass;
use bevy_ui::{Style, Val};

pub const LEFT_0: Left = Left(Val::Px(0.0));
pub const LEFT_AUTO: Left = Left(Val::Auto);
pub const LEFT_FULL: Left = Left(Val::Percent(100.0));

pub fn left(px: f32) -> Left {
    Left(Val::Px(px))
}

pub fn left_vw(percent: f32) -> Left {
    Left(Val::Vw(percent))
}

pub fn left_vmin(percent: f32) -> Left {
    Left(Val::VMin(percent))
}

pub fn left_vmax(percent: f32) -> Left {
    Left(Val::VMax(percent))
}

pub fn left_div(n: u32, d: u32) -> Left {
    Left(Val::Percent((n as f32 / d as f32) * 100.0))
}

pub fn left_fract(percent: f32) -> Left {
    Left(Val::Percent(percent))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Left(pub Val);

impl ApplyClass<Left> for Style {
    fn apply_class(&mut self, class: &Left) {
        self.left = class.0;
    }
}
