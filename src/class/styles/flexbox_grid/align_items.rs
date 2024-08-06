use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{AlignItems, Style};

pub const ITEMS_BASELINE: StyleClass = StyleClass::AlignItems(AlignItems::Baseline);
pub const ITEMS_CENTER: StyleClass = StyleClass::AlignItems(AlignItems::Center);
pub const ITEMS_DEFAULT: StyleClass = StyleClass::AlignItems(AlignItems::Default);
pub const ITEMS_END: StyleClass = StyleClass::AlignItems(AlignItems::End);
pub const ITEMS_FLEX_END: StyleClass = StyleClass::AlignItems(AlignItems::FlexEnd);
pub const ITEMS_FLEX_START: StyleClass = StyleClass::AlignItems(AlignItems::FlexStart);
pub const ITEMS_START: StyleClass = StyleClass::AlignItems(AlignItems::Start);
pub const ITEMS_STRETCH: StyleClass = StyleClass::AlignItems(AlignItems::Stretch);

impl ApplyClass for AlignItems {
    type Component = Style;

    fn apply_class(&self, component: &mut Self::Component) {
        component.align_items = *self;
    }
}
