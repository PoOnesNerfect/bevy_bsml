use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::Style;

pub use bevy::ui::AlignContent;

pub const CONTENT_CENTER: StyleClass = StyleClass::AlignContent(AlignContent::Center);
pub const CONTENT_DEFAULT: StyleClass = StyleClass::AlignContent(AlignContent::Default);
pub const CONTENT_END: StyleClass = StyleClass::AlignContent(AlignContent::FlexEnd);
pub const CONTENT_START: StyleClass = StyleClass::AlignContent(AlignContent::FlexStart);
pub const CONTENT_AROUND: StyleClass = StyleClass::AlignContent(AlignContent::SpaceAround);
pub const CONTENT_BETWEEN: StyleClass = StyleClass::AlignContent(AlignContent::SpaceBetween);
pub const CONTENT_EVENLY: StyleClass = StyleClass::AlignContent(AlignContent::SpaceEvenly);
pub const CONTENT_STRETCH: StyleClass = StyleClass::AlignContent(AlignContent::Stretch);

impl ApplyClass<AlignContent> for Style {
    fn apply_class(&mut self, class: &AlignContent) {
        self.align_content = *class;
    }
}
