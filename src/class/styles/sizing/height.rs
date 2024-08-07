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

pub fn h_vh(percent: f32) -> Height {
    Height(Val::Vh(percent))
}

pub fn h_vmin(percent: f32) -> Height {
    Height(Val::VMin(percent))
}

pub fn h_vmax(percent: f32) -> Height {
    Height(Val::VMax(percent))
}

pub fn h_div(n: u32, d: u32) -> Height {
    Height(Val::Percent((n as f32 / d as f32) * 100.0))
}

pub fn h_frac(percent: f32) -> Height {
    Height(Val::Percent(percent))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Height(pub Val);

impl ApplyClass<Height> for Style {
    fn apply_class(&mut self, class: &Height) {
        self.height = class.0;
    }
}
