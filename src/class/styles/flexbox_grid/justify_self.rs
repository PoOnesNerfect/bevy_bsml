//! Used to control how the specified item is aligned within the space it's given.
//! - For Flexbox items, this property has no effect. See `justify_content` for main axis alignment of flex items.
//! - For CSS Grid items, controls inline (horizontal) axis alignment of a grid item within its grid area.
//!
//! <https://developer.mozilla.org/en-US/docs/Web/CSS/justify-self>

use crate::class::ApplyClass;
use bevy::ui::Style;

pub use bevy::ui::JustifySelf;

/// Use the parent node's [`JustifyItems`] value to determine how this item should be aligned.
pub const JUSTIFY_SELF_AUTO: JustifySelf = JustifySelf::Auto;
/// This item will be aligned with the start of the axis.
pub const JUSTIFY_SELF_START: JustifySelf = JustifySelf::Start;
/// This item will be aligned with the end of the axis.
pub const JUSTIFY_SELF_END: JustifySelf = JustifySelf::End;
/// This item will be aligned along the center of the axis.
pub const JUSTIFY_SELF_CENTER: JustifySelf = JustifySelf::Center;
/// This item will be aligned at the baseline.
pub const JUSTIFY_SELF_BASELINE: JustifySelf = JustifySelf::Baseline;
/// This item will be stretched to fill the space it's given.
pub const JUSTIFY_SELF_STRETCH: JustifySelf = JustifySelf::Stretch;

impl ApplyClass<JustifySelf> for Style {
    #[inline]
    fn apply_class(&mut self, class: &JustifySelf) {
        self.justify_self = *class;
    }
}
