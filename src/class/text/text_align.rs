use super::ApplyClass;
use bevy::text::Text;

pub use bevy::text::JustifyText;

impl ApplyClass<JustifyText> for Text {
    fn apply_class(&mut self, class: &JustifyText) {
        self.justify = *class;
    }
}
