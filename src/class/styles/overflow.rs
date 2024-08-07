use crate::class::ApplyClass;
use bevy::ui::{OverflowAxis, Style};

pub use bevy::ui::Overflow;

pub const OVERFLOW_VISIBLE: Overflow = Overflow {
    x: OverflowAxis::Visible,
    y: OverflowAxis::Visible,
};

pub const OVERFLOW_CLIP: Overflow = Overflow {
    x: OverflowAxis::Clip,
    y: OverflowAxis::Clip,
};

pub const OVERFLOW_HIDDEN: Overflow = Overflow {
    x: OverflowAxis::Hidden,
    y: OverflowAxis::Hidden,
};

pub const OVERFLOW_X_VISIBLE: OverflowX = OverflowX(OverflowAxis::Visible);
pub const OVERFLOW_X_CLIP: OverflowX = OverflowX(OverflowAxis::Clip);
pub const OVERFLOW_X_HIDDEN: OverflowX = OverflowX(OverflowAxis::Hidden);

pub const OVERFLOW_Y_VISIBLE: OverflowY = OverflowY(OverflowAxis::Visible);
pub const OVERFLOW_Y_CLIP: OverflowY = OverflowY(OverflowAxis::Clip);
pub const OVERFLOW_Y_HIDDEN: OverflowY = OverflowY(OverflowAxis::Hidden);

#[derive(Debug, Clone, PartialEq)]
pub struct OverflowX(pub OverflowAxis);

#[derive(Debug, Clone, PartialEq)]
pub struct OverflowY(pub OverflowAxis);

impl ApplyClass<Overflow> for Style {
    #[inline]
    fn apply_class(&mut self, class: &Overflow) {
        self.overflow = *class;
    }
}

impl ApplyClass<OverflowX> for Style {
    #[inline]
    fn apply_class(&mut self, class: &OverflowX) {
        self.overflow.x = class.0;
    }
}

impl ApplyClass<OverflowY> for Style {
    #[inline]
    fn apply_class(&mut self, class: &OverflowY) {
        self.overflow.y = class.0;
    }
}
