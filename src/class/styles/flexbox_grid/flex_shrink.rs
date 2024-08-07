use crate::class::ApplyClass;

pub const SHRINK_0: FlexShrink = FlexShrink(0.);
pub const SHRINK: FlexShrink = FlexShrink(1.);

pub fn shrink(val: f32) -> FlexShrink {
    FlexShrink(val)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlexShrink(pub f32);

impl ApplyClass<FlexShrink> for bevy_ui::Style {
    fn apply_class(&mut self, class: &FlexShrink) {
        self.flex_shrink = class.0;
    }
}
