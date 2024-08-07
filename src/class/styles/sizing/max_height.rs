use crate::class::ApplyClass;
use bevy::ui::{Style, Val};

pub const MAX_H_AUTO: MaxHeight = MaxHeight(Val::Auto);
pub const MAX_H_FULL: MaxHeight = MaxHeight(Val::Percent(100.0));
pub const MAX_H_SCREEN: MaxHeight = MaxHeight(Val::Vh(100.0));
pub const MAX_H_SVW: MaxHeight = MaxHeight(Val::VMin(100.0));
pub const MAX_H_LVW: MaxHeight = MaxHeight(Val::VMax(100.0));

pub fn max_h(px: f32) -> MaxHeight {
    MaxHeight(Val::Px(px))
}

pub fn max_h_vh(percent: f32) -> MaxHeight {
    MaxHeight(Val::Vh(percent))
}

pub fn max_h_vmin(percent: f32) -> MaxHeight {
    MaxHeight(Val::VMin(percent))
}

pub fn max_h_vmax(percent: f32) -> MaxHeight {
    MaxHeight(Val::VMax(percent))
}

pub fn max_h_div(n: u32, d: u32) -> MaxHeight {
    MaxHeight(Val::Percent((n as f32 / d as f32) * 100.0))
}

pub fn max_h_fract(percent: f32) -> MaxHeight {
    MaxHeight(Val::Percent(percent))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaxHeight(pub Val);

impl ApplyClass<MaxHeight> for Style {
    fn apply_class(&mut self, class: &MaxHeight) {
        self.height = class.0;
    }
}
