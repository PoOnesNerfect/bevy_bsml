use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{AlignItems, Interaction, Style};

pub const ITEMS_CENTER: StyleClass = StyleClass::AlignItems(AlignItems::Center);

impl ApplyClass for AlignItems {
    type Component = Style;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.align_items = *self;
    }
}
