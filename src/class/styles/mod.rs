use crate::class::ApplyClass;
use derive_more::From;

pub mod flexbox_grid;
pub mod layout;
pub mod sizing;

pub(super) mod styles_prelude {
    pub use super::flexbox_grid::prelude::*;
    pub use super::layout::prelude::*;
    pub use super::sizing::prelude::*;
    pub use bevy::ui::{
        AlignContent, AlignItems, FlexDirection, FlexWrap, JustifyContent, PositionType, Style,
    };
}
use styles_prelude::*;

impl_style_class!(
    Width,
    MinWidth,
    MaxWidth,
    Height,
    AspectRatio,
    PositionType,
    Top,
    Bottom,
    Right,
    Left,
    JustifyContent,
    AlignContent,
    AlignItems,
    FlexDirection,
    FlexWrap,
    Gap,
    RowGap,
    ColGap
);

use self::macros::impl_style_class;
mod macros {
    macro_rules! impl_style_class {
        ($($f:ident),*) => {
            #[doc(hidden)]
            #[derive(Debug, Clone, From, PartialEq)]
            pub enum StyleClass {
                $($f($f)),*
            }

            impl ApplyClass<StyleClass> for Style {
                fn apply_class(&mut self, class: &StyleClass) {
                    match class {
                        $(StyleClass::$f(val) => self.apply_class(val)),*
                    }
                }
            }

            $(
                impl From<$f> for super::BsmlClass {
                    fn from(val: $f) -> Self {
                        Self::Style(StyleClass::$f(val))
                    }
                }
            )*
        };
    }

    pub(super) use impl_style_class;
}
