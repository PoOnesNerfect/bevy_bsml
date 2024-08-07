use crate::class::ApplyClass;
use bevy_ui::{Style, UiRect, Val};

/// Margin in px
pub fn m(px: f32) -> Margin {
    Margin(Val::Px(px))
}

/// Margin X in px
pub fn mx(px: f32) -> MarginX {
    MarginX(Val::Px(px))
}

/// Margin Y in px
pub fn my(px: f32) -> MarginY {
    MarginY(Val::Px(px))
}

/// Margin Top in px
pub fn mt(px: f32) -> MarginTop {
    MarginTop(Val::Px(px))
}

/// Margin Bottom in px
pub fn mb(px: f32) -> MarginBottom {
    MarginBottom(Val::Px(px))
}

/// Margin Left in px
pub fn ml(px: f32) -> MarginLeft {
    MarginLeft(Val::Px(px))
}

/// Margin Right in px
pub fn mr(px: f32) -> MarginRight {
    MarginRight(Val::Px(px))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Margin(pub Val);

#[derive(Debug, Clone, PartialEq)]
pub struct MarginX(pub Val);

#[derive(Debug, Clone, PartialEq)]
pub struct MarginY(pub Val);

#[derive(Debug, Clone, PartialEq)]
pub struct MarginTop(pub Val);

#[derive(Debug, Clone, PartialEq)]
pub struct MarginBottom(pub Val);

#[derive(Debug, Clone, PartialEq)]
pub struct MarginLeft(pub Val);

#[derive(Debug, Clone, PartialEq)]
pub struct MarginRight(pub Val);

impl ApplyClass<Margin> for Style {
    fn apply_class(&mut self, class: &Margin) {
        self.margin = UiRect {
            left: class.0.clone(),
            right: class.0.clone(),
            top: class.0.clone(),
            bottom: class.0.clone(),
        };
    }
}

impl ApplyClass<MarginX> for Style {
    fn apply_class(&mut self, class: &MarginX) {
        self.margin.left = class.0.clone();
        self.margin.right = class.0.clone();
    }
}

impl ApplyClass<MarginY> for Style {
    fn apply_class(&mut self, class: &MarginY) {
        self.margin.top = class.0.clone();
        self.margin.bottom = class.0.clone();
    }
}

impl ApplyClass<MarginTop> for Style {
    fn apply_class(&mut self, class: &MarginTop) {
        self.margin.top = class.0.clone();
    }
}

impl ApplyClass<MarginBottom> for Style {
    fn apply_class(&mut self, class: &MarginBottom) {
        self.margin.bottom = class.0.clone();
    }
}

impl ApplyClass<MarginLeft> for Style {
    fn apply_class(&mut self, class: &MarginLeft) {
        self.margin.left = class.0.clone();
    }
}

impl ApplyClass<MarginRight> for Style {
    fn apply_class(&mut self, class: &MarginRight) {
        self.margin.right = class.0.clone();
    }
}
