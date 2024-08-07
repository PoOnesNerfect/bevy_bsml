use crate::class::ApplyClass;
use bevy::ui::{Style, Val};

pub const MAX_W_AUTO: MaxWidth = MaxWidth(Val::Auto);
pub const MAX_W_FULL: MaxWidth = MaxWidth(Val::Percent(100.0));
pub const MAX_W_SCREEN: MaxWidth = MaxWidth(Val::Vw(100.0));
pub const MAX_W_SVW: MaxWidth = MaxWidth(Val::VMin(100.0));
pub const MAX_W_LVW: MaxWidth = MaxWidth(Val::VMax(100.0));

pub fn max_w(px: f32) -> MaxWidth {
    MaxWidth(Val::Px(px))
}

pub fn max_w_vw(percent: f32) -> MaxWidth {
    MaxWidth(Val::Vw(percent))
}

pub fn max_w_vmin(percent: f32) -> MaxWidth {
    MaxWidth(Val::VMin(percent))
}

pub fn max_w_vmax(percent: f32) -> MaxWidth {
    MaxWidth(Val::VMax(percent))
}

pub fn max_w_div(n: u32, d: u32) -> MaxWidth {
    MaxWidth(Val::Percent((n as f32 / d as f32) * 100.0))
}

pub fn max_w_fract(percent: f32) -> MaxWidth {
    MaxWidth(Val::Percent(percent))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MaxWidth(pub Val);

impl ApplyClass<MaxWidth> for Style {
    fn apply_class(&mut self, class: &MaxWidth) {
        self.max_width = class.0;
    }
}
