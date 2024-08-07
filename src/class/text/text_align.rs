use super::ApplyClass;
use bevy_text::Text;

pub use bevy_text::JustifyText;

impl ApplyClass<JustifyText> for Text {
    fn apply_class(&mut self, class: &JustifyText) {
        self.justify = *class;
    }
}
