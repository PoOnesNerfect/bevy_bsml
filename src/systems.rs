use crate::{
    map::{BackgroundColorClassMap, BorderColorClassMap, StyleClassMap, ZIndexClassMap},
    ApplyClass, BsmlNode,
};
use bevy::{
    prelude::{Changed, Entity, Query, Res, With},
    ui::{BackgroundColor, BorderColor, Interaction, Style, ZIndex},
};

pub fn apply_style_class_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut Style),
        (Changed<Interaction>, With<BsmlNode>),
    >,
    class_map: Res<StyleClassMap>,
) {
    let Some(class_map) = class_map.0.as_ref() else {
        return;
    };

    for (entity, interaction, mut component) in &mut interaction_query {
        let classes = class_map
            .iter()
            .find_map(|(e, c)| (*e == entity).then_some(c));

        let Some(classes) = classes else {
            continue;
        };

        for class in classes {
            class.apply_class(*interaction, &mut component);
        }
    }
}

pub fn apply_background_color_class_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<BsmlNode>),
    >,
    class_map: Res<BackgroundColorClassMap>,
) {
    let Some(class_map) = class_map.0.as_ref() else {
        return;
    };

    for (entity, interaction, mut component) in &mut interaction_query {
        let classes = class_map
            .iter()
            .find_map(|(e, c)| (*e == entity).then_some(c));

        let Some(classes) = classes else {
            continue;
        };

        for class in classes {
            class.apply_class(*interaction, &mut component);
        }
    }
}

pub fn apply_border_color_class_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut BorderColor),
        (Changed<Interaction>, With<BsmlNode>),
    >,
    class_map: Res<BorderColorClassMap>,
) {
    let Some(class_map) = class_map.0.as_ref() else {
        return;
    };

    for (entity, interaction, mut component) in &mut interaction_query {
        let classes = class_map
            .iter()
            .find_map(|(e, c)| (*e == entity).then_some(c));

        let Some(classes) = classes else {
            continue;
        };

        for class in classes {
            class.apply_class(*interaction, &mut component);
        }
    }
}

pub fn apply_z_index_class_system(
    mut interaction_query: Query<
        (Entity, &Interaction, &mut ZIndex),
        (Changed<Interaction>, With<BsmlNode>),
    >,
    class_map: Res<ZIndexClassMap>,
) {
    let Some(class_map) = class_map.0.as_ref() else {
        return;
    };

    for (entity, interaction, mut component) in &mut interaction_query {
        let classes = class_map
            .iter()
            .find_map(|(e, c)| (*e == entity).then_some(c));

        let Some(classes) = classes else {
            continue;
        };

        for class in classes {
            class.apply_class(*interaction, &mut component);
        }
    }
}
