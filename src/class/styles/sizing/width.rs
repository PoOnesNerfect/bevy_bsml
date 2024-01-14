use crate::ApplyClass;
use bevy::ui::{Interaction, Style, Val};

#[derive(Debug, Clone, Copy)]
pub struct Width(pub Val);

impl Width {
    pub fn px(px: f32) -> Self {
        Self(Val::Px(px))
    }
}

impl ApplyClass for Width {
    type Component = Style;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.width = self.0;
    }
}

pub const W_AUTO: Width = Width(Val::Auto);
pub const W_FULL: Width = Width(Val::Percent(100.0));
