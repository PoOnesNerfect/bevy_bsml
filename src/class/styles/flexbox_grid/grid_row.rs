use bevy::ui::{GridPlacement, Style};
use derive_more::From;

use crate::class::ApplyClass;

#[derive(Debug, Clone, From, PartialEq)]
pub struct GridRow(pub GridPlacement);

impl ApplyClass<GridRow> for Style {
    fn apply_class(&mut self, class: &GridRow) {
        self.grid_row = class.0;
    }
}
