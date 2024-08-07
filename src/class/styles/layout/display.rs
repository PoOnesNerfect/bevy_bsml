use crate::class::ApplyClass;
use bevy_ui::Style;

pub use bevy_ui::Display;

pub const FLEX: Display = Display::Flex;
pub const BLOCK: Display = Display::Block;
pub const GRID: Display = Display::Grid;
pub const DISPLAY_NONE: Display = Display::None;

impl ApplyClass<Display> for Style {
    fn apply_class(&mut self, class: &Display) {
        self.display = *class;
    }
}
