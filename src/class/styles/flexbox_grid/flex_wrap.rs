use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{FlexWrap, Style};

pub const FLEX_WRAP: StyleClass = StyleClass::FlexWrap(FlexWrap::Wrap);
pub const FLEX_WRAP_REVERSE: StyleClass = StyleClass::FlexWrap(FlexWrap::WrapReverse);
pub const FLEX_NOWRAP: StyleClass = StyleClass::FlexWrap(FlexWrap::NoWrap);

impl ApplyClass<FlexWrap> for Style {
    fn apply_class(&mut self, class: &FlexWrap) {
        self.flex_wrap = *class;
    }
}
