use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{FlexDirection, Style};

pub const FLEX_ROW: StyleClass = StyleClass::FlexDirection(FlexDirection::Row);
pub const FLEX_COL: StyleClass = StyleClass::FlexDirection(FlexDirection::Column);
pub const FLEX_ROW_REVERSE: StyleClass = StyleClass::FlexDirection(FlexDirection::RowReverse);
pub const FLEX_COL_REVERSE: StyleClass = StyleClass::FlexDirection(FlexDirection::ColumnReverse);

impl ApplyClass<FlexDirection> for Style {
    fn apply_class(&mut self, class: &FlexDirection) {
        self.flex_direction = *class;
    }
}
