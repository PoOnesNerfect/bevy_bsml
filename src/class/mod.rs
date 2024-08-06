use self::{
    background_color::BackgroundColorClass, border_color::BorderColorClass, text::TextClass,
};
use bevy::prelude::*;
use derive_more::From;

mod styles;
pub use styles::*;

pub(super) mod prelude {
    pub use super::background_color::*;
    pub use super::border_color::*;
    pub use super::styles_prelude::*;
    pub use super::text::prelude::*;
    pub use super::z_index::*;

    pub use super::hovered;
    pub use super::pressed;
}

pub mod background_color;
pub mod border_color;
pub mod text;
pub mod z_index;

pub fn hovered<C>(class: C) -> (Interaction, C) {
    (Interaction::Hovered, class)
}

pub fn pressed<C>(class: C) -> (Interaction, C) {
    (Interaction::Pressed, class)
}

/// A class that can be applied to a UI node.
#[doc(hidden)]
#[derive(Clone, Debug, From, PartialEq)]
pub enum BsmlClass {
    BackgroundColor(BackgroundColorClass),
    BorderColor(BorderColorClass),
    Style(StyleClass),
    ZIndex(ZIndex),
    Text(TextClass),
}

impl BsmlClass {
    /// Check if the class is of same type to another class.
    #[inline]
    pub(crate) fn eq_class_type(&self, other: &Self) -> bool {
        match (self, other) {
            (BsmlClass::BackgroundColor(_), BsmlClass::BackgroundColor(_)) => true,
            (BsmlClass::BorderColor(_), BsmlClass::BorderColor(_)) => true,
            (BsmlClass::ZIndex(_), BsmlClass::ZIndex(_)) => true,
            (BsmlClass::Text(a), BsmlClass::Text(b)) => {
                std::mem::discriminant(a) == std::mem::discriminant(b)
            }
            (BsmlClass::Style(a), BsmlClass::Style(b)) => {
                std::mem::discriminant(a) == std::mem::discriminant(b)
            }
            _ => false,
        }
    }
}

#[doc(hidden)]
pub trait WithInteraction {
    fn with_interaction(self) -> (Interaction, BsmlClass);
}

impl<T: Into<BsmlClass>> WithInteraction for (Interaction, T) {
    fn with_interaction(self) -> (Interaction, BsmlClass) {
        (self.0, self.1.into())
    }
}

impl<T: Into<BsmlClass>> WithInteraction for T {
    fn with_interaction(self) -> (Interaction, BsmlClass) {
        (Interaction::None, self.into())
    }
}

/// A tailwind class that can be applied to a UI component.
#[doc(hidden)]
pub trait ApplyClass<Class> {
    fn apply_class(&mut self, class: &Class);
}

impl ApplyClass<BsmlClass> for NodeBundle {
    #[inline]
    fn apply_class(&mut self, class: &BsmlClass) {
        match class {
            BsmlClass::BackgroundColor(class) => self.background_color.apply_class(class),
            BsmlClass::BorderColor(class) => self.border_color.apply_class(class),
            BsmlClass::Style(class) => self.style.apply_class(class),
            BsmlClass::ZIndex(class) => self.z_index.apply_class(class),
            BsmlClass::Text(_) => {}
        }
    }
}

impl ApplyClass<BsmlClass> for TextBundle {
    #[inline]
    fn apply_class(&mut self, class: &BsmlClass) {
        match class {
            BsmlClass::BackgroundColor(class) => self.background_color.apply_class(class),
            BsmlClass::Style(class) => self.style.apply_class(class),
            BsmlClass::ZIndex(class) => self.z_index.apply_class(class),
            BsmlClass::Text(class) => self.text.apply_class(class),
            BsmlClass::BorderColor(_) => {}
        }
    }
}

impl ApplyClass<BsmlClass> for Style {
    #[inline]
    fn apply_class(&mut self, class: &BsmlClass) {
        match class {
            BsmlClass::Style(class) => self.apply_class(class),
            _ => {}
        }
    }
}

impl ApplyClass<BsmlClass> for Text {
    #[inline]
    fn apply_class(&mut self, class: &BsmlClass) {
        match class {
            BsmlClass::Text(class) => self.apply_class(class),
            _ => {}
        }
    }
}

impl ApplyClass<BsmlClass> for BackgroundColor {
    #[inline]
    fn apply_class(&mut self, class: &BsmlClass) {
        match class {
            BsmlClass::BackgroundColor(color) => self.apply_class(color),
            _ => {}
        }
    }
}

impl ApplyClass<BsmlClass> for BorderColor {
    #[inline]
    fn apply_class(&mut self, class: &BsmlClass) {
        match class {
            BsmlClass::BorderColor(color) => self.apply_class(color),
            _ => {}
        }
    }
}

impl ApplyClass<BsmlClass> for ZIndex {
    #[inline]
    fn apply_class(&mut self, class: &BsmlClass) {
        match class {
            BsmlClass::ZIndex(color) => self.apply_class(color),
            _ => {}
        }
    }
}
