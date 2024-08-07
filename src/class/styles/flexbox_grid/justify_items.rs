pub use bevy_ui::JustifyItems;
use bevy_ui::Style;

use crate::class::ApplyClass;

/// The items are packed in their default position as if no alignment was applied.
pub const JUSTIFY_ITEMS_DEFAULT: JustifyItems = JustifyItems::Default;
/// The items are packed towards the start of the axis.
pub const JUSTIFY_ITEMS_START: JustifyItems = JustifyItems::Start;
/// The items are packed towards the end of the axis.
pub const JUSTIFY_ITEMS_END: JustifyItems = JustifyItems::End;
/// The items are packed along the center of the axis
pub const JUSTIFY_ITEMS_CENTER: JustifyItems = JustifyItems::Center;
/// The items are packed such that their baselines align.
pub const JUSTIFY_ITEMS_BASELINE: JustifyItems = JustifyItems::Baseline;
/// The items are stretched to fill the space they're given.
pub const JUSTIFY_ITEMS_STRETCH: JustifyItems = JustifyItems::Stretch;

impl ApplyClass<JustifyItems> for Style {
    #[inline]
    fn apply_class(&mut self, class: &JustifyItems) {
        self.justify_items = *class;
    }
}
