use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{FlexWrap, Interaction, Style};

pub const FLEX_WRAP: StyleClass = StyleClass::FlexWrap(FlexWrap::Wrap);
pub const FLEX_WRAP_REVERSE: StyleClass = StyleClass::FlexWrap(FlexWrap::WrapReverse);
pub const FLEX_NOWRAP: StyleClass = StyleClass::FlexWrap(FlexWrap::NoWrap);

impl ApplyClass for FlexWrap {
    type Component = Style;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.flex_wrap = *self;
    }
}
