use crate::{
    class::{styles::StyleClass, text::TextClass, ApplyClass, InteractionClass},
    BsmlNode,
};
use bevy::{
    prelude::{App, Changed, Children, Component, Entity, Mut, Query, Res, Resource, With, World},
    text::Text,
    ui::{BackgroundColor, BorderColor, Interaction, Style, ZIndex},
};

#[doc(hidden)]
#[derive(Debug, Clone, Resource)]
pub struct ClassMap<T>(pub Option<Vec<(Entity, Vec<InteractionClass<T>>)>>);

impl<T> Default for ClassMap<T> {
    fn default() -> Self {
        Self(None)
    }
}

macros::impl_entity_class_maps!(
    style: Style => StyleClass,
    background_color: BackgroundColor => BackgroundColor,
    border_color: BorderColor => BorderColor,
    z_index: ZIndex => ZIndex,
    text: Text => TextClass,
);
mod macros {
    macro_rules! impl_entity_class_maps {
    ($($f:ident : $c:ty => $t:ty),* $(,)?) => {
        #[doc(hidden)]
        #[derive(Debug, Default, Clone)]
        pub struct EntityClassResources {
            $(pub $f: ClassMap<$t>,)*
        }

        impl EntityClassResources {
            pub fn is_empty(&self) -> bool {
                $(self.$f.0.is_none() &&)* true
            }

            pub fn build_app(app: &mut App) {
                $(app.init_resource::<ClassMap<$t>>();)*

                app.add_systems(
                    bevy::prelude::Update,
                    (
                        $(apply_class_system::<$t, $c>,)*
                    ),
                );

                fn apply_class_system<
                    T: 'static + Send + Sync + ApplyClass<Component = C>,
                    C: Component,
                >(
                    mut interaction_query: Query<
                        (Entity, &Interaction, &mut C),
                        (Changed<Interaction>, With<BsmlNode>),
                    >,
                    class_map: Res<ClassMap<T>>,
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
            }

            pub fn extend(&mut self, other: Self) {
                $(
                if let Some(other) = other.$f.0 {
                    if let Some(self_) = self.$f.0.as_mut() {
                        self_.extend(other);
                    } else {
                        self.$f = ClassMap(Some(other));
                    }
                }
                )*
            }

            pub fn extend_resources(self, world: &mut World) {
                if !self.is_empty() {
                    let Self {
                        $( $f, )*
                    } = self;

                    $(extend_resource(world, $f);)*

                    fn extend_resource<T: 'static + Send + Sync>(
                        world: &mut World,
                        class_map: ClassMap<T>,
                    ) {
                        if class_map.0.is_some() {
                            world.resource_scope(|_, mut resource: Mut<ClassMap<T>>| {
                                if let Some(resource) = resource.0.as_mut() {
                                    resource.extend(class_map.0.unwrap());
                                } else {
                                    *resource = class_map;
                                }
                            });
                        }
                    }
                }
            }
        }

        #[doc(hidden)]
        pub fn remove_node_from_class_resources(world: &mut World, entity: Entity) {
            let mut entities = vec![entity];
            collect_nested_children(world, entity, &mut entities);

            if !entities.is_empty() {
                $(remove_entities_from_class_map::<$t>(world, &entities);)*
            }
        }

        fn collect_nested_children(world: &World, entity: Entity, entities: &mut Vec<Entity>) {
            if let Some(children) = world.get::<Children>(entity) {
                for e in children {
                    collect_nested_children(world, *e, entities);
                }
            }
        }

        fn remove_entities_from_class_map<T: 'static + Send + Sync>(
            world: &mut World,
            entities: &[Entity],
        ) {
            world.resource_scope(|_, mut resource: Mut<ClassMap<T>>| {
                if let Some(resource) = resource.0.as_mut() {
                    resource.retain(|(e, _)| !entities.contains(e));
                }
            });
        }

        #[doc(hidden)]
        pub trait InsertEntityClassResource<T: Component> {
            fn insert_entity_class_resource(self, entity: Entity, class_map: &mut EntityClassResources);
        }

        $(
        impl InsertEntityClassResource<$c> for InteractionClass<$t> {
            fn insert_entity_class_resource(self, entity: Entity, class_map: &mut EntityClassResources) {
                if let Some(class_map) = class_map.$f.0.as_mut() {
                    if let Some(classes) = class_map.iter_mut().find(|(e, _)| *e == entity) {
                        classes.1.push(self);
                    } else {
                        class_map.push((entity, vec![self]));
                    }
                } else {
                    class_map.$f = ClassMap(Some(vec![(entity, vec![self])]));
                }
            }
        }

        impl InsertEntityClassResource<$c> for $t {
            fn insert_entity_class_resource(self, entity: Entity, class_map: &mut EntityClassResources) {
                let class = InteractionClass {
                    inner: self,
                    interaction: Interaction::None,
                };

                class.insert_entity_class_resource(entity, class_map)
            }
        }
        )*
    };
    }
    pub(super) use impl_entity_class_maps;
}
