use bevy::prelude::*;
use derive_more::From;

mod text;
pub use text::*;

mod styles;
pub use styles::*;

pub mod background_color;
pub mod border_color;
pub mod border_radius;
pub mod focus_policy;
pub mod visibility;
pub mod z_index;

pub(super) mod prelude {
    pub use super::hovered;
    pub use super::pressed;

    pub use super::styles_prelude::*;
    pub use super::text_prelude::*;

    pub use super::background_color::*;
    pub use super::border_color::*;
    pub use super::border_radius::*;
    pub use super::focus_policy::*;
    pub use super::visibility::*;
    pub use super::z_index::*;
}
use prelude::*;

pub fn hovered<C>(class: C) -> (Interaction, C) {
    (Interaction::Hovered, class)
}

pub fn pressed<C>(class: C) -> (Interaction, C) {
    (Interaction::Pressed, class)
}

impl_bsml_class!(
    (enum) Style(StyleClass),
    (enum) Text(TextClass),
    BackgroundColor(BackgroundColorClass),
    BorderColor(BorderColorClass),
    BorderRadius(BorderRadiusClass),
    Visibility(Visibility),
    ZIndex(ZIndex),
    FocusPolicy(FocusPolicy)
);

apply_class_to_bundle!(NodeBundle {
    style: Style,
    background_color: BackgroundColor,
    border_color: BorderColor,
    border_radius: BorderRadius,
    visibility: Visibility,
    z_index: ZIndex,
    focus_policy: FocusPolicy
});

apply_class_to_bundle!(TextBundle {
    style: Style,
    text: Text,
    background_color: BackgroundColor,
    visibility: Visibility,
    z_index: ZIndex,
    focus_policy: FocusPolicy
});

apply_class_to_bundle!(ImageBundle {
    style: Style,
    background_color: BackgroundColor,
    visibility: Visibility,
    z_index: ZIndex,
    focus_policy: FocusPolicy
});

apply_class_to_bundle!(MaterialNodeBundle<M: UiMaterial> {
    style: Style,
    visibility: Visibility,
    z_index: ZIndex,
    focus_policy: FocusPolicy
});

#[doc(hidden)]
pub trait WithInteraction {
    fn with_interaction(self) -> (Interaction, BsmlClass);
}

impl<T: Into<BsmlClass>> WithInteraction for (Interaction, T) {
    fn with_interaction(self) -> (Interaction, BsmlClass) {
        (self.0, self.1.into())
    }
}

impl<T: Into<BsmlClass>> WithInteraction for T {
    fn with_interaction(self) -> (Interaction, BsmlClass) {
        (Interaction::None, self.into())
    }
}

/// A tailwind class that can be applied to a UI component.
#[doc(hidden)]
pub trait ApplyClass<Class> {
    fn apply_class(&mut self, class: &Class);
}

use self::macros::*;
mod macros {
    macro_rules! impl_bsml_class {
        ($($(($enum:ident))? $component:ident ($class:ident)),*) => {
            /// A class that can be applied to a UI node.
            #[doc(hidden)]
            #[derive(Clone, Debug, From, PartialEq)]
            pub enum BsmlClass {
                $(
                    $component ($class)
                ),+
            }

            impl BsmlClass {
                /// Check if the class is of same type to another class.
                #[allow(unused_variables)]
                #[inline]
                pub(crate) fn eq_class_type(&self, other: &Self) -> bool {
                    match (self, other) {
                        $(
                            (BsmlClass:: $component (a), BsmlClass:: $component (b)) => impl_bsml_class!(@eq $($enum)? a b),
                        )*
                        _ => false
                    }
                }
            }

            $(
                impl ApplyClass<BsmlClass> for $component {
                    #[inline]
                    fn apply_class(&mut self, class: &BsmlClass) {
                        match class {
                            BsmlClass::$component(class) => self.apply_class(class),
                            _ => {}
                        }
                    }
                }
            )*
        };
        (@eq enum $a:ident $b:ident) => { std::mem::discriminant($a) == std::mem::discriminant($b) };
        (@eq $a:ident $b:ident) => { true };
    }
    pub(super) use impl_bsml_class;

    macro_rules! apply_class_to_bundle {
        ($bundle:ident $(<$gen:ident $(: $trait:ident)?>)? {$($v:ident: $class:ident),*}) => {
            impl $(<$gen $(: $trait)?>)? ApplyClass<BsmlClass> for $bundle $(<$gen>)? {
                #[inline]
                fn apply_class(&mut self, class: &BsmlClass) {
                    match class {
                        $(
                            BsmlClass::$class(val) => self.$v.apply_class(val),
                        )*
                        _ => {}
                    }
                }
            }
        };
    }
    pub(super) use apply_class_to_bundle;

    macro_rules! impl_class {
        ($class:ident -> $component:ident {$($v:ident ($t:ty)),*}) => {
            #[doc(hidden)]
            #[derive(Debug, Clone, From, PartialEq)]
            pub enum $class  {
                $($v($t)),*
            }

            $(
                impl From<$t> for crate::class::BsmlClass {
                    #[inline]
                    fn from(val: $t) -> Self {
                        Self::$component($class ::$v(val))
                    }
                }
            )*

            impl crate::class::ApplyClass<$class> for $component {
                #[inline]
                fn apply_class(&mut self, class: &$class) {
                    match class {
                        $(
                            $class ::$v(val) => self.apply_class(val),
                        )*
                    }
                }
            }
        };
    }
    pub(super) use impl_class;
}
