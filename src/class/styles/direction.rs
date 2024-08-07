pub use bevy::ui::Direction;

use crate::class::ApplyClass;

pub const DIRECTION_INHERIT: Direction = Direction::Inherit;
pub const LEFT_TO_RIGHT: Direction = Direction::LeftToRight;
pub const RIGHT_TO_LEFT: Direction = Direction::RightToLeft;

impl ApplyClass<Direction> for bevy::ui::Style {
    #[inline]
    fn apply_class(&mut self, class: &Direction) {
        self.direction = *class;
    }
}
