use crate::class::impl_class;
use derive_more::From;

pub mod border;
pub mod direction;
pub mod flexbox_grid;
pub mod layout;
pub mod overflow;
pub mod sizing;
pub mod spacing;

pub(super) mod styles_prelude {
    pub use super::border::*;
    pub use super::direction::*;
    pub use super::flexbox_grid::prelude::*;
    pub use super::layout::prelude::*;
    pub use super::overflow::*;
    pub use super::sizing::prelude::*;
    pub use super::spacing::prelude::*;

    pub use bevy_ui::Style;
}
use styles_prelude::*;

impl_class!(StyleClass -> Style {
    // Sizing
    Width(Width),
    MinWidth(MinWidth),
    MaxWidth(MaxWidth),
    MinHeight(MinHeight),
    Height(Height),

    // Flexbox Grid
    FlexBasis(FlexBasis),
    FlexGrow(FlexGrow),
    FlexShrink(FlexShrink),
    FlexDirection(FlexDirection),
    FlexWrap(FlexWrap),
    Gap(Gap),
    RowGap(RowGap),
    ColGap(ColGap),
    JustifyContent(JustifyContent),
    JustifyItems(JustifyItems),
    JustifySelf(JustifySelf),
    AlignContent(AlignContent),
    AlignItems(AlignItems),
    AlignSelf(AlignSelf),
    GridAutoFlow(GridAutoFlow),
    GridTemplateRows(GridTemplateRows),
    GridTemplateColumns(GridTemplateColumns),
    GridAutoRows(GridAutoRows),
    GridAutoCols(GridAutoCols),
    GridRow(GridRow),
    GridCol(GridCol),

    // Layout
    Display(Display),
    Direction(Direction),
    Overflow(Overflow),
    OverflowX(OverflowX),
    OverflowY(OverflowY),
    AspectRatio(AspectRatio),
    PositionType(PositionType),
    Top(Top),
    Bottom(Bottom),
    Right(Right),
    Left(Left),

    // Spacing
    Margin(Margin),
    MarginX(MarginX),
    MarginY(MarginY),
    MarginTop(MarginTop),
    MarginBottom(MarginBottom),
    MarginLeft(MarginLeft),
    MarginRight(MarginRight),
    Padding(Padding),
    PaddingX(PaddingX),
    PaddingY(PaddingY),
    PaddingTop(PaddingTop),
    PaddingBottom(PaddingBottom),
    PaddingLeft(PaddingLeft),
    PaddingRight(PaddingRight),
    Border(Border),
    BorderX(BorderX),
    BorderY(BorderY),
    BorderTop(BorderTop),
    BorderBottom(BorderBottom),
    BorderLeft(BorderLeft),
    BorderRight(BorderRight)
});
