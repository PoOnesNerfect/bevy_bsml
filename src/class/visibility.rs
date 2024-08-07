use crate::class::ApplyClass;

pub use bevy::render::view::Visibility;

/// Show the node.
pub const VISIBLE: Visibility = Visibility::Visible;

/// Hide the node.
pub const HIDDEN: Visibility = Visibility::Hidden;

/// Inherit the visibility from the parent.
pub const INHERITED: Visibility = Visibility::Inherited;

impl ApplyClass<Visibility> for Visibility {
    fn apply_class(&mut self, class: &Visibility) {
        *self = *class;
    }
}
