use crate::class::ApplyClass;
use bevy_ui::Style;

pub use bevy_ui::JustifyContent;

pub const JUSTIFY_CENTER: JustifyContent = JustifyContent::Center;
pub const JUSTIFY_DEFAULT: JustifyContent = JustifyContent::Default;
pub const JUSTIFY_END: JustifyContent = JustifyContent::FlexEnd;
pub const JUSTIFY_START: JustifyContent = JustifyContent::FlexStart;
pub const JUSTIFY_AROUND: JustifyContent = JustifyContent::SpaceAround;
pub const JUSTIFY_BETWEEN: JustifyContent = JustifyContent::SpaceBetween;
pub const JUSTIFY_EVENLY: JustifyContent = JustifyContent::SpaceEvenly;
pub const JUSTIFY_STRETCH: JustifyContent = JustifyContent::Stretch;

impl ApplyClass<JustifyContent> for Style {
    fn apply_class(&mut self, class: &JustifyContent) {
        self.justify_content = *class;
    }
}
