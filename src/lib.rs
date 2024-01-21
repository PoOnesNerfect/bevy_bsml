use bevy::{
    ecs::system::{Command, EntityCommands},
    prelude::{despawn_with_children_recursive, App, Commands, Component, Entity, Plugin},
    reflect::Reflect,
};
use class_list::ClassListMap;

pub use bevy;
// used for solving hygiene issue when trying to access `self` in macro rules
#[doc(hidden)]
pub use replace_ident::replace_ident;

pub mod class;
pub mod class_list;
pub use class_list::{
    BackgroundColorClassList, BorderColorClassList, StyleClassList, TextClassList, ZIndexClassList,
};

/// Contains all the items needed to use Bsml
pub mod prelude {
    pub use crate::{
        bsml,
        class::{
            background_color::*, border_color::*, flexbox_grid::align_items::*,
            flexbox_grid::flex_direction::*, flexbox_grid::flex_wrap::*, flexbox_grid::gap::*,
            flexbox_grid::justify_content::*, hovered, pressed, sizing::*, text::*, z_index::*,
        },
        class_list::*,
        BsmlPlugin, SpawnBsml,
    };
    pub use bevy::prelude::default;
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

pub trait SpawnBsml<'w, 's> {
    fn spawn_bsml<'a, T: Bsml>(&'a mut self, node: T) -> EntityCommands<'w, 's, 'a>;
    fn despawn_bsml(&mut self, entity: Entity);
}

impl<'w, 's> SpawnBsml<'w, 's> for bevy::ecs::system::Commands<'w, 's> {
    fn spawn_bsml<'a, T: Bsml>(&'a mut self, node: T) -> EntityCommands<'w, 's, 'a> {
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

pub trait Bsml {
    fn spawn(self, commands: &mut Commands, slot: &[Entity]) -> Entity;

    fn taking_slot(&self) -> bool {
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
                let __entity = $crate::bsml!(@spawn(this, commands, slot) ($itag $($attr)*) $({$($content)*})?);
                commands.entity(__entity).insert(self);

                __entity
            }

