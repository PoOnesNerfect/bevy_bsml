use bevy::{
    ecs::system::Command,
    prelude::{despawn_with_children_recursive, App, Commands, Component, Entity, Plugin},
    reflect::Reflect,
};
use class_list_map::ClassListMap;

pub use bevy;
pub use bevy::ui::Val;

// used for solving hygiene issue when trying to access `self` in macro rules
#[doc(hidden)]
pub use replace_ident::replace_ident;

pub mod class;
pub mod class_list_map;

pub mod prelude {
    pub use crate::{
        bsml,
        class::{
            background_color::*, border_color::*, flexbox_grid::align_items::*,
            flexbox_grid::flex_direction::*, flexbox_grid::flex_wrap::*, flexbox_grid::gap::*,
            flexbox_grid::justify_content::*, hovered, pressed, sizing::*, text::*, z_index::*,
        },
        BsmlPlugin, SpawnBsml,
    };
}

#[derive(Debug, Clone, Copy)]
pub struct BsmlPlugin;

impl Plugin for BsmlPlugin {
    fn build(&self, app: &mut App) {
        ClassListMap::build_app(app);
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
        node.spawn(self, &[])
    }

    fn despawn_bsml(&mut self, entity: Entity) {
        struct BsmlCommand(Entity);

        impl Command for BsmlCommand {
            fn apply(self, world: &mut bevy::prelude::World) {
                despawn_with_children_recursive(world, self.0);
            }
        }

        self.add(BsmlCommand(entity));
    }
}

pub trait Bsml {
    fn spawn(self, commands: &mut Commands, slot: &[Entity]) -> Entity;

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
            fn spawn(self, commands: &mut $crate::bevy::ecs::system::Commands, slot: &[$crate::bevy::ecs::entity::Entity]) -> $crate::bevy::ecs::entity::Entity {
                #[allow(unused_imports)]
                use $crate::class::ApplyClass;
                use $crate::bevy::hierarchy::BuildChildren;

                let this = &self;
                let entity = $crate::bsml!(@spawn(this, commands, slot) ($itag $($attr)*) $({$($content)*})?);
                commands.entity(entity).insert(self);

                entity
            }

