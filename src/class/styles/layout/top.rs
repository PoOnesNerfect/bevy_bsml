use crate::class::ApplyClass;
use bevy_ui::{Style, Val};

pub const TOP_0: Top = Top(Val::Px(0.0));
pub const TOP_AUTO: Top = Top(Val::Auto);
pub const TOP_FULL: Top = Top(Val::Percent(100.0));

pub fn top(px: f32) -> Top {
    Top(Val::Px(px))
}

pub fn top_vw(percent: f32) -> Top {
    Top(Val::Vw(percent))
}

pub fn top_vmin(percent: f32) -> Top {
    Top(Val::VMin(percent))
}

pub fn top_vmax(percent: f32) -> Top {
    Top(Val::VMax(percent))
}

pub fn top_div(n: u32, d: u32) -> Top {
    Top(Val::Percent((n as f32 / d as f32) * 100.0))
}

pub fn top_fract(percent: f32) -> Top {
    Top(Val::Percent(percent))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Top(pub Val);

impl ApplyClass<Top> for Style {
    fn apply_class(&mut self, class: &Top) {
        self.top = class.0;
    }
}
