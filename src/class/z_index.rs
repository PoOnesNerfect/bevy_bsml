use crate::class::ApplyClass;
use bevy::ui::{Interaction, ZIndex};

pub fn z_local(z: i32) -> ZIndex {
    ZIndex::Local(z)
}

pub fn z_global(z: i32) -> ZIndex {
    ZIndex::Global(z)
}

impl ApplyClass for ZIndex {
    type Component = ZIndex;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        *component = *self;
    }
}