            $crate::bsml!(@taking_slot ($itag) $({$($content)*})?);
        }
    };
    // handle node tag
    (@spawn($this:ident, $commands:ident, $slot:ident) (node $(class=[$($class:expr),* $(,)?])?) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        #[allow(unused_mut)]
        let mut bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        #[allow(unused_mut)]
        let mut class_map = $crate::class_list_map::ClassListMap::default();

        $({
            use $crate::class::WithInteraction;
            $(
                let (interaction, mut class) = $crate::replace_ident!(self, $this, $class).with_interaction();

                if interaction == $crate::bevy::ui::Interaction::None {
                    class.apply_to_node_bundle(&mut bundle);
                }

                class_map.insert(interaction, class);
            )*
        })?

        let entity = $commands.spawn((bundle, $crate::bevy::ui::Interaction::None, $crate::BsmlNode)).id();

        class_map.spawn($commands, entity);

        let children = [$($(
            $crate::bsml!(@spawn($this, $commands, $slot) ($($def)+) $({$($imp)*})?),
        )*)?];

        $commands.entity(entity).push_children(&children);

        entity
    }};
    // handle for tag with enumeration
    (@spawn($this:ident, $commands:ident, $slot:ident) (for {$i:ident, $v:ident in $iter:expr} $(class=[$($class:expr),* $(,)?])?) {$(($($def:tt)+) $({$($imp:tt)*})?)*}) => {{
        #[allow(unused_mut)]
        let mut bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        #[allow(unused_mut)]
        let mut class_map = $crate::class_list_map::ClassListMap::default();

        $({
            use $crate::class::WithInteraction;
            $(
                let (interaction, mut class) = $crate::replace_ident!(self, $this, $class).with_interaction();

                if interaction == $crate::bevy::ui::Interaction::None {
                    class.apply_to_node_bundle(&mut bundle);
                }

                class_map.insert(interaction, class);
            )*
        })?

        let entity = $commands.spawn((bundle, $crate::bevy::ui::Interaction::None, $crate::BsmlNode)).id();

        class_map.spawn($commands, entity);

        for ($i, $v) in ($crate::replace_ident!(self, $this, $iter)).into_iter().enumerate() {
            let children = [$(
                $crate::bsml!(@spawn($this, $commands, $slot) ($($def)+) $({$($imp)*})?),
            )*];

            $commands.entity(entity).push_children(&children);
        }

        entity
    }};
    // handle for tag
    (@spawn($this:ident, $commands:ident, $slot:ident) (for {$i:ident in $iter:expr} $(class=[$($class:expr),* $(,)?])?) {$(($($def:tt)+) $({$($imp:tt)*})?)*}) => {{
        #[allow(unused_mut)]
        let mut bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        #[allow(unused_mut)]
        let mut class_map = $crate::class_list_map::ClassListMap::default();

        $({
            use $crate::class::WithInteraction;
            $(
                let (interaction, mut class) = $crate::replace_ident!(self, $this, $class).with_interaction();

                if interaction == $crate::bevy::ui::Interaction::None {
                    class.apply_to_node_bundle(&mut bundle);
                }

                class_map.insert(interaction, class);
            )*
        })?

        let entity = $commands.spawn((bundle, $crate::bevy::ui::Interaction::None, $crate::BsmlNode)).id();

        class_map.spawn($commands, entity);

        for $i in $crate::replace_ident!(self, $this, $iter) {
            let children = [$(
                $crate::bsml!(@spawn($this, $commands, $slot) ($($def)+) $({$($imp)*})?),
            )*];

            $commands.entity(entity).push_children(&children);
        }

        entity
    }};
    // handle text tag
    (@spawn($this:ident, $commands:ident, $slot:ident) (text $(class=[$($class:expr),* $(,)?])?) {$literal:expr $(,$value:expr)*}) => {{
        let mut bundle = $crate::bevy::prelude::TextBundle::from_section(
            format!($literal $(, $crate::replace_ident!(self, $this, $value))*),
            $crate::bevy::text::TextStyle::default());

        #[allow(unused_mut)]
        let mut class_map = $crate::class_list_map::ClassListMap::default();

        $({
            use $crate::class::WithInteraction;
            $(
                let (interaction, mut class) = $class.with_interaction();

                if interaction == $crate::bevy::ui::Interaction::None {
                    class.apply_to_text_bundle(&mut bundle);
                }

                class_map.insert(interaction, class);
            )*
        })?

        let entity = $commands.spawn((bundle, $crate::bevy::ui::Interaction::None, $crate::BsmlNode)).id();

        class_map.spawn($commands, entity);

        entity
    }};
    // handle slot tag
    (@spawn($this:ident, $commands:ident, $slot:ident) (slot $(class=[$($class:expr),* $(,)?])?) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        #[allow(unused_mut)]
        let mut bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        #[allow(unused_mut)]
        let mut class_map = $crate::class_list_map::ClassListMap::default();

        $({
            use $crate::class::WithInteraction;
            $(
                let (interaction, mut class) = $crate::replace_ident!(self, $this, $class).with_interaction();

                if interaction == $crate::bevy::ui::Interaction::None {
                    class.apply_to_node_bundle(&mut bundle);
                }

                class_map.insert(interaction, class);
            )*
        })?

        let entity = $commands.spawn((bundle, $crate::bevy::ui::Interaction::None, $crate::BsmlNode)).id();

        class_map.spawn($commands, entity);

        if $slot .is_empty() {
            let children = [
                $($(
                    $crate::bsml!(@spawn($this, $commands, $slot) ($($def)+) $({$($imp)*})?),
                )*)?
            ];
            $commands.entity(entity).push_children(&children);
        } else {
            $commands.entity(entity).push_children(&$slot);
        }

        entity
    }};
    // handle custom component
    (@spawn($this:ident, $commands:ident, $_slot:ident) ($itag:ident $($attr:tt)*) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        let tag = $crate::bsml!(@tag($this, $itag) in:[$($attr)*] fields:[]);

        let entity = if <$itag as $crate::Bsml> :: taking_slot() {
            let children = [$($(
                $crate::bsml!(@spawn($this, $commands, $_slot) ($($def)+) $({$($imp)*})?),
            )*)?];

            tag.spawn($commands, &children)
        } else {

            tag.spawn($commands, &[])
        };

        entity
    }};
    // parse attribute tokens into fields, and populate to initialize component
    (@tag($this:ident, $tag:ident) in:[] fields:[$(($f:ident : $v:expr)),* $(,)?] $($default:ident)? ) => {
        $tag { $($f: $v),* $(, ..Default::$default())? }
    };
    (@tag($this:ident, $tag:ident) in:[..default] fields:[$(($f:ident : $v:expr)),* $(,)?] ) => {
        $crate::bsml!(@tag($this, $tag) in:[] fields:[$(($f: $v),)*] default)
    };
    (@tag($this:ident, $tag:ident) in:[$t:ident={$e:expr} $($i:tt)*] fields:[$(($f:ident: $v:expr)),*] ) => {
        $crate::bsml!(@tag($this, $tag) in:[$($i)*] fields:[$(($f: $v),)* ($t: $crate::replace_ident!(self, $this, $e))])
    };
    (@tag($this:ident, $tag:ident) in:[$t:ident=$t2:ident $($i:tt)*] fields:[$(($f:ident: $v:expr)),*] ) => {
        $crate::bsml!(@tag($this, $tag) in:[$($i)*] fields:[$(($f: $v),)* ($t: $t2)])
    };
    (@tag($this:ident, $tag:ident) in:[$t:ident $($i:tt)*] fields:[$(($f:ident: $v:expr)),*] ) => {
        $crate::bsml!(@tag($this, $tag) in:[$($i)*] fields:[$(($f: $v),)* ($t: $t)])
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
}
