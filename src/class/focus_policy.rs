use crate::class::ApplyClass;

pub use bevy::ui::FocusPolicy;

pub const FOCUS_BLOCK: FocusPolicy = FocusPolicy::Block;
pub const FOCUS_PASS: FocusPolicy = FocusPolicy::Pass;

impl ApplyClass<FocusPolicy> for FocusPolicy {
    fn apply_class(&mut self, class: &FocusPolicy) {
        *self = *class;
    }
}
