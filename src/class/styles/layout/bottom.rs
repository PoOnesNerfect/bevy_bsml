use crate::class::ApplyClass;
use bevy_ui::{Style, Val};

pub const BOTTOM_0: Bottom = Bottom(Val::Px(0.0));
pub const BOTTOM_AUTO: Bottom = Bottom(Val::Auto);
pub const BOTTOM_FULL: Bottom = Bottom(Val::Percent(100.0));

pub fn bottom(px: f32) -> Bottom {
    Bottom(Val::Px(px))
}

pub fn bottom_vw(percent: f32) -> Bottom {
    Bottom(Val::Vw(percent))
}

pub fn bottom_vmin(percent: f32) -> Bottom {
    Bottom(Val::VMin(percent))
}

pub fn bottom_vmax(percent: f32) -> Bottom {
    Bottom(Val::VMax(percent))
}

pub fn bottom_div(n: u32, d: u32) -> Bottom {
    Bottom(Val::Percent((n as f32 / d as f32) * 100.0))
}

pub fn bottom_fract(percent: f32) -> Bottom {
    Bottom(Val::Percent(percent))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Bottom(pub Val);

impl ApplyClass<Bottom> for Style {
    fn apply_class(&mut self, class: &Bottom) {
        self.bottom = class.0;
    }
}
