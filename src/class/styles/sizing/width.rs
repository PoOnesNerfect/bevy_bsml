use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{Interaction, Style, Val};

pub const W_AUTO: StyleClass = StyleClass::Width(Width(Val::Auto));
pub const W_FULL: StyleClass = StyleClass::Width(Width(Val::Percent(100.0)));
pub const W_SCREEN: StyleClass = StyleClass::Width(Width(Val::Vw(100.0)));
pub const W_SVW: StyleClass = StyleClass::Width(Width(Val::VMin(100.0)));
pub const W_LVW: StyleClass = StyleClass::Width(Width(Val::VMax(100.0)));

pub fn w_px(px: f32) -> StyleClass {
    StyleClass::Width(Width(Val::Px(px)))
}

pub fn w_vw(percent: f32) -> StyleClass {
    StyleClass::Width(Width(Val::Vw(percent)))
}

pub fn w_vmin(percent: f32) -> StyleClass {
    StyleClass::Width(Width(Val::VMin(percent)))
}

pub fn w_vmax(percent: f32) -> StyleClass {
    StyleClass::Width(Width(Val::VMax(percent)))
}

pub fn w_div(n: u32, d: u32) -> StyleClass {
    StyleClass::Width(Width(Val::Percent((n as f32 / d as f32) * 100.0)))
}

pub fn w_perc(percent: f32) -> StyleClass {
    StyleClass::Width(Width(Val::Percent(percent)))
}

#[derive(Debug, Clone, Copy)]
pub struct Width(pub Val);

impl ApplyClass for Width {
    type Component = Style;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.width = self.0;
    }
}
