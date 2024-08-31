use super::{macros::impl_class, ApplyClass};
use derive_more::From;

pub mod font;
pub mod font_size;
pub mod text_align;
pub mod text_color;
pub mod text_value;

pub(super) mod text_prelude {
    pub use super::{font::*, font_size::*, text_align::*, text_color::*, text_value::*};
    pub use bevy_text::Text;
}
use text_prelude::*;

impl_class!(TextClass -> Text {
    FontHandle(FontHandle),
    FontSize(FontSize),
    JustifyText(JustifyText),
    TextColor(TextColor),
    TextValue(TextValue)
});
