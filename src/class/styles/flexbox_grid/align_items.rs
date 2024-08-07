use crate::class::ApplyClass;
use bevy_ui::Style;

pub use bevy_ui::AlignItems;

/// The items are packed in their default position as if no alignment was applied.
pub const ITEMS_DEFAULT: AlignItems = AlignItems::Default;
/// The items are packed towards the start of the axis, unless the flex direction is reversed;
/// then they are packed towards the end of the axis.
pub const ITEMS_START: AlignItems = AlignItems::FlexStart;
/// The items are packed towards the end of the axis, unless the flex direction is reversed;
/// then they are packed towards the start of the axis.
pub const ITEMS_END: AlignItems = AlignItems::FlexEnd;
/// The items are packed along the center of the axis.
pub const ITEMS_CENTER: AlignItems = AlignItems::Center;
/// The items are packed such that their baselines align.
pub const ITEMS_BASELINE: AlignItems = AlignItems::Baseline;
/// The items are stretched to fill the space they're given.
pub const ITEMS_STRETCH: AlignItems = AlignItems::Stretch;

impl ApplyClass<AlignItems> for Style {
    fn apply_class(&mut self, class: &AlignItems) {
        self.align_items = *class;
    }
}
