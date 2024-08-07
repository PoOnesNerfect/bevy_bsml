pub use bevy::ui::BorderRadius;
use bevy::ui::Val;

use super::ApplyClass;

pub const ROUNDED_NONE: BorderRadiusClass = BorderRadiusClass::All(Val::Px(0.));
pub const ROUNDED_FULL: BorderRadiusClass = BorderRadiusClass::All(Val::Px(f32::MAX));

pub const ROUNDED_T_NONE: BorderRadiusClass = BorderRadiusClass::Top(Val::Px(0.0));
pub const ROUNDED_T_FULL: BorderRadiusClass = BorderRadiusClass::Top(Val::Px(f32::MAX));
pub const ROUNDED_B_NONE: BorderRadiusClass = BorderRadiusClass::Bottom(Val::Px(0.0));
pub const ROUNDED_B_FULL: BorderRadiusClass = BorderRadiusClass::Bottom(Val::Px(f32::MAX));
pub const ROUNDED_R_NONE: BorderRadiusClass = BorderRadiusClass::Right(Val::Px(0.0));
pub const ROUNDED_R_FULL: BorderRadiusClass = BorderRadiusClass::Right(Val::Px(f32::MAX));
pub const ROUNDED_L_NONE: BorderRadiusClass = BorderRadiusClass::Left(Val::Px(0.0));
pub const ROUNDED_L_FULL: BorderRadiusClass = BorderRadiusClass::Left(Val::Px(f32::MAX));
pub const ROUNDED_TL_NONE: BorderRadiusClass = BorderRadiusClass::TopLeft(Val::Px(0.0));
pub const ROUNDED_TL_FULL: BorderRadiusClass = BorderRadiusClass::TopLeft(Val::Px(f32::MAX));
pub const ROUNDED_TR_NONE: BorderRadiusClass = BorderRadiusClass::TopRight(Val::Px(0.0));
pub const ROUNDED_TR_FULL: BorderRadiusClass = BorderRadiusClass::TopRight(Val::Px(f32::MAX));
pub const ROUNDED_BL_NONE: BorderRadiusClass = BorderRadiusClass::BottomLeft(Val::Px(0.0));
pub const ROUNDED_BL_FULL: BorderRadiusClass = BorderRadiusClass::BottomLeft(Val::Px(f32::MAX));
pub const ROUNDED_BR_NONE: BorderRadiusClass = BorderRadiusClass::BottomRight(Val::Px(0.0));
pub const ROUNDED_BR_FULL: BorderRadiusClass = BorderRadiusClass::BottomRight(Val::Px(f32::MAX));

pub fn rounded(px: f32) -> BorderRadiusClass {
    BorderRadiusClass::All(Val::Px(px))
}

pub fn rounded_fract(fract: f32) -> BorderRadiusClass {
    BorderRadiusClass::All(Val::Percent(fract))
}

pub fn rounded_t(px: f32) -> BorderRadiusClass {
    BorderRadiusClass::Top(Val::Px(px))
}

pub fn rounded_t_fract(fract: f32) -> BorderRadiusClass {
    BorderRadiusClass::Top(Val::Percent(fract))
}

pub fn rounded_b(px: f32) -> BorderRadiusClass {
    BorderRadiusClass::Bottom(Val::Px(px))
}

pub fn rounded_b_fract(fract: f32) -> BorderRadiusClass {
    BorderRadiusClass::Bottom(Val::Percent(fract))
}

pub fn rounded_r(px: f32) -> BorderRadiusClass {
    BorderRadiusClass::Right(Val::Px(px))
}

pub fn rounded_r_fract(fract: f32) -> BorderRadiusClass {
    BorderRadiusClass::Right(Val::Percent(fract))
}

pub fn rounded_l(px: f32) -> BorderRadiusClass {
    BorderRadiusClass::Left(Val::Px(px))
}

pub fn rounded_l_fract(fract: f32) -> BorderRadiusClass {
    BorderRadiusClass::Left(Val::Percent(fract))
}

pub fn rounded_tl(px: f32) -> BorderRadiusClass {
    BorderRadiusClass::TopLeft(Val::Px(px))
}

pub fn rounded_tl_fract(fract: f32) -> BorderRadiusClass {
    BorderRadiusClass::TopLeft(Val::Percent(fract))
}

pub fn rounded_tr(px: f32) -> BorderRadiusClass {
    BorderRadiusClass::TopRight(Val::Px(px))
}

pub fn rounded_tr_fract(fract: f32) -> BorderRadiusClass {
    BorderRadiusClass::TopRight(Val::Percent(fract))
}

pub fn rounded_bl(px: f32) -> BorderRadiusClass {
    BorderRadiusClass::BottomLeft(Val::Px(px))
}

pub fn rounded_bl_fract(fract: f32) -> BorderRadiusClass {
    BorderRadiusClass::BottomLeft(Val::Percent(fract))
}

pub fn rounded_br(px: f32) -> BorderRadiusClass {
    BorderRadiusClass::BottomRight(Val::Px(px))
}

pub fn rounded_br_fract(fract: f32) -> BorderRadiusClass {
    BorderRadiusClass::BottomRight(Val::Percent(fract))
}

#[derive(Clone, Debug, PartialEq)]
pub enum BorderRadiusClass {
    All(Val),
    Top(Val),
    Bottom(Val),
    Right(Val),
    Left(Val),
    TopLeft(Val),
    TopRight(Val),
    BottomLeft(Val),
    BottomRight(Val),
}

impl ApplyClass<BorderRadiusClass> for BorderRadius {
    fn apply_class(&mut self, class: &BorderRadiusClass) {
        println!("applying class: {:?}", class);
        match class {
            BorderRadiusClass::All(val) => *self = BorderRadius::all(val.clone()),
            BorderRadiusClass::Top(val) => {
                self.top_left = val.clone();
                self.top_right = val.clone();
            }
            BorderRadiusClass::Bottom(val) => {
                self.bottom_left = val.clone();
                self.bottom_right = val.clone();
            }
            BorderRadiusClass::Right(val) => {
                self.top_right = val.clone();
                self.bottom_right = val.clone();
            }
            BorderRadiusClass::Left(val) => {
                self.top_left = val.clone();
                self.bottom_left = val.clone();
            }
            BorderRadiusClass::TopLeft(val) => {
                self.top_left = val.clone();
            }
            BorderRadiusClass::TopRight(val) => {
                self.top_right = val.clone();
            }
            BorderRadiusClass::BottomLeft(val) => {
                self.bottom_left = val.clone();
            }
            BorderRadiusClass::BottomRight(val) => {
                self.bottom_right = val.clone();
            }
        }
    }
}
