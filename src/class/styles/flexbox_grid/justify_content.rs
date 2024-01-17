use crate::class::{styles::StyleClass, ApplyClass};
use bevy::ui::{JustifyContent, Style};

pub const JUSTIFY_CENTER: StyleClass = StyleClass::JustifyContent(JustifyContent::Center);

impl ApplyClass for JustifyContent {
    type Component = Style;

    fn apply_class(&self, component: &mut Self::Component) {
        component.justify_content = *self;
    }
}
