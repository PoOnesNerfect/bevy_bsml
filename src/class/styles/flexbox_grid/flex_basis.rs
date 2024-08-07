use bevy_ui::Val;

use crate::class::ApplyClass;

pub const BASIS_AUTO: FlexBasis = FlexBasis(Val::Auto);
pub const BASIS_FULL: FlexBasis = FlexBasis(Val::Percent(100.0));

pub fn basis(px: f32) -> FlexBasis {
    FlexBasis(Val::Px(px))
}

pub fn basis_fract(fraction: f32) -> FlexBasis {
    FlexBasis(Val::Percent(fraction))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlexBasis(pub Val);

impl ApplyClass<FlexBasis> for bevy_ui::Style {
    fn apply_class(&mut self, class: &FlexBasis) {
        self.flex_basis = class.0;
    }
}
