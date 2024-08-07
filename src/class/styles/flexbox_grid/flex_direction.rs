use crate::class::ApplyClass;
use bevy_ui::Style;

pub use bevy_ui::FlexDirection;

pub const FLEX_ROW: FlexDirection = FlexDirection::Row;
pub const FLEX_COL: FlexDirection = FlexDirection::Column;
pub const FLEX_ROW_REVERSE: FlexDirection = FlexDirection::RowReverse;
pub const FLEX_COL_REVERSE: FlexDirection = FlexDirection::ColumnReverse;

impl ApplyClass<FlexDirection> for Style {
    fn apply_class(&mut self, class: &FlexDirection) {
        self.flex_direction = *class;
    }
}
