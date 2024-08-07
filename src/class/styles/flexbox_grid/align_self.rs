pub use bevy::ui::AlignSelf;

use crate::class::ApplyClass;

/// Use the parent node's [`AlignItems`] value to determine how this item should be aligned.
pub const SELF_AUTO: AlignSelf = AlignSelf::Auto;
/// This item will be aligned with the start of the axis, unless the flex direction is reversed;
/// then it will be aligned with the end of the axis
pub const SELF_START: AlignSelf = AlignSelf::FlexStart;
/// This item will be aligned with the end of the axis, unless the flex direction is reversed;
/// then it will be aligned with the start of the axis
pub const SELF_END: AlignSelf = AlignSelf::FlexEnd;
/// This item will be aligned along the center of the axis.
pub const SELF_CENTER: AlignSelf = AlignSelf::Center;
/// This item will be aligned at the baseline.
pub const SELF_BASELINE: AlignSelf = AlignSelf::Baseline;
/// This item will be stretched to fill the container.
pub const SELF_STRETCH: AlignSelf = AlignSelf::Stretch;

impl ApplyClass<AlignSelf> for bevy::ui::Style {
    fn apply_class(&mut self, class: &AlignSelf) {
        self.align_self = *class;
    }
}
