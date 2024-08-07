use crate::class::ApplyClass;

pub const GROW_0: FlexGrow = FlexGrow(0.);
pub const GROW: FlexGrow = FlexGrow(1.);

pub fn grow(val: f32) -> FlexGrow {
    FlexGrow(val)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FlexGrow(pub f32);

impl ApplyClass<FlexGrow> for bevy::ui::Style {
    fn apply_class(&mut self, class: &FlexGrow) {
        self.flex_grow = class.0;
    }
}
