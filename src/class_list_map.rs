use crate::{
    class::{
        background_color::BackgroundColorClass, border_color::BorderColorClass, text::TextClass,
        ApplyClass, ClassEnum, StyleClass,
    },
    BsmlNode,
};
use bevy::{
    prelude::{App, Changed, Commands, Component, Entity, Query, With},
    ui::{Interaction, ZIndex},
};

#[derive(Debug, Clone, Component)]
pub struct ClassList<T>(Vec<(Interaction, T)>);

macros::impl_class_list_map!(
    style: StyleClass => Style,
    background_color: BackgroundColorClass => BackgroundColor,
    border_color: BorderColorClass => BorderColor,
    z_index: ZIndex => ZIndex,
    text: TextClass => Text,
);
mod macros {
    macro_rules! impl_class_list_map {
    ($($f:ident : $t:ty => $i:ident),* $(,)?) => {
        #[derive(Debug, Default, Clone)]
        pub struct ClassListMap {
            $(pub $f: Option<ClassList<$t>>,)*
        }

        impl ClassListMap {
            pub fn is_empty(&self) -> bool {
                $(self.$f.is_none() &&)* true
            }

            pub fn insert(&mut self, interaction: Interaction, class: ClassEnum) {
                match class {
                    $(ClassEnum::$i(class) => {
                        if self.$f.is_none() {
                            self.$f = Some(ClassList(Vec::new()));
                        }

                        self.$f.as_mut().unwrap().0.push((interaction, class));
                    })*
                }
            }

            pub fn spawn(mut self, commands: &mut Commands, entity: Entity) {
                $(if let Some(class_list) = self.$f.take() {
                    commands.entity(entity).insert(class_list);
                })*
            }

            pub fn build_app(app: &mut App) {
                app.add_systems(
                    bevy::prelude::Update,
                    (
                        $(apply_class_system::<$t>,)*
                    ),
                );

                fn apply_class_system<T: 'static + Send + Sync + ApplyClass>(
                    mut query: Query<
                        (&Interaction, &ClassList<T>, &mut T::Component),
                        (Changed<Interaction>, With<BsmlNode>),
                    >,
                ) {
                    for (interaction, classes, mut component) in &mut query {
                        for (i, c) in &classes.0 {
                            if i == interaction {
                                c.apply_class(&mut component);
                            }
                        }
                    }
                }
            }
        }
    };
    }

    pub(super) use impl_class_list_map;
}
