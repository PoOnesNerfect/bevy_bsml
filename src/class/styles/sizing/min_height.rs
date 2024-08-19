use crate::class::ApplyClass;
use bevy_ui::{Style, Val};

pub const MIN_H_AUTO: MinHeight = MinHeight(Val::Auto);
pub const MIN_H_FULL: MinHeight = MinHeight(Val::Percent(100.0));
pub const MIN_H_SCREEN: MinHeight = MinHeight(Val::Vh(100.0));
pub const MIN_H_SVW: MinHeight = MinHeight(Val::VMin(100.0));
pub const MIN_H_LVW: MinHeight = MinHeight(Val::VMax(100.0));

pub fn min_h(px: f32) -> MinHeight {
    MinHeight(Val::Px(px))
}

pub fn min_h_vh(fraction: f32) -> MinHeight {
    MinHeight(Val::Vh(fraction * 100.))
}

pub fn min_h_vmin(fraction: f32) -> MinHeight {
    MinHeight(Val::VMin(fraction * 100.))
}

pub fn min_h_vmax(fraction: f32) -> MinHeight {
    MinHeight(Val::VMax(fraction * 100.))
}

pub fn min_h_div(n: u32, d: u32) -> MinHeight {
    MinHeight(Val::Percent((n as f32 / d as f32) * 100.0))
}

pub fn min_h_fract(fract: f32) -> MinHeight {
    MinHeight(Val::Percent(fract * 100.))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinHeight(pub Val);

impl ApplyClass<MinHeight> for Style {
    fn apply_class(&mut self, class: &MinHeight) {
        self.height = class.0;
    }
}
