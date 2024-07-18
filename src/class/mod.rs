use self::{
    background_color::BackgroundColorClass, border_color::BorderColorClass, text::TextClass,
};
use bevy::{
    prelude::{Component, NodeBundle, TextBundle},
    ui::{Interaction, ZIndex},
};
use derive_more::From;

mod styles;
pub use styles::*;

pub mod background_color;
pub mod border_color;
pub mod text;
pub mod z_index;

pub fn hovered<C: ApplyClass>(class: C) -> (Interaction, C) {
    (Interaction::Hovered, class)
}

pub fn pressed<C: ApplyClass>(class: C) -> (Interaction, C) {
    (Interaction::Pressed, class)
}

#[derive(Clone, From)]
pub enum ClassEnum {
    BackgroundColor(BackgroundColorClass),
    BorderColor(BorderColorClass),
    Style(StyleClass),
    ZIndex(ZIndex),
    Text(TextClass),
}

#[doc(hidden)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<'a> From<&'a mut NodeBundle> for Either<&'a mut NodeBundle, &'a mut TextBundle> {
    fn from(bundle: &'a mut NodeBundle) -> Self {
        Either::Left(bundle)
    }
}

impl<'a> From<&'a mut TextBundle> for Either<&'a mut NodeBundle, &'a mut TextBundle> {
    fn from(bundle: &'a mut TextBundle) -> Self {
        Either::Right(bundle)
    }
}

impl ClassEnum {
    pub fn apply_to_either_bundle<'a, T: Into<Either<&'a mut NodeBundle, &'a mut TextBundle>>>(
        &self,
        bundle: T,
    ) {
        match bundle.into() {
            Either::Left(bundle) => self.apply_to_node_bundle(bundle),
            Either::Right(bundle) => self.apply_to_text_bundle(bundle),
        }
    }

    pub fn apply_to_node_bundle(&self, bundle: &mut NodeBundle) {
        match self {
            ClassEnum::BackgroundColor(class) => class.apply_class(&mut bundle.background_color),
            ClassEnum::BorderColor(class) => class.apply_class(&mut bundle.border_color),
            ClassEnum::Style(class) => class.apply_class(&mut bundle.style),
            ClassEnum::ZIndex(class) => class.apply_class(&mut bundle.z_index),
            ClassEnum::Text(_) => {}
        }
    }

    pub fn apply_to_text_bundle(&self, bundle: &mut TextBundle) {
        match self {
            ClassEnum::Text(class) => class.apply_class(&mut bundle.text),
            ClassEnum::BackgroundColor(class) => class.apply_class(&mut bundle.background_color),
            ClassEnum::Style(class) => class.apply_class(&mut bundle.style),
            ClassEnum::ZIndex(class) => class.apply_class(&mut bundle.z_index),
            ClassEnum::BorderColor(_) => {}
        }
    }
}

pub trait WithInteraction {
    fn with_interaction(self) -> (Interaction, ClassEnum);
}

impl<T: Into<ClassEnum>> WithInteraction for (Interaction, T) {
    fn with_interaction(self) -> (Interaction, ClassEnum) {
        (self.0, self.1.into())
    }
}

impl<T: Into<ClassEnum>> WithInteraction for T {
    fn with_interaction(self) -> (Interaction, ClassEnum) {
        (Interaction::None, self.into())
    }
}

pub trait ApplyClass {
    type Component: Component;

    fn apply_class(&self, component: &mut Self::Component);
}
