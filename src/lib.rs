pub use bevy;
pub use bevy::ui::Val;
use bevy::{
    ecs::system::Command,
    prelude::{despawn_with_children_recursive, App, Commands, Component, Entity, Plugin},
    reflect::Reflect,
};
use resources::EntityClassResources;

pub mod class;
pub mod resources;

#[derive(Debug, Clone, Copy)]
pub struct BsmlPlugin;

impl Plugin for BsmlPlugin {
    fn build(&self, app: &mut App) {
        EntityClassResources::build_app(app);
    }

    fn is_unique(&self) -> bool {
        true
    }
}

pub trait SpawnBsml {
    fn spawn_bsml<T: Bsml>(&mut self, node: T) -> Entity;
    fn despawn_bsml(&mut self, entity: Entity);
}

impl<'w, 's> SpawnBsml for bevy::ecs::system::Commands<'w, 's> {
    fn spawn_bsml<T: Bsml>(&mut self, node: T) -> Entity {
        let (entity, class_resources) = node.spawn(self, &[]);

        struct BsmlCommand(EntityClassResources);

        impl Command for BsmlCommand {
            fn apply(self, world: &mut bevy::prelude::World) {
                self.0.extend_resources(world);
            }
        }

        self.add(BsmlCommand(class_resources));

        entity
    }

    fn despawn_bsml(&mut self, entity: Entity) {
        struct BsmlCommand(Entity);

        impl Command for BsmlCommand {
            fn apply(self, world: &mut bevy::prelude::World) {
                resources::remove_node_from_class_resources(world, self.0);
                despawn_with_children_recursive(world, self.0);
            }
        }

        self.add(BsmlCommand(entity));
    }
}

pub trait Bsml {
    fn spawn(self, commands: &mut Commands, slot: &[Entity]) -> (Entity, EntityClassResources);

    fn taking_slot() -> bool {
        false
    }
}

/// Marker component for bsml node
#[derive(Debug, Clone, Component, Reflect)]
pub struct BsmlNode;

