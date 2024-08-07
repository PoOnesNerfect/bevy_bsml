use crate::class::ApplyClass;
use bevy_ui::Style;

pub use bevy_ui::PositionType;

pub const ABSOLUTE: PositionType = PositionType::Absolute;
pub const RELATIVE: PositionType = PositionType::Relative;

impl ApplyClass<PositionType> for Style {
    fn apply_class(&mut self, class: &PositionType) {
        self.position_type = *class;
    }
}
