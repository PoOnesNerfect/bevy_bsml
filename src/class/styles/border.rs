use bevy::ui::Val;

use crate::class::ApplyClass;

/// Border width in px
pub fn border(px: f32) -> Border {
    Border(Val::Px(px))
}

/// Border width x in px
pub fn border_x(px: f32) -> BorderX {
    BorderX(Val::Px(px))
}

/// Border width y in px
pub fn border_y(px: f32) -> BorderY {
    BorderY(Val::Px(px))
}

/// Border top width in px
pub fn border_t(px: f32) -> BorderTop {
    BorderTop(Val::Px(px))
}

/// Border bottom width in px
pub fn border_b(px: f32) -> BorderBottom {
    BorderBottom(Val::Px(px))
}

/// Border right width in px
pub fn border_r(px: f32) -> BorderRight {
    BorderRight(Val::Px(px))
}

/// Border left width in px
pub fn border_l(px: f32) -> BorderLeft {
    BorderLeft(Val::Px(px))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Border(Val);

#[derive(Debug, Clone, PartialEq)]
pub struct BorderX(Val);

#[derive(Debug, Clone, PartialEq)]
pub struct BorderY(Val);

#[derive(Debug, Clone, PartialEq)]
pub struct BorderTop(Val);

#[derive(Debug, Clone, PartialEq)]
pub struct BorderBottom(Val);

#[derive(Debug, Clone, PartialEq)]
pub struct BorderRight(Val);

#[derive(Debug, Clone, PartialEq)]
pub struct BorderLeft(Val);

impl ApplyClass<Border> for bevy::ui::Style {
    fn apply_class(&mut self, class: &Border) {
        self.border.top = class.0.clone();
        self.border.bottom = class.0.clone();
        self.border.left = class.0.clone();
        self.border.right = class.0.clone();
    }
}

impl ApplyClass<BorderX> for bevy::ui::Style {
    fn apply_class(&mut self, class: &BorderX) {
        self.border.left = class.0.clone();
        self.border.right = class.0.clone();
    }
}

impl ApplyClass<BorderY> for bevy::ui::Style {
    fn apply_class(&mut self, class: &BorderY) {
        self.border.top = class.0.clone();
        self.border.bottom = class.0.clone();
    }
}

impl ApplyClass<BorderTop> for bevy::ui::Style {
    fn apply_class(&mut self, class: &BorderTop) {
        self.border.top = class.0.clone();
    }
}

impl ApplyClass<BorderBottom> for bevy::ui::Style {
    fn apply_class(&mut self, class: &BorderBottom) {
        self.border.bottom = class.0.clone();
    }
}

impl ApplyClass<BorderRight> for bevy::ui::Style {
    fn apply_class(&mut self, class: &BorderRight) {
        self.border.right = class.0.clone();
    }
}

impl ApplyClass<BorderLeft> for bevy::ui::Style {
    fn apply_class(&mut self, class: &BorderLeft) {
        self.border.left = class.0.clone();
    }
}
