use super::{macros::impl_class, ApplyClass};
use derive_more::From;

pub mod font_size;
pub mod text_align;
pub mod text_color;

pub(super) mod text_prelude {
    pub use super::{font_size::*, text_align::*, text_color::*};
    pub use bevy_text::Text;
}
use text_prelude::*;

impl_class!(TextClass -> Text {
    FontSize(FontSize),
    JustifyText(JustifyText),
    TextColor(TextColor)
});
