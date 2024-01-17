use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{AlignContent, Style};

pub const CONTENT_CENTER: StyleClass = StyleClass::AlignContent(AlignContent::Center);

impl ApplyClass for AlignContent {
    type Component = Style;

    fn apply_class(&self, component: &mut Self::Component) {
        component.align_content = *self;
    }
}
