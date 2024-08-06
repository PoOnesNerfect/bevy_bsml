use super::ApplyClass;
use bevy::text::{JustifyText, Text};
use derive_more::From;

mod color;

pub use color::*;

#[derive(Debug, Clone, From)]
pub enum TextClass {
    FontSize(FontSize),
    JustifyText(JustifyText),
    TextColor(TextColor),
}

impl ApplyClass for TextClass {
    type Component = Text;

    fn apply_class(&self, component: &mut Self::Component) {
        match self {
            Self::FontSize(font_size) => font_size.apply_class(component),
            Self::JustifyText(justify_text) => justify_text.apply_class(component),
            Self::TextColor(text_color) => text_color.apply_class(component),
        }
    }
}

impl ApplyClass for JustifyText {
    type Component = Text;

    fn apply_class(&self, component: &mut Self::Component) {
        component.justify = *self;
    }
}

pub const TEXT_XS: TextClass = TextClass::FontSize(FontSize(12.0));
pub const TEXT_SM: TextClass = TextClass::FontSize(FontSize(14.0));
pub const TEXT_BASE: TextClass = TextClass::FontSize(FontSize(16.0));

#[derive(Debug, Clone, Copy)]
pub struct FontSize(f32);

impl FontSize {
    pub fn px(px: f32) -> TextClass {
        TextClass::FontSize(Self(px))
    }
}

impl ApplyClass for FontSize {
    type Component = Text;

    fn apply_class(&self, component: &mut Self::Component) {
        component.sections[0].style.font_size = self.0;
    }
}
