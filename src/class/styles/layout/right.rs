use crate::class::ApplyClass;
use bevy_ui::{Style, Val};

pub const RIGHT_0: Right = Right(Val::Px(0.0));
pub const RIGHT_AUTO: Right = Right(Val::Auto);
pub const RIGHT_FULL: Right = Right(Val::Percent(100.0));

pub fn right(px: f32) -> Right {
    Right(Val::Px(px))
}

pub fn right_vw(percent: f32) -> Right {
    Right(Val::Vw(percent))
}

pub fn right_vmin(percent: f32) -> Right {
    Right(Val::VMin(percent))
}

pub fn right_vmax(percent: f32) -> Right {
    Right(Val::VMax(percent))
}

pub fn right_div(n: u32, d: u32) -> Right {
    Right(Val::Percent((n as f32 / d as f32) * 100.0))
}

pub fn right_fract(percent: f32) -> Right {
    Right(Val::Percent(percent))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Right(pub Val);

impl ApplyClass<Right> for Style {
    fn apply_class(&mut self, class: &Right) {
        self.right = class.0;
    }
}
