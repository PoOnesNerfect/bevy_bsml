use self::{
    layout::AspectRatio,
    sizing::{Height, Width},
};
use crate::class::ApplyClass;
use bevy::ui::{Interaction, JustifyContent, Style};
use derive_more::From;

pub mod flexbox_grid;
pub mod layout;
pub mod sizing;

#[derive(Debug, Clone, From)]
pub enum StyleClass {
    Width(Width),
    Height(Height),
    AspectRatio(AspectRatio),
    JustifyContent(JustifyContent),
}

impl ApplyClass for StyleClass {
    type Component = Style;

    fn apply_class(&self, interaction: Interaction, component: &mut Self::Component) {
        match self {
            Self::Width(width) => width.apply_class(interaction, component),
            Self::Height(height) => height.apply_class(interaction, component),
            Self::AspectRatio(aspect_ratio) => aspect_ratio.apply_class(interaction, component),
            Self::JustifyContent(justify_content) => {
                justify_content.apply_class(interaction, component)
            }
        }
    }
}
