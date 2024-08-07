use super::ApplyClass;
use bevy_text::Text;

pub const TEXT_XS: FontSize = FontSize(12.0);
pub const TEXT_SM: FontSize = FontSize(14.0);
pub const TEXT_BASE: FontSize = FontSize(16.0);
pub const TEXT_LG: FontSize = FontSize(18.0);
pub const TEXT_XL: FontSize = FontSize(20.0);
pub const TEXT_2XL: FontSize = FontSize(24.0);
pub const TEXT_3XL: FontSize = FontSize(30.0);
pub const TEXT_4XL: FontSize = FontSize(36.0);
pub const TEXT_5XL: FontSize = FontSize(48.0);
pub const TEXT_6XL: FontSize = FontSize(64.0);
pub const TEXT_7XL: FontSize = FontSize(80.0);
pub const TEXT_8XL: FontSize = FontSize(96.0);
pub const TEXT_9XL: FontSize = FontSize(128.0);

pub fn font(px: f32) -> FontSize {
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
