use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{AlignItems, Style};

pub const ITEMS_CENTER: StyleClass = StyleClass::AlignItems(AlignItems::Center);

impl ApplyClass for AlignItems {
    type Component = Style;

    fn apply_class(&self, component: &mut Self::Component) {
        component.align_items = *self;
    }
}
