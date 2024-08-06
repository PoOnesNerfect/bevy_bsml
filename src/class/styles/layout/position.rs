use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{PositionType, Style};

pub const ABSOLUTE: StyleClass = StyleClass::PositionType(PositionType::Absolute);
pub const RELATIVE: StyleClass = StyleClass::PositionType(PositionType::Relative);

impl ApplyClass<PositionType> for Style {
    fn apply_class(&mut self, class: &PositionType) {
        self.position_type = *class;
    }
}
