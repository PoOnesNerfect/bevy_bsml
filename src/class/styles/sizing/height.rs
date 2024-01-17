use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{Style, Val};

pub const H_AUTO: StyleClass = StyleClass::Height(Height(Val::Auto));
pub const H_FULL: StyleClass = StyleClass::Height(Height(Val::Percent(100.0)));
pub const H_SCREEN: StyleClass = StyleClass::Height(Height(Val::Vh(100.0)));
pub const H_SVW: StyleClass = StyleClass::Height(Height(Val::VMin(100.0)));
pub const H_LVW: StyleClass = StyleClass::Height(Height(Val::VMax(100.0)));

pub fn h_px(px: f32) -> StyleClass {
    StyleClass::Height(Height(Val::Px(px)))
}

pub fn h_vh(percent: f32) -> StyleClass {
    StyleClass::Height(Height(Val::Vh(percent)))
}

pub fn h_vmin(percent: f32) -> StyleClass {
    StyleClass::Height(Height(Val::VMin(percent)))
}

pub fn h_vmax(percent: f32) -> StyleClass {
    StyleClass::Height(Height(Val::VMax(percent)))
}

pub fn h_div(n: u32, d: u32) -> StyleClass {
    StyleClass::Height(Height(Val::Percent((n as f32 / d as f32) * 100.0)))
}

pub fn h_perc(percent: f32) -> StyleClass {
    StyleClass::Height(Height(Val::Percent(percent)))
}

#[derive(Debug, Clone, Copy)]
pub struct Height(pub Val);

impl ApplyClass for Height {
    type Component = Style;

    fn apply_class(&self, component: &mut Self::Component) {
        component.height = self.0;
    }
}
