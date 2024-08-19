use crate::class::ApplyClass;
use bevy_ui::{Style, Val};

pub const H_AUTO: Height = Height(Val::Auto);
pub const H_FULL: Height = Height(Val::Percent(100.0));
pub const H_SCREEN: Height = Height(Val::Vh(100.0));
pub const H_SVW: Height = Height(Val::VMin(100.0));
pub const H_LVW: Height = Height(Val::VMax(100.0));

pub fn h(px: f32) -> Height {
    Height(Val::Px(px))
}

pub fn h_vh(fraction: f32) -> Height {
    Height(Val::Vh(fraction * 100.))
}

pub fn h_vmin(fraction: f32) -> Height {
    Height(Val::VMin(fraction * 100.))
}

pub fn h_vmax(fraction: f32) -> Height {
    Height(Val::VMax(fraction * 100.))
}

pub fn h_div(n: u32, d: u32) -> Height {
    Height(Val::Percent((n as f32 / d as f32) * 100.0))
}

pub fn h_fract(fract: f32) -> Height {
    Height(Val::Percent(fract * 100.))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Height(pub Val);

impl ApplyClass<Height> for Style {
    fn apply_class(&mut self, class: &Height) {
        self.height = class.0;
    }
}
