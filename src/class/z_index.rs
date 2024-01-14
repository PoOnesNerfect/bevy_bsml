use crate::ApplyClass;
use bevy::ui::{Interaction, ZIndex};

impl ApplyClass for ZIndex {
    type Component = ZIndex;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        *component = *self;
    }
}
