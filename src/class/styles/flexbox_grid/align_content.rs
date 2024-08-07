use crate::class::ApplyClass;
use bevy_ui::Style;

pub use bevy_ui::AlignContent;

pub const CONTENT_CENTER: AlignContent = AlignContent::Center;
pub const CONTENT_DEFAULT: AlignContent = AlignContent::Default;
pub const CONTENT_END: AlignContent = AlignContent::FlexEnd;
pub const CONTENT_START: AlignContent = AlignContent::FlexStart;
pub const CONTENT_AROUND: AlignContent = AlignContent::SpaceAround;
pub const CONTENT_BETWEEN: AlignContent = AlignContent::SpaceBetween;
pub const CONTENT_EVENLY: AlignContent = AlignContent::SpaceEvenly;
pub const CONTENT_STRETCH: AlignContent = AlignContent::Stretch;

impl ApplyClass<AlignContent> for Style {
    fn apply_class(&mut self, class: &AlignContent) {
        self.align_content = *class;
    }
}
