use crate::class::ApplyClass;
use bevy::ui::{Style, Val};

pub fn gap(px: f32) -> Gap {
    Gap(Val::Px(px))
}

pub fn gap_x(px: f32) -> ColGap {
    ColGap(Val::Px(px))
}

pub fn gap_y(px: f32) -> RowGap {
    RowGap(Val::Px(px))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Gap(Val);

impl ApplyClass<Gap> for Style {
    fn apply_class(&mut self, class: &Gap) {
        self.row_gap = class.0;
        self.column_gap = class.0;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RowGap(Val);

impl ApplyClass<RowGap> for Style {
    fn apply_class(&mut self, class: &RowGap) {
        self.row_gap = class.0;
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ColGap(Val);

impl ApplyClass<ColGap> for Style {
    fn apply_class(&mut self, class: &ColGap) {
        self.column_gap = class.0;
    }
}
