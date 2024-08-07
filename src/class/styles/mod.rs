use crate::class::impl_class;
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

impl_class!(StyleClass -> Style {
    Width(Width),
    MinWidth(MinWidth),
    MaxWidth(MaxWidth),
    Height(Height),
    AspectRatio(AspectRatio),
    PositionType(PositionType),
    Top(Top),
    Bottom(Bottom),
    Right(Right),
    Left(Left),
    JustifyContent(JustifyContent),
    AlignContent(AlignContent),
    AlignItems(AlignItems),
    FlexDirection(FlexDirection),
    FlexWrap(FlexWrap),
    Gap(Gap),
    RowGap(RowGap),
    ColGap(ColGap)
});
