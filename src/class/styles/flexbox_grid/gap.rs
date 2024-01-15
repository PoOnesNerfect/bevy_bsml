use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{Interaction, Style, Val};

pub fn gap(px: f32) -> StyleClass {
    StyleClass::Gap(Gap(Val::Px(px)))
}

pub fn gap_x(px: f32) -> StyleClass {
    StyleClass::ColGap(ColGap(Val::Px(px)))
}

pub fn gap_y(px: f32) -> StyleClass {
    StyleClass::RowGap(RowGap(Val::Px(px)))
}

#[derive(Debug, Clone)]
pub struct Gap(Val);

impl ApplyClass for Gap {
    type Component = Style;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.row_gap = self.0;
        component.column_gap = self.0;
    }
}

#[derive(Debug, Clone)]
pub struct RowGap(Val);

impl ApplyClass for RowGap {
    type Component = Style;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.row_gap = self.0;
    }
}

#[derive(Debug, Clone)]
pub struct ColGap(Val);

impl ApplyClass for ColGap {
    type Component = Style;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.column_gap = self.0;
    }
}