#[macro_export]
macro_rules! bsml {
    (($itag:ident $($attr:tt)*) $({$($content:tt)*})?) => {{
        #[derive($crate::bevy::prelude::Component, Clone, Copy)]
        pub struct __BsmlTag;
        $crate::bsml!(__BsmlTag; ($itag $($attr)*) $({$($content)*})?);
        __BsmlTag
    }};
    ($tag:ident; ($itag:ident $($attr:tt)*) $({$($content:tt)*})?) => {
        impl $crate::Bsml for $tag {
            #[allow(unused_variables)]
            fn spawn(self, commands: &mut $crate::bevy::ecs::system::Commands, slot: &[$crate::bevy::ecs::entity::Entity]) -> ($crate::bevy::ecs::entity::Entity, $crate::resources::EntityClassResources) {
                #[allow(unused_imports)]
                use $crate::{class::ApplyClass, resources::InsertEntityClassResource};
                use $crate::bevy::hierarchy::BuildChildren;

                #[allow(unused_mut)]
                let mut class_resources = $crate::resources::EntityClassResources::default();

                let entity = $crate::bsml!(@spawn(self, commands, slot, class_resources) ($itag $($attr)*) $({$($content)*})?);
                commands.entity(entity).insert(self);

                (entity, class_resources)
            }

            $crate::bsml!(@taking_slot ($itag) $({$($content)*})?);
        }
    };
    // handle node tag
    (@spawn($this:ident, $commands:ident, $slot:ident, $class_resources:ident) (node $(class=[$($class:expr),* $(,)?])?) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        #[allow(unused_mut)]
        let mut bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        $($(
            $crate::class::apply_class_to_node_bundle(&mut bundle, $crate::bevy::ui::Interaction::None, $class);
        )*)?

        let node = $commands.spawn((bundle, $crate::bevy::ui::Interaction::None, $crate::BsmlNode)).id();

        $($(
            $class .insert_entity_class_resource(node, &mut $class_resources);
        )*)?

        let children = [$($(
            $crate::bsml!(@spawn($this, $commands, $slot, $class_resources) ($($def)+) $({$($imp)*})?),
        )*)?];
        $commands.entity(node).push_children(&children);

        node
    }};
    // handle text tag
    (@spawn($this:ident, $commands:ident, $slot:ident, $class_resources:ident) (text $(class=[$($class:expr),* $(,)?])?) {$($words:tt)+}) => {{
        let mut bundle = $crate::bevy::prelude::TextBundle::from_section(
            $crate::bsml!(@format($this) $($words)+),
            $crate::bevy::text::TextStyle::default());

        $($(
            $crate::class::apply_class_to_text_bundle(&mut bundle, $crate::bevy::ui::Interaction::None, $class);
        )*)?

        let id = $commands.spawn((bundle, $crate::bevy::ui::Interaction::None, $crate::BsmlNode)).id();

        $($(
            $class .insert_entity_class_resource(id, &mut $class_resources);
        )*)?

        id
    }};
    // handle slot tag
    (@spawn($this:ident, $commands:ident, $slot:ident, $class_resources:ident) (slot $(class=[$($class:expr),* $(,)?])?) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        #[allow(unused_mut)]
        let mut bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        $($(
            $crate::class::apply_class_to_node_bundle(&mut bundle, $crate::bevy::ui::Interaction::None, $class);
        )*)?

        let parent = $commands.spawn((bundle, $crate::bevy::ui::Interaction::None, $crate::BsmlNode)).id();

        if $slot .is_empty() {
            let children = [
                $($(
                    $crate::bsml!(@spawn($this, $commands, $slot, $class_resources) ($($def)+) $({$($imp)*})?),
                )*)?
            ];
            $commands.entity(parent).push_children(&children);
        } else {
            $commands.entity(parent).push_children(&$slot);
        }

        $($(
            $class .insert_entity_class_resource(parent, &mut $class_resources);
        )*)?

        parent
    }};
    // handle custom component
    (@spawn($this:ident, $commands:ident, $_slot:ident, $class_resources:ident) ($itag:ident $($attr:tt)*) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        let tag = $crate::bsml!(@tag($this, $itag) in:[$($attr)*] fields:[]);

        let (parent, tag_class_resources) = if <$itag as $crate::Bsml> :: taking_slot() {
            let children = [$($(
                $crate::bsml!(@spawn($this, $commands, $_slot, $class_resources) ($($def)+) $({$($imp)*})?),
            )*)?];

            tag.spawn($commands, &children)
        } else {

            tag.spawn($commands, &[])
        };

        $class_resources.extend(tag_class_resources);

        parent
    }};
    // parse attribute tokens into fields, and populate to initialize component
    (@tag($this:ident, $tag:ident) in:[] fields:[$(($f:ident : $v:expr)),*] ) => {
        $tag { $($f: $v),* }
    };
    (@tag($this:ident, $tag:ident) in:[$t:ident={self . $field:ident $(. $method:ident $($(::<$($pa:path),+>)?($($p:expr),* $(,)?))?)* $(as $c:ty)?} $($i:tt)*] fields:[$(($f:ident: $v:expr)),*] ) => {
        $crate::bsml!(@tag($this, $tag) in:[$($i)*] fields:[$(($f: $v),)* ($t: $this . $field $(. $method $($(::<$($pa),+>)? ($($p),*))?)? .clone() $(as $c)?)])
    };
    (@tag($this:ident, $tag:ident) in:[$t:ident={$e:expr} $($i:tt)*] fields:[$(($f:ident: $v:expr)),*] ) => {
        $crate::bsml!(@tag($this, $tag) in:[$($i)*] fields:[$(($f: $v),)* ($t: $e)])
    };
    (@tag($this:ident, $tag:ident) in:[$t:ident=$t2:ident $($i:tt)*] fields:[$(($f:ident: $v:expr)),*] ) => {
        $crate::bsml!(@tag($this, $tag) in:[$($i)*] fields:[$(($f: $v),)* ($t: $this . $t2 .clone())])
    };
    (@tag($this:ident, $tag:ident) in:[$t:ident $($i:tt)*] fields:[$(($f:ident: $v:expr)),*] ) => {
        $crate::bsml!(@tag($this, $tag) in:[$($i)*] fields:[$(($f: $v),)* ($t: $this . $t .clone())])
    };
    // root component struct definition
    (@struct_def $tag:ident {}) => {
        #[derive(Debug, Clone, $crate::bevy::ecs::component::Component, $crate::bevy::reflect::Reflect)]
        pub struct $tag;
    };
    (@struct_def $tag:ident {$($f:ident : $t:ty),*}) => {
        #[derive(Debug, Clone, $crate::bevy::ecs::component::Component, $crate::bevy::reflect::Reflect)]
        pub struct $tag {
            $(
                pub $f: $t,
            )*
        }
    };
    // impl taking_slot (slot) exists
    (@taking_slot (slot $($_:tt)*) $($_1:tt)?) => {
        fn taking_slot() -> bool {
            true
        }
    };
    (@taking_slot (text $($_:tt)*) $($_1:tt)?) => {};
    (@taking_slot (node $($_:tt)*) $($_1:tt)?) => {};
    (@taking_slot ($($_:tt)*) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {
        $($(
            $crate::bsml!(@taking_slot ($($def)+) $({$($imp)*})? );
        )*)?
    };
    // parse string tokens with fields into format string
    (@format($this:ident) $str:tt $(,$f:ident)* $(,)?) => {
        format!($str, $($this . $f),*)
    };
}
