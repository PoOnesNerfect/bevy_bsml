use self::{
    flexbox_grid::gap::*,
    layout::AspectRatio,
    macros::impl_style_class,
    sizing::{Height, MinWidth, Width},
};
use crate::class::ApplyClass;
use bevy::ui::{AlignContent, AlignItems, FlexDirection, FlexWrap, JustifyContent, Style};
use derive_more::From;

pub mod flexbox_grid;
pub mod layout;
pub mod sizing;

impl_style_class!(
    Width,
    MinWidth,
    Height,
    AspectRatio,
    JustifyContent,
    AlignContent,
    AlignItems,
    FlexDirection,
    FlexWrap,
    Gap,
    RowGap,
    ColGap
);

mod macros {
    macro_rules! impl_style_class {
        ($($f:ident),*) => {
            #[derive(Debug, Clone, From)]
            pub enum StyleClass {
                $($f($f)),*
            }

            impl ApplyClass for StyleClass {
                type Component = Style;

                fn apply_class(&self, component: &mut Self::Component) {
                    match self {
                        $(Self::$f(val) => val.apply_class(component)),*
                    }
                }
            }
        };
    }

    pub(super) use impl_style_class;
}
