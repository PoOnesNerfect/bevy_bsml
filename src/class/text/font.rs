use super::ApplyClass;
use bevy_asset::Handle;
use bevy_text::{Font, Text};

/// A class for setting the font of a text.
pub fn font(handle: Handle<Font>) -> FontHandle {
    FontHandle(handle)
}

#[derive(Debug, Clone, PartialEq)]
pub struct FontHandle(pub Handle<Font>);

impl ApplyClass<FontHandle> for Text {
    fn apply_class(&mut self, class: &FontHandle) {
        self.sections[0].style.font = class.0.clone();
    }
}
