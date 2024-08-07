use crate::class::ApplyClass;
use bevy_ui::{Style, Val};

pub const MIN_W_AUTO: MinWidth = MinWidth(Val::Auto);
pub const MIN_W_FULL: MinWidth = MinWidth(Val::Percent(100.0));
pub const MIN_W_SCREEN: MinWidth = MinWidth(Val::Vw(100.0));
pub const MIN_W_SVW: MinWidth = MinWidth(Val::VMin(100.0));
pub const MIN_W_LVW: MinWidth = MinWidth(Val::VMax(100.0));

pub fn min_w(px: f32) -> MinWidth {
    MinWidth(Val::Px(px))
}

pub fn min_w_vw(percent: f32) -> MinWidth {
    MinWidth(Val::Vw(percent))
}

pub fn min_w_vmin(percent: f32) -> MinWidth {
    MinWidth(Val::VMin(percent))
}

pub fn min_w_vmax(percent: f32) -> MinWidth {
    MinWidth(Val::VMax(percent))
}

pub fn min_w_div(n: u32, d: u32) -> MinWidth {
    MinWidth(Val::Percent((n as f32 / d as f32) * 100.0))
}

pub fn min_w_fract(percent: f32) -> MinWidth {
    MinWidth(Val::Percent(percent))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinWidth(pub Val);

impl ApplyClass<MinWidth> for Style {
    fn apply_class(&mut self, class: &MinWidth) {
        self.min_width = class.0;
    }
}
