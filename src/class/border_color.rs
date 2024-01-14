use crate::ApplyClass;
use bevy::{
    prelude::Color,
    ui::{BorderColor, Interaction},
};

pub const BORDER_TRANSPARENT: BorderColor = BorderColor(Color::NONE);

impl ApplyClass for BorderColor {
    type Component = BorderColor;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        *component = *self;
    }
}
