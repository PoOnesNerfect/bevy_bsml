use super::ApplyClass;
use bevy::text::Text;

pub const TEXT_XS: FontSize = FontSize(12.0);
pub const TEXT_SM: FontSize = FontSize(14.0);
pub const TEXT_BASE: FontSize = FontSize(16.0);

pub fn font_px(px: f32) -> FontSize {
    FontSize(px)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FontSize(f32);

impl FontSize {
    pub fn px(px: f32) -> FontSize {
        Self(px)
    }
}

impl ApplyClass<FontSize> for Text {
    fn apply_class(&mut self, class: &FontSize) {
        self.sections[0].style.font_size = class.0;
    }
}
