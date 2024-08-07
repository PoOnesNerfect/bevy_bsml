use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::Style;

pub use bevy::ui::JustifyContent;

pub const JUSTIFY_CENTER: StyleClass = StyleClass::JustifyContent(JustifyContent::Center);
pub const JUSTIFY_DEFAULT: StyleClass = StyleClass::JustifyContent(JustifyContent::Default);
pub const JUSTIFY_END: StyleClass = StyleClass::JustifyContent(JustifyContent::FlexEnd);
pub const JUSTIFY_START: StyleClass = StyleClass::JustifyContent(JustifyContent::FlexStart);
pub const JUSTIFY_AROUND: StyleClass = StyleClass::JustifyContent(JustifyContent::SpaceAround);
pub const JUSTIFY_BETWEEN: StyleClass = StyleClass::JustifyContent(JustifyContent::SpaceBetween);
pub const JUSTIFY_EVENLY: StyleClass = StyleClass::JustifyContent(JustifyContent::SpaceEvenly);
pub const JUSTIFY_STRETCH: StyleClass = StyleClass::JustifyContent(JustifyContent::Stretch);

impl ApplyClass<JustifyContent> for Style {
    fn apply_class(&mut self, class: &JustifyContent) {
        self.justify_content = *class;
    }
}
