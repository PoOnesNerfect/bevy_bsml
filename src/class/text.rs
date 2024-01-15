use super::ApplyClass;
use bevy::{
    prelude::Color,
    text::{Text, TextAlignment},
    ui::Interaction,
};
use derive_more::From;

#[derive(Debug, Clone, From)]
pub enum TextClass {
    FontSize(FontSize),
    TextAlignment(TextAlignment),
    TextColor(TextColor),
}

impl ApplyClass for TextClass {
    type Component = Text;

    fn apply_class(&self, interaction: Interaction, component: &mut Self::Component) {
        match self {
            Self::FontSize(font_size) => font_size.apply_class(interaction, component),
            Self::TextAlignment(text_alignment) => {
                text_alignment.apply_class(interaction, component)
            }
            Self::TextColor(text_color) => text_color.apply_class(interaction, component),
        }
    }
}

impl ApplyClass for TextAlignment {
    type Component = Text;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
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

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.sections[0].style.font_size = self.0;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TextColor(Color);

impl ApplyClass for TextColor {
    type Component = Text;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.sections[0].style.color = self.0;
    }
}
