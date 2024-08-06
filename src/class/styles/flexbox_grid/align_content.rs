use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{AlignContent, Style};

pub const CONTENT_CENTER: StyleClass = StyleClass::AlignContent(AlignContent::Center);
pub const CONTENT_END: StyleClass = StyleClass::AlignContent(AlignContent::End);
pub const CONTENT_DEFAULT: StyleClass = StyleClass::AlignContent(AlignContent::Default);
pub const CONTENT_FLEX_END: StyleClass = StyleClass::AlignContent(AlignContent::FlexEnd);
pub const CONTENT_FLEX_START: StyleClass = StyleClass::AlignContent(AlignContent::FlexStart);
pub const CONTENT_SPACE_AROUND: StyleClass = StyleClass::AlignContent(AlignContent::SpaceAround);
pub const CONTENT_SPACE_BETWEEN: StyleClass = StyleClass::AlignContent(AlignContent::SpaceBetween);
pub const CONTENT_SPACE_EVENLY: StyleClass = StyleClass::AlignContent(AlignContent::SpaceEvenly);
pub const CONTENT_START: StyleClass = StyleClass::AlignContent(AlignContent::Start);
pub const CONTENT_STRETCH: StyleClass = StyleClass::AlignContent(AlignContent::Stretch);

impl ApplyClass for AlignContent {
    type Component = Style;

    fn apply_class(&self, component: &mut Self::Component) {
        component.align_content = *self;
    }
}
