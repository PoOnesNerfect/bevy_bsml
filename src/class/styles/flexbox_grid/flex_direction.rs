use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{FlexDirection, Interaction, Style};

pub const FLEX_ROW: StyleClass = StyleClass::FlexDirection(FlexDirection::Row);
pub const FLEX_COL: StyleClass = StyleClass::FlexDirection(FlexDirection::Column);
pub const FLEX_ROW_REVERSE: StyleClass = StyleClass::FlexDirection(FlexDirection::RowReverse);
pub const FLEX_COL_REVERSE: StyleClass = StyleClass::FlexDirection(FlexDirection::ColumnReverse);

impl ApplyClass for FlexDirection {
    type Component = Style;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.flex_direction = *self;
    }
}