            $crate::bsml!(@taking_slot ($itag) $({$($content)*})?);
        }
    };
    // handle node tag
    (@spawn($this:ident, $commands:ident, $slot:ident) (node $(labels=[$($label:expr),* $(,)?])? $(class=[$($class:expr),* $(,)?])?) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        #[allow(unused_mut)]
        let mut __bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        #[allow(unused_mut)]
        let mut __class_map = $crate::class_list::ClassListMap::default();

        let labels = ($($(
            $crate::replace_ident!(self, $this, $label),
        )*)?);
        let labels_ref = &labels;

        $({
            use $crate::class::WithInteraction;
            $(
                let (__interaction, mut __class) = $crate::replace_ident!(labels, labels_ref, $crate::replace_ident!(self, $this, $class)).with_interaction();

                if __interaction == $crate::bevy::ui::Interaction::None {
                    __class.apply_to_node_bundle(&mut __bundle);
                }

                __class_map.insert(__interaction, __class);
            )*
        })?

        let __entity = $commands.spawn((
            __bundle,
            $crate::bevy::ui::Interaction::None,
            $crate::BsmlNode,
            labels
        ))
        .id();

        __class_map.spawn($commands, __entity);

        let __children = [$($(
            $crate::bsml!(@spawn($this, $commands, $slot) ($($def)+) $({$($imp)*})?),
        )*)?];

        $commands.entity(__entity).push_children(&__children);

        __entity
    }};
    // handle for tag with enumeration
    (@spawn($this:ident, $commands:ident, $slot:ident) (for {$i:ident, $v:ident in $iter:expr} $(labels=[$($label:expr),* $(,)?])? $(class=[$($class:expr),* $(,)?])?) {$(($($def:tt)+) $({$($imp:tt)*})?)*}) => {{
        #[allow(unused_mut)]
        let mut __bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        #[allow(unused_mut)]
        let mut __class_map = $crate::class_list::ClassListMap::default();

        let labels = ($($(
            $crate::replace_ident!(self, $this, $label),
        )*)?);
        let labels_ref = &labels;

        $({
            use $crate::class::WithInteraction;
            $(
                let (__interaction, mut __class) = $crate::replace_ident!(labels, labels_ref, $crate::replace_ident!(self, $this, $class)).with_interaction();

                if __interaction == $crate::bevy::ui::Interaction::None {
                    __class.apply_to_node_bundle(&mut __bundle);
                }

                __class_map.insert(__interaction, __class);
            )*
        })?

        let __entity = $commands.spawn((
            __bundle,
            $crate::bevy::ui::Interaction::None,
            $crate::BsmlNode,
            labels,
        ))
        .id();

        __class_map.spawn($commands, __entity);

        for ($i, $v) in ($crate::replace_ident!(self, $this, $iter)).into_iter().enumerate() {
            let __children = [$(
                $crate::bsml!(@spawn($this, $commands, $slot) ($($def)+) $({$($imp)*})?),
            )*];

            $commands.entity(__entity).push_children(&__children);
        }

        __entity
    }};
    // handle for tag
    (@spawn($this:ident, $commands:ident, $slot:ident) (for {$v:ident in $iter:expr} $(labels=[$($label:expr),* $(,)?])? $(class=[$($class:expr),* $(,)?])?) {$(($($def:tt)+) $({$($imp:tt)*})?)*}) => {{
        #[allow(unused_mut)]
        let mut __bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        #[allow(unused_mut)]
        let mut __class_map = $crate::class_list::ClassListMap::default();

        let labels = ($($(
            $crate::replace_ident!(self, $this, $label),
        )*)?);
        let labels_ref = &labels;

        $({
            use $crate::class::WithInteraction;
            $(
                let (__interaction, mut __class) = $crate::replace_ident!(labels, labels_ref, $crate::replace_ident!(self, $this, $class)).with_interaction();

                if __interaction == $crate::bevy::ui::Interaction::None {
                    __class.apply_to_node_bundle(&mut __bundle);
                }

                __class_map.insert(__interaction, __class);
            )*
        })?

        let __entity = $commands.spawn((
            __bundle,
            $crate::bevy::ui::Interaction::None,
            $crate::BsmlNode,
        ))
        .id();

        __class_map.spawn($commands, __entity);

        for $v in $crate::replace_ident!(labels, labels_ref, $crate::replace_ident!(self, $this, $iter)) {
            let __children = [$(
                $crate::bsml!(@spawn($this, $commands, $slot) ($($def)+) $({$($imp)*})?),
            )*];

            $commands.entity(__entity).push_children(&__children);
        }

        $commands.entity(__entity).insert(labels);

        __entity
    }};
    // handle text tag
    (@spawn($this:ident, $commands:ident, $slot:ident) (text $(labels=[$($label:expr),* $(,)?])? $(class=[$($class:expr),* $(,)?])?) {$literal:expr $(,$value:expr)*}) => {{
        #[allow(unused_mut)]
        let mut __class_map = $crate::class_list::ClassListMap::default();

        let labels = ($($(
            $crate::replace_ident!(self, $this, $label),
        )*)?);
        let labels_ref = &labels;

        let mut __bundle = $crate::bevy::prelude::TextBundle::from_section(
            format!($literal $(, $crate::replace_ident!(labels, labels_ref, $crate::replace_ident!(self, $this, $value)))*),
            $crate::bevy::text::TextStyle::default()
        );

        $({
            use $crate::class::WithInteraction;
            $(
                let (__interaction, mut __class) = $class.with_interaction();

                if __interaction == $crate::bevy::ui::Interaction::None {
                    __class.apply_to_text_bundle(&mut __bundle);
                }

                __class_map.insert(__interaction, __class);
            )*
        })?

        let __entity = $commands.spawn((
            __bundle,
            $crate::bevy::ui::Interaction::None,
            $crate::BsmlNode,
            labels,
        ))
        .id();

        __class_map.spawn($commands, __entity);

        __entity
    }};
    // handle slot tag
    (@spawn($this:ident, $commands:ident, $slot:ident) (slot $(labels=[$($label:expr),* $(,)?])? $(class=[$($class:expr),* $(,)?])?) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        #[allow(unused_mut)]
        let mut __bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        #[allow(unused_mut)]
        let mut __class_map = $crate::class_list::ClassListMap::default();

        let labels = ($($(
            $crate::replace_ident!(self, $this, $label),
        )*)?);
        let labels_ref = &labels;

        $({
            use $crate::class::WithInteraction;
            $(
                let (__interaction, mut __class) = $crate::replace_ident!(labels, labels_ref, $crate::replace_ident!(self, $this, $class)).with_interaction();

                if __interaction == $crate::bevy::ui::Interaction::None {
                    __class.apply_to_node_bundle(&mut __bundle);
                }

                __class_map.insert(__interaction, __class);
            )*
        })?

        let __entity = $commands.spawn((
            __bundle,
            $crate::bevy::ui::Interaction::None,
            $crate::BsmlNode,
            labels,
        ))
        .id();

        __class_map.spawn($commands, __entity);

        if $slot .is_empty() {
            let __children = [
                $($(
                    $crate::bsml!(@spawn($this, $commands, $slot) ($($def)+) $({$($imp)*})?),
                )*)?
            ];
            $commands.entity(__entity).push_children(&__children);
        } else {
            $commands.entity(__entity).push_children(&$slot);
        }

        __entity
    }};
    // handle custom component
    (@spawn($this:ident, $commands:ident, $_slot:ident) ($component:expr) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        let __tag = $crate::replace_ident!(self, $this, $component);

        let __entity = if __tag.taking_slot() {
            let __children = [$($(
                $crate::bsml!(@spawn($this, $commands, $_slot) ($($def)+) $({$($imp)*})?),
            )*)?];

            __tag.spawn($commands, &__children)
        } else {

            __tag.spawn($commands, &[])
        };

        __entity
    }};
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
