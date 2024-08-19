use crate::class::ApplyClass;
use bevy_ui::{Style, Val};

pub const MAX_H_AUTO: MaxHeight = MaxHeight(Val::Auto);
pub const MAX_H_FULL: MaxHeight = MaxHeight(Val::Percent(100.0));
pub const MAX_H_SCREEN: MaxHeight = MaxHeight(Val::Vh(100.0));
pub const MAX_H_SVW: MaxHeight = MaxHeight(Val::VMin(100.0));
pub const MAX_H_LVW: MaxHeight = MaxHeight(Val::VMax(100.0));

pub fn max_h(px: f32) -> MaxHeight {
    MaxHeight(Val::Px(px))
}

pub fn max_h_vh(fraction: f32) -> MaxHeight {
    MaxHeight(Val::Vh(fraction * 100.))
}

pub fn max_h_vmin(fraction: f32) -> MaxHeight {
    MaxHeight(Val::VMin(fraction * 100.))
}

pub fn max_h_vmax(fraction: f32) -> MaxHeight {
    MaxHeight(Val::VMax(fraction * 100.))
}

pub fn max_h_div(n: u32, d: u32) -> MaxHeight {
    MaxHeight(Val::Percent((n as f32 / d as f32) * 100.0))
}

pub fn max_h_fract(fract: f32) -> MaxHeight {
    MaxHeight(Val::Percent(fract * 100.))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaxHeight(pub Val);

impl ApplyClass<MaxHeight> for Style {
    fn apply_class(&mut self, class: &MaxHeight) {
        self.height = class.0;
    }
}
