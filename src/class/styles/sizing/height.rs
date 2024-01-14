use crate::ApplyClass;
use bevy::ui::{Interaction, Style, Val};

#[derive(Debug, Clone, Copy)]
pub struct Height(pub Val);

impl Height {
    pub fn px(px: f32) -> Self {
        Self(Val::Px(px))
    }
}

impl ApplyClass for Height {
    type Component = Style;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.height = self.0;
    }
}

pub const H_AUTO: Height = Height(Val::Auto);
pub const H_FULL: Height = Height(Val::Percent(100.0));
