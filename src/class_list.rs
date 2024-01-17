use crate::{
    class::{
        background_color::BackgroundColorClass, border_color::BorderColorClass, text::TextClass,
        ApplyClass, ClassEnum, StyleClass,
    },
    BsmlNode,
};
use bevy::{
    prelude::{App, Changed, Commands, Component, Entity, Or, Query, With},
    ui::{Interaction, ZIndex},
};

/// Bevy component: list of classes that apply the styles to UI Node.
///
/// You can access this component in your system to change styles of a node.
#[derive(Debug, Clone, Component)]
pub struct ClassList<T>(Vec<(Interaction, T)>);

impl<T> ClassList<T> {
    pub fn set<F: Into<T>>(&mut self, interaction: Interaction, class: F) {
        let class = class.into();

        let variant_eq = |a: &T, b: &T| std::mem::discriminant(&a) == std::mem::discriminant(&b);

        for (i, c) in &mut self.0 {
            if *i == interaction && variant_eq(c, &class) {
                *c = class;
                return;
            }
        }

        self.0.push((interaction, class));
    }
}

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
        #[doc(hidden)]
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
                        (Or<(Changed<Interaction>, Changed<ClassList<T>>)>, With<BsmlNode>),
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
