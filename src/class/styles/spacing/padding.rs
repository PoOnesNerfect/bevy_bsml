use crate::class::ApplyClass;
use bevy_ui::{Style, UiRect, Val};

/// padding in px
pub fn p(px: f32) -> Padding {
    Padding(Val::Px(px))
}

/// padding x in px
pub fn px(px: f32) -> PaddingX {
    PaddingX(Val::Px(px))
}

/// padding y in px
pub fn py(px: f32) -> PaddingY {
    PaddingY(Val::Px(px))
}

/// padding top in px
pub fn pt(px: f32) -> PaddingTop {
    PaddingTop(Val::Px(px))
}

/// padding bottom in px
pub fn pb(px: f32) -> PaddingBottom {
    PaddingBottom(Val::Px(px))
}

/// padding left in px
pub fn pl(px: f32) -> PaddingLeft {
    PaddingLeft(Val::Px(px))
}

/// padding right in px
pub fn pr(px: f32) -> PaddingRight {
    PaddingRight(Val::Px(px))
}

#[derive(Debug, Clone, PartialEq)]
pub struct Padding(pub Val);

#[derive(Debug, Clone, PartialEq)]
pub struct PaddingX(pub Val);

#[derive(Debug, Clone, PartialEq)]
pub struct PaddingY(pub Val);

#[derive(Debug, Clone, PartialEq)]
pub struct PaddingTop(pub Val);

#[derive(Debug, Clone, PartialEq)]
pub struct PaddingBottom(pub Val);

#[derive(Debug, Clone, PartialEq)]
pub struct PaddingLeft(pub Val);

#[derive(Debug, Clone, PartialEq)]
pub struct PaddingRight(pub Val);

impl ApplyClass<Padding> for Style {
    fn apply_class(&mut self, class: &Padding) {
        self.padding = UiRect {
            left: class.0.clone(),
            right: class.0.clone(),
            top: class.0.clone(),
            bottom: class.0.clone(),
        };
    }
}

impl ApplyClass<PaddingX> for Style {
    fn apply_class(&mut self, class: &PaddingX) {
        self.padding.left = class.0.clone();
        self.padding.right = class.0.clone();
    }
}

impl ApplyClass<PaddingY> for Style {
    fn apply_class(&mut self, class: &PaddingY) {
        self.padding.top = class.0.clone();
        self.padding.bottom = class.0.clone();
    }
}

impl ApplyClass<PaddingTop> for Style {
    fn apply_class(&mut self, class: &PaddingTop) {
        self.padding.top = class.0.clone();
    }
}

impl ApplyClass<PaddingBottom> for Style {
    fn apply_class(&mut self, class: &PaddingBottom) {
        self.padding.bottom = class.0.clone();
    }
}

impl ApplyClass<PaddingLeft> for Style {
    fn apply_class(&mut self, class: &PaddingLeft) {
        self.padding.left = class.0.clone();
    }
}

impl ApplyClass<PaddingRight> for Style {
    fn apply_class(&mut self, class: &PaddingRight) {
        self.padding.right = class.0.clone();
    }
}
