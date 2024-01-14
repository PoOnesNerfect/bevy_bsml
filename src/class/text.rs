use super::ApplyClass;
use bevy::{
    text::{Text, TextAlignment},
    ui::Interaction,
};
use derive_more::From;

#[derive(Debug, Clone, From)]
pub enum TextClass {
    FontSize(FontSize),
    TextAlignment(TextAlignment),
}

impl ApplyClass for TextClass {
    type Component = Text;

    fn apply_class(&self, interaction: Interaction, component: &mut Self::Component) {
        match self {
            Self::FontSize(font_size) => font_size.apply_class(interaction, component),
            Self::TextAlignment(text_alignment) => {
                text_alignment.apply_class(interaction, component)
            }
        }
    }
}

impl ApplyClass for TextAlignment {
    type Component = Text;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.alignment = *self;
    }
}

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
