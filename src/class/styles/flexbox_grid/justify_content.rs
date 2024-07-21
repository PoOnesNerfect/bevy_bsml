use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{JustifyContent, Style};

pub const JUSTIFY_CENTER: StyleClass = StyleClass::JustifyContent(JustifyContent::Center);
pub const JUSTIFY_DEFAULT: StyleClass = StyleClass::JustifyContent(JustifyContent::Default);
pub const JUSTIFY_END: StyleClass = StyleClass::JustifyContent(JustifyContent::End);
pub const JUSTIFY_FLEX_END: StyleClass = StyleClass::JustifyContent(JustifyContent::FlexEnd);
pub const JUSTIFY_FLEX_START: StyleClass = StyleClass::JustifyContent(JustifyContent::FlexStart);
pub const JUSTIFY_SPACE_AROUND: StyleClass =
    StyleClass::JustifyContent(JustifyContent::SpaceAround);
pub const JUSTIFY_SPACE_BETWEEN: StyleClass =
    StyleClass::JustifyContent(JustifyContent::SpaceBetween);
pub const JUSTIFY_SPACE_EVENLY: StyleClass =
    StyleClass::JustifyContent(JustifyContent::SpaceEvenly);
pub const JUSTIFY_START: StyleClass = StyleClass::JustifyContent(JustifyContent::Start);
pub const JUSTIFY_STRETCH: StyleClass = StyleClass::JustifyContent(JustifyContent::Stretch);

impl ApplyClass for JustifyContent {
    type Component = Style;

    fn apply_class(&self, component: &mut Self::Component) {
        component.justify_content = *self;
    }
}
