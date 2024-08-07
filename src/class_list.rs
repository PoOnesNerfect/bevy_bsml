use bevy_ecs::prelude::*;
use bevy_text::Text;
use bevy_ui::prelude::*;

use crate::{
    class::{ApplyClass, BsmlClass},
    Bsml,
};

/// Bevy component: list of classes that apply the styles to UI Node.
///
/// You can access this component in your system to change styles of a node.
#[derive(Debug, Clone, Component, Default)]
pub struct BsmlClasses(pub Vec<(Interaction, BsmlClass)>);

impl BsmlClasses {
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }

    pub fn insert(&mut self, interaction: Interaction, class: impl Into<BsmlClass>) {
        let class = class.into();

        for (i, c) in &mut self.0 {
            if *i == interaction && c.eq_class_type(&class) {
                *c = class;
                return;
            }
        }

        self.0.push((interaction, class));
    }

    pub fn remove(&mut self, interaction: Interaction, class: impl Into<BsmlClass>) {
        let class = class.into();

        self.0
            .retain(|(i, c)| *i != interaction || !c.eq_class_type(&class));
    }

    pub fn iter(&self) -> impl Iterator<Item = &(Interaction, BsmlClass)> {
        self.0.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut (Interaction, BsmlClass)> {
        self.0.iter_mut()
    }
}

pub(super) fn apply_class_system(
    mut query: Query<
        (
            &Interaction,
            &BsmlClasses,
            Option<&mut Style>,
            Option<&mut Text>,
            Option<&mut ZIndex>,
            Option<&mut BorderColor>,
            Option<&mut BackgroundColor>,
        ),
        (Or<(Changed<Interaction>, Changed<BsmlClasses>)>, With<Bsml>),
    >,
) {
    for (interaction, classes, mut style, mut text, mut z_index, mut border_color, mut bg_color) in
        &mut query
    {
        for (_, class) in classes.0.iter().filter(|(i, _)| i == interaction) {
            if let Some(style) = &mut style {
                style.apply_class(class);
            }
            if let Some(text) = &mut text {
                text.apply_class(class);
            }
            if let Some(z_index) = &mut z_index {
                z_index.apply_class(class);
            }
            if let Some(border_color) = &mut border_color {
                border_color.apply_class(class);
            }
            if let Some(bg_color) = &mut bg_color {
                bg_color.apply_class(class);
            }
        }
    }
}
