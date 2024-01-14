pub use bevy;
pub use bevy::ui::Val;
use bevy::{
    prelude::{App, Commands, Component, Entity, Plugin},
    reflect::Reflect,
};

pub mod class;

mod map;
pub use map::*;

mod systems;
use systems::{
    apply_background_color_class_system, apply_border_color_class_system, apply_style_class_system,
    apply_text_class_system, apply_z_index_class_system,
};

#[derive(Debug, Clone, Copy)]
pub struct BsmlPlugin;

impl Plugin for BsmlPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StyleClassMap>();
        app.init_resource::<BackgroundColorClassMap>();
        app.init_resource::<BorderColorClassMap>();
        app.init_resource::<ZIndexClassMap>();
        app.init_resource::<TextClassMap>();

        app.add_systems(bevy::prelude::Update, apply_style_class_system);
        app.add_systems(bevy::prelude::Update, apply_border_color_class_system);
        app.add_systems(bevy::prelude::Update, apply_background_color_class_system);
        app.add_systems(bevy::prelude::Update, apply_z_index_class_system);
        app.add_systems(bevy::prelude::Update, apply_text_class_system);
    }
}

pub trait BsmlCommand {
    fn spawn_bsml<T: Bsml>(&mut self, node: T) -> Entity;
}
impl<'w, 's> BsmlCommand for bevy::ecs::system::Commands<'w, 's> {
    fn spawn_bsml<T: Bsml>(&mut self, node: T) -> Entity {
        node.spawn(self, &[])
    }
}

pub trait Bsml {
    fn spawn(&self, commands: &mut Commands, slot: &[Entity]) -> Entity;

    fn taking_slot() -> bool {
        false
    }
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct BsmlNode;

#[macro_export]
macro_rules! bsml {
    (($tag:ident $({$f:ident : $t:ty $(, $f2:ident : $t2: ty)* $(,)?})* $(class=[$($class:expr),* $(,)?])?) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {
        $crate::bsml!(@struct_def $tag {$($f : $t $(, $f2 : $t2)*),*});

        impl $crate::Bsml for $tag {
            #[allow(unused_variables)]
            fn spawn(&self, commands: &mut $crate::bevy::ecs::system::Commands, slot: &[$crate::bevy::ecs::entity::Entity]) -> $crate::bevy::ecs::entity::Entity {
                #[allow(unused_imports)]
                use $crate::{class::ApplyClass, InsertEntityClassMaps};

                #[allow(unused_mut)]
                let mut bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

                $($(
                    $crate::class::apply_class_to_node_bundle(&mut bundle, $crate::bevy::ui::Interaction::None, $class);
                )*)?

                let parent = commands.spawn((self.clone(), bundle, $crate::bevy::ui::Interaction::None, $crate::BsmlNode)).id();

                #[allow(unused_mut)]
                let mut entity_class_maps = $crate::EntityClassMaps::default();

                $($(
                    $class .insert_entity_class_maps(parent, &mut entity_class_maps);
                )*)?

                $($({
                    use $crate::bevy::hierarchy::BuildChildren;

                    let child = $crate::bsml!(@spawn(self, commands, slot, entity_class_maps) ($($def)+) $({$($imp)*})?);
                    commands.entity(parent).add_child(child);
                })*)?

                entity_class_maps.sync_resources(commands);

                parent
            }

            $($(
                $crate::bsml!(@taking_slot ($($def)+) $({$($imp)*})?);
            )*)?
        }
    };
    // handle node tag
    (@spawn($this:ident, $commands:ident, $slot:ident, $class_maps:ident) (node $(class=[$($class:expr),* $(,)?])?) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        #[allow(unused_mut)]
        let mut bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        $($(
            $crate::class::apply_class_to_node_bundle(&mut bundle, $crate::bevy::ui::Interaction::None, $class);
        )*)?

        let node = $commands.spawn((bundle, $crate::bevy::ui::Interaction::None, $crate::BsmlNode)).id();

        $($(
            $class .insert_entity_class_maps(node, &mut $class_maps);
        )*)?

        let children = [$($(
            $crate::bsml!(@spawn($this, $commands, $slot, $class_maps) ($($def)+) $({$($imp)*})?),
        )*)?];
        $commands.entity(node).push_children(&children);

        node
    }};
    // handle text tag
    (@spawn($this:ident, $commands:ident, $slot:ident, $class_maps:ident) (text $(class=[$($class:expr),* $(,)?])?) {$($words:tt)+}) => {{
        let mut bundle = $crate::bevy::prelude::TextBundle::from_section($crate::bsml!(@stringify($this) in: [$($words)+], out: [], fields: []), $crate::bevy::text::TextStyle::default());

        $($(
            $crate::class::apply_class_to_text_bundle(&mut bundle, $crate::bevy::ui::Interaction::None, $class);
        )*)?

        let id = $commands.spawn((bundle, $crate::bevy::ui::Interaction::None, $crate::BsmlNode)).id();

        $($(
            $class .insert_entity_class_maps(id, &mut $class_maps);
        )*)?

        id
    }};
    // handle slot tag
    (@spawn($this:ident, $commands:ident, $slot:ident, $class_maps:ident) (slot) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        #[allow(unused_mut)]
        let mut bundle = $crate::bevy::ui::node_bundles::NodeBundle::default();

        let parent = $commands.spawn((bundle, $crate::bevy::ui::Interaction::None, $crate::BsmlNode)).id();

        if $slot .is_empty() {
            let children = [
                $($(
                    $crate::bsml!(@spawn($this, $commands, $slot, $class_maps) ($($def)+) $({$($imp)*})?),
                )*)?
            ];
            $commands.entity(parent).push_children(&children);
        } else {
            $commands.entity(parent).push_children(&$slot);
        }

        parent
    }};
    // handle custom component
    (@spawn($this:ident, $commands:ident, $_slot:ident, $class_maps:ident) ($itag:ident $($attr:tt)*) $({$(($($def:tt)+) $({$($imp:tt)*})?)*})?) => {{
        let tag = $crate::bsml!(@tag($this, $itag) in:[$($attr)*] fields:[]);

        let parent = if <$itag as $crate::Bsml> :: taking_slot() {
            let children = [$($(
                $crate::bsml!(@spawn($this, $commands, $_slot, $class_maps) ($($def)+) $({$($imp)*})?),
            )*)?];

            tag.spawn($commands, &children)
        } else {
            tag.spawn($commands, &[])
        };


        parent
    }};
    // parse attribute tokens into fields, and populate to initialize component
    (@tag($this:ident, $tag:ident) in:[] fields:[] ) => {
        $tag
    };
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
    (@taking_slot (slot) $($_:tt)?) => {
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
    (@stringify($this:ident) in: [], out: [$($out:tt)*], fields: [$($f:ident)*]) => {
        format!(stringify!($($out)*), $($this . $f),*)
    };
    (@stringify($this:ident) in: [{$n:ident} $($in:tt)*], out: [$($out:tt)*], fields: [$($f:ident)*]) => {
        $crate::bsml!(@stringify($this) in: [$($in)*], out: [$($out)* {}], fields: [$($f)* $n])
    };
    (@stringify($this:ident) in: [$n:tt $($in:tt)*], out: [$($out:tt)*], fields: [$($f:ident)*]) => {
        $crate::bsml!(@stringify($this) in: [$($in)*], out: [$($out)* $n], fields: [$($f)*])
    };
}
