use crate::class::ApplyClass;
use bevy::ui::{Style, Val};

pub const W_AUTO: Width = Width(Val::Auto);
pub const W_FULL: Width = Width(Val::Percent(100.0));
pub const W_SCREEN: Width = Width(Val::Vw(100.0));
pub const W_SVW: Width = Width(Val::VMin(100.0));
pub const W_LVW: Width = Width(Val::VMax(100.0));

pub fn w(px: f32) -> Width {
    Width(Val::Px(px))
}

pub fn w_vw(percent: f32) -> Width {
    Width(Val::Vw(percent))
}

pub fn w_vmin(percent: f32) -> Width {
    Width(Val::VMin(percent))
}

pub fn w_vmax(percent: f32) -> Width {
    Width(Val::VMax(percent))
}

pub fn w_div(n: u32, d: u32) -> Width {
    Width(Val::Percent((n as f32 / d as f32) * 100.0))
}

pub fn w_fract(percent: f32) -> Width {
    Width(Val::Percent(percent))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Width(pub Val);

impl ApplyClass<Width> for Style {
    fn apply_class(&mut self, class: &Width) {
        self.width = class.0;
    }
}
