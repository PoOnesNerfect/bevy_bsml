use crate::ApplyClass;
use bevy::ui::{Interaction, JustifyContent, Style};

impl ApplyClass for JustifyContent {
    type Component = Style;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.justify_content = *self;
    }
}

pub const JUSTIFY_CENTER: JustifyContent = JustifyContent::Center;
