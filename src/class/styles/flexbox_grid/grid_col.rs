use bevy_ui::{GridPlacement, Style};
use derive_more::From;

use crate::class::ApplyClass;

#[derive(Debug, Clone, From, PartialEq)]
pub struct GridCol(pub GridPlacement);

impl ApplyClass<GridCol> for Style {
    fn apply_class(&mut self, class: &GridCol) {
        self.grid_column = class.0;
    }
}
