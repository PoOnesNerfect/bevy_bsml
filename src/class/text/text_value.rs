use super::ApplyClass;
use bevy_text::Text;

pub fn text(value: impl ToString) -> TextValue {
    TextValue(value.to_string())
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextValue(pub String);

impl ApplyClass<TextValue> for Text {
    fn apply_class(&mut self, class: &TextValue) {
        self.sections[0].value = class.0.clone();
    }
}
