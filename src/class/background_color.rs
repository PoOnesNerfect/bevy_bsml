use crate::ApplyClass;
use bevy::{
    prelude::Color,
    ui::{BackgroundColor, Interaction},
};

pub const BG_TRANSPARENT: BackgroundColor = BackgroundColor(Color::NONE);

impl ApplyClass for BackgroundColor {
    type Component = BackgroundColor;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        *component = *self;
    }
}
