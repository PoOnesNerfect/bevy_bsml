use self::color::TextColor;
use super::ApplyClass;
use bevy::text::{Text, TextAlignment};
use derive_more::From;

pub mod color;

#[derive(Debug, Clone, From)]
pub enum TextClass {
    FontSize(FontSize),
    TextAlignment(TextAlignment),
    TextColor(TextColor),
}

impl ApplyClass for TextClass {
    type Component = Text;

    fn apply_class(&self, component: &mut Self::Component) {
        match self {
            Self::FontSize(font_size) => font_size.apply_class(component),
            Self::TextAlignment(text_alignment) => text_alignment.apply_class(component),
            Self::TextColor(text_color) => text_color.apply_class(component),
        }
    }
}

impl ApplyClass for TextAlignment {
    type Component = Text;

    fn apply_class(&self, component: &mut Self::Component) {
        component.alignment = *self;
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
