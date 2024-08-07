use crate::class::ApplyClass;
use bevy_ui::Style;

pub use bevy_ui::FlexWrap;

pub const FLEX_WRAP: FlexWrap = FlexWrap::Wrap;
pub const FLEX_WRAP_REVERSE: FlexWrap = FlexWrap::WrapReverse;
pub const FLEX_NOWRAP: FlexWrap = FlexWrap::NoWrap;

impl ApplyClass<FlexWrap> for Style {
    fn apply_class(&mut self, class: &FlexWrap) {
        self.flex_wrap = *class;
    }
}
