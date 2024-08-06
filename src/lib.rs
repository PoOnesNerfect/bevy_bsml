use bevy::{
    ecs::{system::EntityCommands, world::Command},
    prelude::*,
};

#[doc(hidden)]
pub use bevy;

// used for solving hygiene issue when trying to access `self` in macro rules
#[doc(hidden)]
pub use replace_ident::replace_ident;

pub mod class;
mod class_list;

pub use class_list::BsmlClasses;

/// Contains all the items needed to use Bsml
pub mod prelude {
    pub use crate::class::prelude::*;
    pub use crate::{bsml, BsmlClasses, BsmlPlugin, SpawnBsml};
}

#[derive(Debug, Clone, Copy)]
pub struct BsmlPlugin;

impl Plugin for BsmlPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            PostUpdate,
            class_list::apply_class_system.before(bevy::ui::UiSystem::Layout),
        );
    }

    fn is_unique(&self) -> bool {
        true
    }
}

pub trait SpawnBsml {
    fn spawn_bsml<'a, T: BsmlElement>(&'a mut self, node: T) -> EntityCommands<'a>;
    fn despawn_bsml(&mut self, entity: Entity);
}

impl<'w, 's> SpawnBsml for bevy::ecs::system::Commands<'w, 's> {
    fn spawn_bsml<'a, T: BsmlElement>(&'a mut self, node: T) -> EntityCommands<'a> {
        let entity = node.spawn(self, &[]);
        self.entity(entity)
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

pub trait BsmlElement {
    fn spawn(self, commands: &mut Commands, slot: &[Entity]) -> Entity;

    fn taking_slot(&self) -> bool {
        false
    }
}

/// Marker component for bsml node
#[derive(Debug, Clone, Component, Reflect)]
pub struct Bsml;

