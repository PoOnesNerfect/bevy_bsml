use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{Style, Val};

pub const MIN_W_AUTO: StyleClass = StyleClass::MinWidth(MinWidth(Val::Auto));
pub const MIN_W_FULL: StyleClass = StyleClass::MinWidth(MinWidth(Val::Percent(100.0)));
pub const MIN_W_SCREEN: StyleClass = StyleClass::MinWidth(MinWidth(Val::Vw(100.0)));
pub const MIN_W_SVW: StyleClass = StyleClass::MinWidth(MinWidth(Val::VMin(100.0)));
pub const MIN_W_LVW: StyleClass = StyleClass::MinWidth(MinWidth(Val::VMax(100.0)));

pub fn min_w_px(px: f32) -> StyleClass {
    StyleClass::MinWidth(MinWidth(Val::Px(px)))
}

pub fn min_w_vw(percent: f32) -> StyleClass {
    StyleClass::MinWidth(MinWidth(Val::Vw(percent)))
}

pub fn min_w_vmin(percent: f32) -> StyleClass {
    StyleClass::MinWidth(MinWidth(Val::VMin(percent)))
}

pub fn min_w_vmax(percent: f32) -> StyleClass {
    StyleClass::MinWidth(MinWidth(Val::VMax(percent)))
}

pub fn min_w_div(n: u32, d: u32) -> StyleClass {
    StyleClass::MinWidth(MinWidth(Val::Percent((n as f32 / d as f32) * 100.0)))
}

pub fn min_w_perc(percent: f32) -> StyleClass {
    StyleClass::MinWidth(MinWidth(Val::Percent(percent)))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct MinWidth(pub Val);

impl ApplyClass<MinWidth> for Style {
    fn apply_class(&mut self, class: &MinWidth) {
        self.min_width = class.0;
    }
}
