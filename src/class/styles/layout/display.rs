use bevy::prelude::*;

pub use bevy::prelude::Display;

use crate::class::ApplyClass;

pub const FLEX: Display = Display::Flex;
pub const BLOCK: Display = Display::Block;
pub const GRID: Display = Display::Grid;
pub const DISPLAY_NONE: Display = Display::None;

impl ApplyClass<Display> for Style {
    fn apply_class(&mut self, class: &Display) {
        self.display = *class;
    }
}
