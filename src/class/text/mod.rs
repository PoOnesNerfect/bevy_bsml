use super::ApplyClass;
use derive_more::From;

pub mod font_size;
pub mod text_align;
pub mod text_color;

pub(super) mod prelude {
    pub use super::{font_size::*, text_align::*, text_color::*};
    pub use bevy::text::Text;
}
use prelude::*;

#[derive(Debug, Clone, From, PartialEq)]
pub enum TextClass {
    FontSize(FontSize),
    JustifyText(JustifyText),
    TextColor(TextColor),
}

impl ApplyClass<TextClass> for Text {
    fn apply_class(&mut self, class: &TextClass) {
        match class {
            TextClass::FontSize(font_size) => self.apply_class(font_size),
            TextClass::JustifyText(justify_text) => self.apply_class(justify_text),
            TextClass::TextColor(text_color) => self.apply_class(text_color),
        }
    }
}

impl From<FontSize> for super::BsmlClass {
    fn from(val: FontSize) -> Self {
        Self::Text(TextClass::FontSize(val))
    }
}
impl From<JustifyText> for super::BsmlClass {
    fn from(val: JustifyText) -> Self {
        Self::Text(TextClass::JustifyText(val))
    }
}
impl From<TextColor> for super::BsmlClass {
    fn from(val: TextColor) -> Self {
        Self::Text(TextClass::TextColor(val))
    }
}
