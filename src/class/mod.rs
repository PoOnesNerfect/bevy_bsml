use bevy::{
    prelude::{Component, NodeBundle, TextBundle},
    text::{Text, TextAlignment},
    ui::{BackgroundColor, BorderColor, Interaction, Style, ZIndex},
};

pub mod background_color;
pub mod border_color;
pub mod styles;
pub mod text;
pub mod z_index;

pub trait ApplyClass {
    type Component: Component;

    fn apply_class(&self, interaction: Interaction, component: &mut Self::Component);
}

pub fn hovered<C: ApplyClass>(class: C) -> InteractionClass<C> {
    InteractionClass {
        inner: class,
        interaction: Interaction::Hovered,
    }
}

pub fn pressed<C: ApplyClass>(class: C) -> InteractionClass<C> {
    InteractionClass {
        inner: class,
        interaction: Interaction::Pressed,
    }
}

#[derive(Debug, Clone)]
pub struct InteractionClass<T> {
    pub inner: T,
    pub interaction: Interaction,
}

impl<T: ApplyClass> ApplyClass for InteractionClass<T> {
    type Component = T::Component;

    fn apply_class(&self, interaction: Interaction, component: &mut Self::Component) {
        if interaction == self.interaction {
            self.inner.apply_class(interaction, component);
        }
    }
}

#[doc(hidden)]
pub fn apply_class_to_node_bundle<C: ApplyClass>(
    bundle: &mut NodeBundle,
    interaction: Interaction,
    class: C,
) where
    C::Component: FromBundle<NodeBundle>,
{
    class.apply_class(interaction, C::Component::from_bundle(bundle));
}

#[doc(hidden)]
pub fn apply_class_to_text_bundle<C: ApplyClass>(
    bundle: &mut TextBundle,
    interaction: Interaction,
    class: C,
) where
    C::Component: FromBundle<TextBundle>,
{
    class.apply_class(interaction, C::Component::from_bundle(bundle));
}

pub trait FromBundle<Bundle> {
    fn from_bundle(bundle: &mut Bundle) -> &mut Self;
}

impl FromBundle<NodeBundle> for Style {
    fn from_bundle(bundle: &mut NodeBundle) -> &mut Self {
        &mut bundle.style
    }
}

impl FromBundle<TextBundle> for Style {
    fn from_bundle(bundle: &mut TextBundle) -> &mut Self {
        &mut bundle.style
    }
}

impl FromBundle<NodeBundle> for BackgroundColor {
    fn from_bundle(bundle: &mut NodeBundle) -> &mut Self {
        &mut bundle.background_color
    }
}

impl FromBundle<TextBundle> for BackgroundColor {
    fn from_bundle(bundle: &mut TextBundle) -> &mut Self {
        &mut bundle.background_color
    }
}

impl FromBundle<NodeBundle> for BorderColor {
    fn from_bundle(bundle: &mut NodeBundle) -> &mut Self {
        &mut bundle.border_color
    }
}

impl FromBundle<NodeBundle> for ZIndex {
    fn from_bundle(bundle: &mut NodeBundle) -> &mut Self {
        &mut bundle.z_index
    }
}

impl FromBundle<TextBundle> for ZIndex {
    fn from_bundle(bundle: &mut TextBundle) -> &mut Self {
        &mut bundle.z_index
    }
}

impl FromBundle<TextBundle> for Text {
    fn from_bundle(bundle: &mut TextBundle) -> &mut Self {
        &mut bundle.text
    }
}

impl FromBundle<TextBundle> for TextAlignment {
    fn from_bundle(bundle: &mut TextBundle) -> &mut Self {
        &mut bundle.text.alignment
    }
}
