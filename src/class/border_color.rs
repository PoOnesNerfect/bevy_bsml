use crate::class::ApplyClass;
use bevy_color::{palettes::tailwind::*, Alpha, Color};
use bevy_ui::BorderColor;
use std::ops::Div;

pub fn border_color(color: impl Into<Color>) -> BorderColorClass {
    BorderColorClass(color.into())
}

pub const BORDER_TRANSPARENT: BorderColorClass = BorderColorClass(Color::NONE);
pub const BORDER_BLACK: BorderColorClass = BorderColorClass(Color::BLACK);
pub const BORDER_WHITE: BorderColorClass = BorderColorClass(Color::WHITE);
pub const BORDER_SLATE_50: BorderColorClass = BorderColorClass(Color::Srgba(SLATE_50));
pub const BORDER_SLATE_100: BorderColorClass = BorderColorClass(Color::Srgba(SLATE_100));
pub const BORDER_SLATE_200: BorderColorClass = BorderColorClass(Color::Srgba(SLATE_200));
pub const BORDER_SLATE_300: BorderColorClass = BorderColorClass(Color::Srgba(SLATE_300));
pub const BORDER_SLATE_400: BorderColorClass = BorderColorClass(Color::Srgba(SLATE_400));
pub const BORDER_SLATE_500: BorderColorClass = BorderColorClass(Color::Srgba(SLATE_500));
pub const BORDER_SLATE_600: BorderColorClass = BorderColorClass(Color::Srgba(SLATE_600));
pub const BORDER_SLATE_700: BorderColorClass = BorderColorClass(Color::Srgba(SLATE_700));
pub const BORDER_SLATE_800: BorderColorClass = BorderColorClass(Color::Srgba(SLATE_800));
pub const BORDER_SLATE_900: BorderColorClass = BorderColorClass(Color::Srgba(SLATE_900));
pub const BORDER_SLATE_950: BorderColorClass = BorderColorClass(Color::Srgba(SLATE_950));
pub const BORDER_GRAY_50: BorderColorClass = BorderColorClass(Color::Srgba(GRAY_50));
pub const BORDER_GRAY_100: BorderColorClass = BorderColorClass(Color::Srgba(GRAY_100));
pub const BORDER_GRAY_200: BorderColorClass = BorderColorClass(Color::Srgba(GRAY_200));
pub const BORDER_GRAY_300: BorderColorClass = BorderColorClass(Color::Srgba(GRAY_300));
pub const BORDER_GRAY_400: BorderColorClass = BorderColorClass(Color::Srgba(GRAY_400));
pub const BORDER_GRAY_500: BorderColorClass = BorderColorClass(Color::Srgba(GRAY_500));
pub const BORDER_GRAY_600: BorderColorClass = BorderColorClass(Color::Srgba(GRAY_600));
pub const BORDER_GRAY_700: BorderColorClass = BorderColorClass(Color::Srgba(GRAY_700));
pub const BORDER_GRAY_800: BorderColorClass = BorderColorClass(Color::Srgba(GRAY_800));
pub const BORDER_GRAY_900: BorderColorClass = BorderColorClass(Color::Srgba(GRAY_900));
pub const BORDER_GRAY_950: BorderColorClass = BorderColorClass(Color::Srgba(GRAY_950));
pub const BORDER_ZINC_50: BorderColorClass = BorderColorClass(Color::Srgba(ZINC_50));
pub const BORDER_ZINC_100: BorderColorClass = BorderColorClass(Color::Srgba(ZINC_100));
pub const BORDER_ZINC_200: BorderColorClass = BorderColorClass(Color::Srgba(ZINC_200));
pub const BORDER_ZINC_300: BorderColorClass = BorderColorClass(Color::Srgba(ZINC_300));
pub const BORDER_ZINC_400: BorderColorClass = BorderColorClass(Color::Srgba(ZINC_400));
pub const BORDER_ZINC_500: BorderColorClass = BorderColorClass(Color::Srgba(ZINC_500));
pub const BORDER_ZINC_600: BorderColorClass = BorderColorClass(Color::Srgba(ZINC_600));
pub const BORDER_ZINC_700: BorderColorClass = BorderColorClass(Color::Srgba(ZINC_700));
pub const BORDER_ZINC_800: BorderColorClass = BorderColorClass(Color::Srgba(ZINC_800));
pub const BORDER_ZINC_900: BorderColorClass = BorderColorClass(Color::Srgba(ZINC_900));
pub const BORDER_ZINC_950: BorderColorClass = BorderColorClass(Color::Srgba(ZINC_950));
pub const BORDER_NEUTRAL_50: BorderColorClass = BorderColorClass(Color::Srgba(NEUTRAL_50));
pub const BORDER_NEUTRAL_100: BorderColorClass = BorderColorClass(Color::Srgba(NEUTRAL_100));
pub const BORDER_NEUTRAL_200: BorderColorClass = BorderColorClass(Color::Srgba(NEUTRAL_200));
pub const BORDER_NEUTRAL_300: BorderColorClass = BorderColorClass(Color::Srgba(NEUTRAL_300));
pub const BORDER_NEUTRAL_400: BorderColorClass = BorderColorClass(Color::Srgba(NEUTRAL_400));
pub const BORDER_NEUTRAL_500: BorderColorClass = BorderColorClass(Color::Srgba(NEUTRAL_500));
pub const BORDER_NEUTRAL_600: BorderColorClass = BorderColorClass(Color::Srgba(NEUTRAL_600));
pub const BORDER_NEUTRAL_700: BorderColorClass = BorderColorClass(Color::Srgba(NEUTRAL_700));
pub const BORDER_NEUTRAL_800: BorderColorClass = BorderColorClass(Color::Srgba(NEUTRAL_800));
pub const BORDER_NEUTRAL_900: BorderColorClass = BorderColorClass(Color::Srgba(NEUTRAL_900));
pub const BORDER_NEUTRAL_950: BorderColorClass = BorderColorClass(Color::Srgba(NEUTRAL_950));
pub const BORDER_STONE_50: BorderColorClass = BorderColorClass(Color::Srgba(STONE_50));
pub const BORDER_STONE_100: BorderColorClass = BorderColorClass(Color::Srgba(STONE_100));
pub const BORDER_STONE_200: BorderColorClass = BorderColorClass(Color::Srgba(STONE_200));
pub const BORDER_STONE_300: BorderColorClass = BorderColorClass(Color::Srgba(STONE_300));
pub const BORDER_STONE_400: BorderColorClass = BorderColorClass(Color::Srgba(STONE_400));
pub const BORDER_STONE_500: BorderColorClass = BorderColorClass(Color::Srgba(STONE_500));
pub const BORDER_STONE_600: BorderColorClass = BorderColorClass(Color::Srgba(STONE_600));
pub const BORDER_STONE_700: BorderColorClass = BorderColorClass(Color::Srgba(STONE_700));
pub const BORDER_STONE_800: BorderColorClass = BorderColorClass(Color::Srgba(STONE_800));
pub const BORDER_STONE_900: BorderColorClass = BorderColorClass(Color::Srgba(STONE_900));
pub const BORDER_STONE_950: BorderColorClass = BorderColorClass(Color::Srgba(STONE_950));
pub const BORDER_RED_50: BorderColorClass = BorderColorClass(Color::Srgba(RED_50));
pub const BORDER_RED_100: BorderColorClass = BorderColorClass(Color::Srgba(RED_100));
pub const BORDER_RED_200: BorderColorClass = BorderColorClass(Color::Srgba(RED_200));
pub const BORDER_RED_300: BorderColorClass = BorderColorClass(Color::Srgba(RED_300));
pub const BORDER_RED_400: BorderColorClass = BorderColorClass(Color::Srgba(RED_400));
pub const BORDER_RED_500: BorderColorClass = BorderColorClass(Color::Srgba(RED_500));
pub const BORDER_RED_600: BorderColorClass = BorderColorClass(Color::Srgba(RED_600));
pub const BORDER_RED_700: BorderColorClass = BorderColorClass(Color::Srgba(RED_700));
pub const BORDER_RED_800: BorderColorClass = BorderColorClass(Color::Srgba(RED_800));
pub const BORDER_RED_900: BorderColorClass = BorderColorClass(Color::Srgba(RED_900));
pub const BORDER_RED_950: BorderColorClass = BorderColorClass(Color::Srgba(RED_950));
pub const BORDER_ORANGE_50: BorderColorClass = BorderColorClass(Color::Srgba(ORANGE_50));
pub const BORDER_ORANGE_100: BorderColorClass = BorderColorClass(Color::Srgba(ORANGE_100));
pub const BORDER_ORANGE_200: BorderColorClass = BorderColorClass(Color::Srgba(ORANGE_200));
pub const BORDER_ORANGE_300: BorderColorClass = BorderColorClass(Color::Srgba(ORANGE_300));
pub const BORDER_ORANGE_400: BorderColorClass = BorderColorClass(Color::Srgba(ORANGE_400));
pub const BORDER_ORANGE_500: BorderColorClass = BorderColorClass(Color::Srgba(ORANGE_500));
pub const BORDER_ORANGE_600: BorderColorClass = BorderColorClass(Color::Srgba(ORANGE_600));
pub const BORDER_ORANGE_700: BorderColorClass = BorderColorClass(Color::Srgba(ORANGE_700));
pub const BORDER_ORANGE_800: BorderColorClass = BorderColorClass(Color::Srgba(ORANGE_800));
pub const BORDER_ORANGE_900: BorderColorClass = BorderColorClass(Color::Srgba(ORANGE_900));
pub const BORDER_ORANGE_950: BorderColorClass = BorderColorClass(Color::Srgba(ORANGE_950));
pub const BORDER_AMBER_50: BorderColorClass = BorderColorClass(Color::Srgba(AMBER_50));
pub const BORDER_AMBER_100: BorderColorClass = BorderColorClass(Color::Srgba(AMBER_100));
pub const BORDER_AMBER_200: BorderColorClass = BorderColorClass(Color::Srgba(AMBER_200));
pub const BORDER_AMBER_300: BorderColorClass = BorderColorClass(Color::Srgba(AMBER_300));
pub const BORDER_AMBER_400: BorderColorClass = BorderColorClass(Color::Srgba(AMBER_400));
pub const BORDER_AMBER_500: BorderColorClass = BorderColorClass(Color::Srgba(AMBER_500));
pub const BORDER_AMBER_600: BorderColorClass = BorderColorClass(Color::Srgba(AMBER_600));
pub const BORDER_AMBER_700: BorderColorClass = BorderColorClass(Color::Srgba(AMBER_700));
pub const BORDER_AMBER_800: BorderColorClass = BorderColorClass(Color::Srgba(AMBER_800));
pub const BORDER_AMBER_900: BorderColorClass = BorderColorClass(Color::Srgba(AMBER_900));
pub const BORDER_AMBER_950: BorderColorClass = BorderColorClass(Color::Srgba(AMBER_950));
pub const BORDER_YELLOW_50: BorderColorClass = BorderColorClass(Color::Srgba(YELLOW_50));
pub const BORDER_YELLOW_100: BorderColorClass = BorderColorClass(Color::Srgba(YELLOW_100));
pub const BORDER_YELLOW_200: BorderColorClass = BorderColorClass(Color::Srgba(YELLOW_200));
pub const BORDER_YELLOW_300: BorderColorClass = BorderColorClass(Color::Srgba(YELLOW_300));
pub const BORDER_YELLOW_400: BorderColorClass = BorderColorClass(Color::Srgba(YELLOW_400));
pub const BORDER_YELLOW_500: BorderColorClass = BorderColorClass(Color::Srgba(YELLOW_500));
pub const BORDER_YELLOW_600: BorderColorClass = BorderColorClass(Color::Srgba(YELLOW_600));
pub const BORDER_YELLOW_700: BorderColorClass = BorderColorClass(Color::Srgba(YELLOW_700));
pub const BORDER_YELLOW_800: BorderColorClass = BorderColorClass(Color::Srgba(YELLOW_800));
pub const BORDER_YELLOW_900: BorderColorClass = BorderColorClass(Color::Srgba(YELLOW_900));
pub const BORDER_YELLOW_950: BorderColorClass = BorderColorClass(Color::Srgba(YELLOW_950));
pub const BORDER_LIME_50: BorderColorClass = BorderColorClass(Color::Srgba(LIME_50));
pub const BORDER_LIME_100: BorderColorClass = BorderColorClass(Color::Srgba(LIME_100));
pub const BORDER_LIME_200: BorderColorClass = BorderColorClass(Color::Srgba(LIME_200));
pub const BORDER_LIME_300: BorderColorClass = BorderColorClass(Color::Srgba(LIME_300));
pub const BORDER_LIME_400: BorderColorClass = BorderColorClass(Color::Srgba(LIME_400));
pub const BORDER_LIME_500: BorderColorClass = BorderColorClass(Color::Srgba(LIME_500));
pub const BORDER_LIME_600: BorderColorClass = BorderColorClass(Color::Srgba(LIME_600));
pub const BORDER_LIME_700: BorderColorClass = BorderColorClass(Color::Srgba(LIME_700));
pub const BORDER_LIME_800: BorderColorClass = BorderColorClass(Color::Srgba(LIME_800));
pub const BORDER_LIME_900: BorderColorClass = BorderColorClass(Color::Srgba(LIME_900));
pub const BORDER_LIME_950: BorderColorClass = BorderColorClass(Color::Srgba(LIME_950));
pub const BORDER_GREEN_50: BorderColorClass = BorderColorClass(Color::Srgba(GREEN_50));
pub const BORDER_GREEN_100: BorderColorClass = BorderColorClass(Color::Srgba(GREEN_100));
pub const BORDER_GREEN_200: BorderColorClass = BorderColorClass(Color::Srgba(GREEN_200));
pub const BORDER_GREEN_300: BorderColorClass = BorderColorClass(Color::Srgba(GREEN_300));
pub const BORDER_GREEN_400: BorderColorClass = BorderColorClass(Color::Srgba(GREEN_400));
pub const BORDER_GREEN_500: BorderColorClass = BorderColorClass(Color::Srgba(GREEN_500));
pub const BORDER_GREEN_600: BorderColorClass = BorderColorClass(Color::Srgba(GREEN_600));
pub const BORDER_GREEN_700: BorderColorClass = BorderColorClass(Color::Srgba(GREEN_700));
pub const BORDER_GREEN_800: BorderColorClass = BorderColorClass(Color::Srgba(GREEN_800));
pub const BORDER_GREEN_900: BorderColorClass = BorderColorClass(Color::Srgba(GREEN_900));
pub const BORDER_GREEN_950: BorderColorClass = BorderColorClass(Color::Srgba(GREEN_950));
pub const BORDER_EMERALD_50: BorderColorClass = BorderColorClass(Color::Srgba(EMERALD_50));
pub const BORDER_EMERALD_100: BorderColorClass = BorderColorClass(Color::Srgba(EMERALD_100));
pub const BORDER_EMERALD_200: BorderColorClass = BorderColorClass(Color::Srgba(EMERALD_200));
pub const BORDER_EMERALD_300: BorderColorClass = BorderColorClass(Color::Srgba(EMERALD_300));
pub const BORDER_EMERALD_400: BorderColorClass = BorderColorClass(Color::Srgba(EMERALD_400));
pub const BORDER_EMERALD_500: BorderColorClass = BorderColorClass(Color::Srgba(EMERALD_500));
pub const BORDER_EMERALD_600: BorderColorClass = BorderColorClass(Color::Srgba(EMERALD_600));
pub const BORDER_EMERALD_700: BorderColorClass = BorderColorClass(Color::Srgba(EMERALD_700));
pub const BORDER_EMERALD_800: BorderColorClass = BorderColorClass(Color::Srgba(EMERALD_800));
pub const BORDER_EMERALD_900: BorderColorClass = BorderColorClass(Color::Srgba(EMERALD_900));
pub const BORDER_EMERALD_950: BorderColorClass = BorderColorClass(Color::Srgba(EMERALD_950));
pub const BORDER_TEAL_50: BorderColorClass = BorderColorClass(Color::Srgba(TEAL_50));
pub const BORDER_TEAL_100: BorderColorClass = BorderColorClass(Color::Srgba(TEAL_100));
pub const BORDER_TEAL_200: BorderColorClass = BorderColorClass(Color::Srgba(TEAL_200));
pub const BORDER_TEAL_300: BorderColorClass = BorderColorClass(Color::Srgba(TEAL_300));
pub const BORDER_TEAL_400: BorderColorClass = BorderColorClass(Color::Srgba(TEAL_400));
pub const BORDER_TEAL_500: BorderColorClass = BorderColorClass(Color::Srgba(TEAL_500));
pub const BORDER_TEAL_600: BorderColorClass = BorderColorClass(Color::Srgba(TEAL_600));
pub const BORDER_TEAL_700: BorderColorClass = BorderColorClass(Color::Srgba(TEAL_700));
pub const BORDER_TEAL_800: BorderColorClass = BorderColorClass(Color::Srgba(TEAL_800));
pub const BORDER_TEAL_900: BorderColorClass = BorderColorClass(Color::Srgba(TEAL_900));
pub const BORDER_TEAL_950: BorderColorClass = BorderColorClass(Color::Srgba(TEAL_950));
pub const BORDER_CYAN_50: BorderColorClass = BorderColorClass(Color::Srgba(CYAN_50));
pub const BORDER_CYAN_100: BorderColorClass = BorderColorClass(Color::Srgba(CYAN_100));
pub const BORDER_CYAN_200: BorderColorClass = BorderColorClass(Color::Srgba(CYAN_200));
pub const BORDER_CYAN_300: BorderColorClass = BorderColorClass(Color::Srgba(CYAN_300));
pub const BORDER_CYAN_400: BorderColorClass = BorderColorClass(Color::Srgba(CYAN_400));
pub const BORDER_CYAN_500: BorderColorClass = BorderColorClass(Color::Srgba(CYAN_500));
pub const BORDER_CYAN_600: BorderColorClass = BorderColorClass(Color::Srgba(CYAN_600));
pub const BORDER_CYAN_700: BorderColorClass = BorderColorClass(Color::Srgba(CYAN_700));
pub const BORDER_CYAN_800: BorderColorClass = BorderColorClass(Color::Srgba(CYAN_800));
pub const BORDER_CYAN_900: BorderColorClass = BorderColorClass(Color::Srgba(CYAN_900));
pub const BORDER_CYAN_950: BorderColorClass = BorderColorClass(Color::Srgba(CYAN_950));
pub const BORDER_SKY_50: BorderColorClass = BorderColorClass(Color::Srgba(SKY_50));
pub const BORDER_SKY_100: BorderColorClass = BorderColorClass(Color::Srgba(SKY_100));
pub const BORDER_SKY_200: BorderColorClass = BorderColorClass(Color::Srgba(SKY_200));
pub const BORDER_SKY_300: BorderColorClass = BorderColorClass(Color::Srgba(SKY_300));
pub const BORDER_SKY_400: BorderColorClass = BorderColorClass(Color::Srgba(SKY_400));
pub const BORDER_SKY_500: BorderColorClass = BorderColorClass(Color::Srgba(SKY_500));
pub const BORDER_SKY_600: BorderColorClass = BorderColorClass(Color::Srgba(SKY_600));
pub const BORDER_SKY_700: BorderColorClass = BorderColorClass(Color::Srgba(SKY_700));
pub const BORDER_SKY_800: BorderColorClass = BorderColorClass(Color::Srgba(SKY_800));
pub const BORDER_SKY_900: BorderColorClass = BorderColorClass(Color::Srgba(SKY_900));
pub const BORDER_SKY_950: BorderColorClass = BorderColorClass(Color::Srgba(SKY_950));
pub const BORDER_BLUE_50: BorderColorClass = BorderColorClass(Color::Srgba(BLUE_50));
pub const BORDER_BLUE_100: BorderColorClass = BorderColorClass(Color::Srgba(BLUE_100));
pub const BORDER_BLUE_200: BorderColorClass = BorderColorClass(Color::Srgba(BLUE_200));
pub const BORDER_BLUE_300: BorderColorClass = BorderColorClass(Color::Srgba(BLUE_300));
pub const BORDER_BLUE_400: BorderColorClass = BorderColorClass(Color::Srgba(BLUE_400));
pub const BORDER_BLUE_500: BorderColorClass = BorderColorClass(Color::Srgba(BLUE_500));
pub const BORDER_BLUE_600: BorderColorClass = BorderColorClass(Color::Srgba(BLUE_600));
pub const BORDER_BLUE_700: BorderColorClass = BorderColorClass(Color::Srgba(BLUE_700));
pub const BORDER_BLUE_800: BorderColorClass = BorderColorClass(Color::Srgba(BLUE_800));
pub const BORDER_BLUE_900: BorderColorClass = BorderColorClass(Color::Srgba(BLUE_900));
pub const BORDER_BLUE_950: BorderColorClass = BorderColorClass(Color::Srgba(BLUE_950));
pub const BORDER_INDIGO_50: BorderColorClass = BorderColorClass(Color::Srgba(INDIGO_50));
pub const BORDER_INDIGO_100: BorderColorClass = BorderColorClass(Color::Srgba(INDIGO_100));
pub const BORDER_INDIGO_200: BorderColorClass = BorderColorClass(Color::Srgba(INDIGO_200));
pub const BORDER_INDIGO_300: BorderColorClass = BorderColorClass(Color::Srgba(INDIGO_300));
pub const BORDER_INDIGO_400: BorderColorClass = BorderColorClass(Color::Srgba(INDIGO_400));
pub const BORDER_INDIGO_500: BorderColorClass = BorderColorClass(Color::Srgba(INDIGO_500));
pub const BORDER_INDIGO_600: BorderColorClass = BorderColorClass(Color::Srgba(INDIGO_600));
pub const BORDER_INDIGO_700: BorderColorClass = BorderColorClass(Color::Srgba(INDIGO_700));
pub const BORDER_INDIGO_800: BorderColorClass = BorderColorClass(Color::Srgba(INDIGO_800));
pub const BORDER_INDIGO_900: BorderColorClass = BorderColorClass(Color::Srgba(INDIGO_900));
pub const BORDER_INDIGO_950: BorderColorClass = BorderColorClass(Color::Srgba(INDIGO_950));
pub const BORDER_VIOLET_50: BorderColorClass = BorderColorClass(Color::Srgba(VIOLET_50));
pub const BORDER_VIOLET_100: BorderColorClass = BorderColorClass(Color::Srgba(VIOLET_100));
pub const BORDER_VIOLET_200: BorderColorClass = BorderColorClass(Color::Srgba(VIOLET_200));
pub const BORDER_VIOLET_300: BorderColorClass = BorderColorClass(Color::Srgba(VIOLET_300));
pub const BORDER_VIOLET_400: BorderColorClass = BorderColorClass(Color::Srgba(VIOLET_400));
pub const BORDER_VIOLET_500: BorderColorClass = BorderColorClass(Color::Srgba(VIOLET_500));
pub const BORDER_VIOLET_600: BorderColorClass = BorderColorClass(Color::Srgba(VIOLET_600));
pub const BORDER_VIOLET_700: BorderColorClass = BorderColorClass(Color::Srgba(VIOLET_700));
pub const BORDER_VIOLET_800: BorderColorClass = BorderColorClass(Color::Srgba(VIOLET_800));
pub const BORDER_VIOLET_900: BorderColorClass = BorderColorClass(Color::Srgba(VIOLET_900));
pub const BORDER_VIOLET_950: BorderColorClass = BorderColorClass(Color::Srgba(VIOLET_950));
pub const BORDER_PURPLE_50: BorderColorClass = BorderColorClass(Color::Srgba(PURPLE_50));
pub const BORDER_PURPLE_100: BorderColorClass = BorderColorClass(Color::Srgba(PURPLE_100));
pub const BORDER_PURPLE_200: BorderColorClass = BorderColorClass(Color::Srgba(PURPLE_200));
pub const BORDER_PURPLE_300: BorderColorClass = BorderColorClass(Color::Srgba(PURPLE_300));
pub const BORDER_PURPLE_400: BorderColorClass = BorderColorClass(Color::Srgba(PURPLE_400));
pub const BORDER_PURPLE_500: BorderColorClass = BorderColorClass(Color::Srgba(PURPLE_500));
pub const BORDER_PURPLE_600: BorderColorClass = BorderColorClass(Color::Srgba(PURPLE_600));
pub const BORDER_PURPLE_700: BorderColorClass = BorderColorClass(Color::Srgba(PURPLE_700));
pub const BORDER_PURPLE_800: BorderColorClass = BorderColorClass(Color::Srgba(PURPLE_800));
pub const BORDER_PURPLE_900: BorderColorClass = BorderColorClass(Color::Srgba(PURPLE_900));
pub const BORDER_PURPLE_950: BorderColorClass = BorderColorClass(Color::Srgba(PURPLE_950));
pub const BORDER_FUCHSIA_50: BorderColorClass = BorderColorClass(Color::Srgba(FUCHSIA_50));
pub const BORDER_FUCHSIA_100: BorderColorClass = BorderColorClass(Color::Srgba(FUCHSIA_100));
pub const BORDER_FUCHSIA_200: BorderColorClass = BorderColorClass(Color::Srgba(FUCHSIA_200));
pub const BORDER_FUCHSIA_300: BorderColorClass = BorderColorClass(Color::Srgba(FUCHSIA_300));
pub const BORDER_FUCHSIA_400: BorderColorClass = BorderColorClass(Color::Srgba(FUCHSIA_400));
pub const BORDER_FUCHSIA_500: BorderColorClass = BorderColorClass(Color::Srgba(FUCHSIA_500));
pub const BORDER_FUCHSIA_600: BorderColorClass = BorderColorClass(Color::Srgba(FUCHSIA_600));
pub const BORDER_FUCHSIA_700: BorderColorClass = BorderColorClass(Color::Srgba(FUCHSIA_700));
pub const BORDER_FUCHSIA_800: BorderColorClass = BorderColorClass(Color::Srgba(FUCHSIA_800));
pub const BORDER_FUCHSIA_900: BorderColorClass = BorderColorClass(Color::Srgba(FUCHSIA_900));
pub const BORDER_FUCHSIA_950: BorderColorClass = BorderColorClass(Color::Srgba(FUCHSIA_950));
pub const BORDER_PINK_50: BorderColorClass = BorderColorClass(Color::Srgba(PINK_50));
pub const BORDER_PINK_100: BorderColorClass = BorderColorClass(Color::Srgba(PINK_100));
pub const BORDER_PINK_200: BorderColorClass = BorderColorClass(Color::Srgba(PINK_200));
pub const BORDER_PINK_300: BorderColorClass = BorderColorClass(Color::Srgba(PINK_300));
pub const BORDER_PINK_400: BorderColorClass = BorderColorClass(Color::Srgba(PINK_400));
pub const BORDER_PINK_500: BorderColorClass = BorderColorClass(Color::Srgba(PINK_500));
pub const BORDER_PINK_600: BorderColorClass = BorderColorClass(Color::Srgba(PINK_600));
pub const BORDER_PINK_700: BorderColorClass = BorderColorClass(Color::Srgba(PINK_700));
pub const BORDER_PINK_800: BorderColorClass = BorderColorClass(Color::Srgba(PINK_800));
pub const BORDER_PINK_900: BorderColorClass = BorderColorClass(Color::Srgba(PINK_900));
pub const BORDER_PINK_950: BorderColorClass = BorderColorClass(Color::Srgba(PINK_950));
pub const BORDER_ROSE_50: BorderColorClass = BorderColorClass(Color::Srgba(ROSE_50));
pub const BORDER_ROSE_100: BorderColorClass = BorderColorClass(Color::Srgba(ROSE_100));
pub const BORDER_ROSE_200: BorderColorClass = BorderColorClass(Color::Srgba(ROSE_200));
pub const BORDER_ROSE_300: BorderColorClass = BorderColorClass(Color::Srgba(ROSE_300));
pub const BORDER_ROSE_400: BorderColorClass = BorderColorClass(Color::Srgba(ROSE_400));
pub const BORDER_ROSE_500: BorderColorClass = BorderColorClass(Color::Srgba(ROSE_500));
pub const BORDER_ROSE_600: BorderColorClass = BorderColorClass(Color::Srgba(ROSE_600));
pub const BORDER_ROSE_700: BorderColorClass = BorderColorClass(Color::Srgba(ROSE_700));
pub const BORDER_ROSE_800: BorderColorClass = BorderColorClass(Color::Srgba(ROSE_800));
pub const BORDER_ROSE_900: BorderColorClass = BorderColorClass(Color::Srgba(ROSE_900));
pub const BORDER_ROSE_950: BorderColorClass = BorderColorClass(Color::Srgba(ROSE_950));

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BorderColorClass(pub Color);

impl Default for BorderColorClass {
    fn default() -> Self {
        Self(Color::WHITE)
    }
}

impl Div<f32> for BorderColorClass {
    type Output = BorderColorClass;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0.with_alpha(rhs))
    }
}

impl From<Color> for BorderColorClass {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

impl From<BorderColor> for BorderColorClass {
    fn from(color: BorderColor) -> Self {
        Self(color.0)
    }
}

impl ApplyClass<BorderColorClass> for BorderColor {
    fn apply_class(&mut self, class: &BorderColorClass) {
        self.0 = class.0;
    }
}