#[macro_export]
macro_rules! bsml {
    (($itag:ident $($attr:tt)*) $({$($content:tt)*})?) => {{
        #[derive($crate::bevy::prelude::Component, Clone, Copy)]
        pub struct __BsmlTag;
        $crate::bsml!(__BsmlTag; ($itag $($attr)*) $({$($content)*})?);
        __BsmlTag
    }};
    ($tag:ident; ($itag:ident $($attr:tt)*) $({$($content:tt)*})?) => {
        impl $crate::BsmlElement for $tag {
            #[allow(unused_variables)]
            fn spawn(self, commands: &mut $crate::bevy::ecs::system::Commands, slot: &[$crate::bevy::ecs::entity::Entity]) -> $crate::bevy::ecs::entity::Entity {
                #[allow(unused_imports)]
                use $crate::class::ApplyClass;
                use $crate::bevy::hierarchy::BuildChildren;

                let this = &self;
                let __entity = $crate::bsml!(@element(this, commands, slot) ($itag $($attr)*) $({$($content)*})?);
                commands.entity(__entity).insert(self);

                __entity
            }

            $crate::bsml!(@taking_slot ($itag) $({$($content)*})?);
        }
    };
    // handle (node) element
    (@element($this:ident, $commands:ident, $slot:ident) (node $($attr:tt)*) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        #[allow(unused_mut)]
        let mut __bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        let __entity = $crate::bsml!(@spawn($this, $commands, $slot, __bundle) $($attr)*);

        let __children = [$($(
            $crate::bsml!(@element($this, $commands, $slot) ($($def)+) $({$($imp)*})?),
        )*)?];

        $commands.entity(__entity).push_children(&__children);

        __entity
    }};
    // handle (for) element with enumeration
    (@element($this:ident, $commands:ident, $slot:ident) (for {$i:ident, $v:ident in $iter:expr} $($attr:tt)*) {$(($($def:tt)+) $({$($imp:tt)*})?)*}) => {{
        #[allow(unused_mut)]
        let mut __bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        let __entity = $crate::bsml!(@spawn($this, $commands, $slot) $($attr)*);

        for ($i, $v) in ($crate::replace_ident!(self, $this, $iter)).into_iter().enumerate() {
            let __children = [$(
                $crate::bsml!(@element($this, $commands, $slot) ($($def)+) $({$($imp)*})?),
            )*];

            $commands.entity(__entity).push_children(&__children);
        }

        __entity
    }};
    // handle (for) element
    (@element($this:ident, $commands:ident, $slot:ident) (for {$v:ident in $iter:expr} $($attr:tt)*) {$(($($def:tt)+) $({$($imp:tt)*})?)*}) => {{
        #[allow(unused_mut)]
        let mut __bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        let __entity = $crate::bsml!(@spawn($this, $commands, $slot, __bundle) $($attr)*);

        for $v in $crate::replace_ident!(labels, labels_ref, $crate::replace_ident!(self, $this, $iter)) {
            let __children = [$(
                $crate::bsml!(@element($this, $commands, $slot) ($($def)+) $({$($imp)*})?),
            )*];

            $commands.entity(__entity).push_children(&__children);
        }

        __entity
    }};
    // handle (text) element
    (@element($this:ident, $commands:ident, $slot:ident) (text $($attr:tt)*) {$literal:expr $(,$value:expr)*}) => {{
        let mut __bundle = $crate::bevy::prelude::TextBundle::from_section(
            format!($literal $(, $crate::replace_ident!(labels, labels_ref, $crate::replace_ident!(self, $this, $value)))*),
            $crate::bevy::text::TextStyle::default()
        );

        $crate::bsml!(@spawn($this, $commands, $slot, __bundle) $($attr)*)
    }};
    // handle (slot) element
    (@element($this:ident, $commands:ident, $slot:ident) (slot $($attr:tt)*) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        #[allow(unused_mut)]
        let mut __bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        let __entity = $crate::bsml!(@spawn($this, $commands, $slot, __bundle) $($attr)*);

        if $slot .is_empty() {
            let __children = [
                $($(
                    $crate::bsml!(@element($this, $commands, $slot) ($($def)+) $({$($imp)*})?),
                )*)?
            ];
            $commands.entity(__entity).push_children(&__children);
        } else {
            $commands.entity(__entity).push_children(&$slot);
        }

        __entity
    }};
    // handle (custom) component
    (@element($this:ident, $commands:ident, $_slot:ident) ($component:expr) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        let __tag = $crate::replace_ident!(self, $this, $component);

        let __entity = if __tag.taking_slot() {
            let __children = [$($(
                $crate::bsml!(@element($this, $commands, $_slot) ($($def)+) $({$($imp)*})?),
            )*)?];

            __tag.spawn($commands, &__children)
        } else {
            __tag.spawn($commands, &[])
        };

        __entity
    }};
    // handle attributes, spawn entity and return entity id
    (@spawn($this:ident, $commands:ident, $slot:ident, $bundle:ident) $(labels=[$($label:expr),* $(,)?])? $(class=[$($class:expr),* $(,)?])?) => {{
        #[allow(unused_mut)]
        let mut __class_map = $crate::BsmlClasses::default();

        let labels = ($($(
            $crate::replace_ident!(self, $this, $label),
        )*)?);
        let labels_ref = &labels;

        $({
            use $crate::class::WithInteraction;
            $(
                let (__interaction, mut __class) = $crate::replace_ident!(labels, labels_ref, $crate::replace_ident!(self, $this, $class)).with_interaction();

                if __interaction == $crate::bevy::ui::Interaction::None {
                    $bundle .apply_class(&__class);
                }

                __class_map.insert(__interaction, __class);
            )*
        })?

        let __entity = $commands.spawn((
            $bundle,
            $crate::bevy::ui::Interaction::None,
            $crate::Bsml,
            labels,
        ))
        .insert(__class_map)
        .id();

        __entity
    }};
    // handle cases where order of attributes is reversed
    (@spawn($this:ident, $commands:ident, $slot:ident, $bundle:ident) class=[$($class:expr),* $(,)?] labels=[$($label:expr),* $(,)?]) => {
        $crate::bsml!(@spawn($this, $commands, $slot, $bundle) labels=[$($label),*] class=[$($class),*])
    };
    // impl taking_slot (slot) exists
    (@taking_slot (slot $($_:tt)*) $($_1:tt)?) => {
        fn taking_slot(&self) -> bool {
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
