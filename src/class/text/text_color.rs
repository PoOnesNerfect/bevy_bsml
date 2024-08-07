use super::ApplyClass;
use bevy_color::{palettes::tailwind::*, Color};
use bevy_text::Text;

pub fn text_color(color: impl Into<Color>) -> TextColor {
    TextColor(color.into())
}

pub const TEXT_TRANSPARENT: TextColor = TextColor(Color::NONE);
pub const TEXT_BLACK: TextColor = TextColor(Color::BLACK);
pub const TEXT_WHITE: TextColor = TextColor(Color::WHITE);
pub const TEXT_SLATE_50: TextColor = TextColor(Color::Srgba(SLATE_50));
pub const TEXT_SLATE_100: TextColor = TextColor(Color::Srgba(SLATE_100));
pub const TEXT_SLATE_200: TextColor = TextColor(Color::Srgba(SLATE_200));
pub const TEXT_SLATE_300: TextColor = TextColor(Color::Srgba(SLATE_300));
pub const TEXT_SLATE_400: TextColor = TextColor(Color::Srgba(SLATE_400));
pub const TEXT_SLATE_500: TextColor = TextColor(Color::Srgba(SLATE_500));
pub const TEXT_SLATE_600: TextColor = TextColor(Color::Srgba(SLATE_600));
pub const TEXT_SLATE_700: TextColor = TextColor(Color::Srgba(SLATE_700));
pub const TEXT_SLATE_800: TextColor = TextColor(Color::Srgba(SLATE_800));
pub const TEXT_SLATE_900: TextColor = TextColor(Color::Srgba(SLATE_900));
pub const TEXT_SLATE_950: TextColor = TextColor(Color::Srgba(SLATE_950));
pub const TEXT_GRAY_50: TextColor = TextColor(Color::Srgba(GRAY_50));
pub const TEXT_GRAY_100: TextColor = TextColor(Color::Srgba(GRAY_100));
pub const TEXT_GRAY_200: TextColor = TextColor(Color::Srgba(GRAY_200));
pub const TEXT_GRAY_300: TextColor = TextColor(Color::Srgba(GRAY_300));
pub const TEXT_GRAY_400: TextColor = TextColor(Color::Srgba(GRAY_400));
pub const TEXT_GRAY_500: TextColor = TextColor(Color::Srgba(GRAY_500));
pub const TEXT_GRAY_600: TextColor = TextColor(Color::Srgba(GRAY_600));
pub const TEXT_GRAY_700: TextColor = TextColor(Color::Srgba(GRAY_700));
pub const TEXT_GRAY_800: TextColor = TextColor(Color::Srgba(GRAY_800));
pub const TEXT_GRAY_900: TextColor = TextColor(Color::Srgba(GRAY_900));
pub const TEXT_GRAY_950: TextColor = TextColor(Color::Srgba(GRAY_950));
pub const TEXT_ZINC_50: TextColor = TextColor(Color::Srgba(ZINC_50));
pub const TEXT_ZINC_100: TextColor = TextColor(Color::Srgba(ZINC_100));
pub const TEXT_ZINC_200: TextColor = TextColor(Color::Srgba(ZINC_200));
pub const TEXT_ZINC_300: TextColor = TextColor(Color::Srgba(ZINC_300));
pub const TEXT_ZINC_400: TextColor = TextColor(Color::Srgba(ZINC_400));
pub const TEXT_ZINC_500: TextColor = TextColor(Color::Srgba(ZINC_500));
pub const TEXT_ZINC_600: TextColor = TextColor(Color::Srgba(ZINC_600));
pub const TEXT_ZINC_700: TextColor = TextColor(Color::Srgba(ZINC_700));
pub const TEXT_ZINC_800: TextColor = TextColor(Color::Srgba(ZINC_800));
pub const TEXT_ZINC_900: TextColor = TextColor(Color::Srgba(ZINC_900));
pub const TEXT_ZINC_950: TextColor = TextColor(Color::Srgba(ZINC_950));
pub const TEXT_NEUTRAL_50: TextColor = TextColor(Color::Srgba(NEUTRAL_50));
pub const TEXT_NEUTRAL_100: TextColor = TextColor(Color::Srgba(NEUTRAL_100));
pub const TEXT_NEUTRAL_200: TextColor = TextColor(Color::Srgba(NEUTRAL_200));
pub const TEXT_NEUTRAL_300: TextColor = TextColor(Color::Srgba(NEUTRAL_300));
pub const TEXT_NEUTRAL_400: TextColor = TextColor(Color::Srgba(NEUTRAL_400));
pub const TEXT_NEUTRAL_500: TextColor = TextColor(Color::Srgba(NEUTRAL_500));
pub const TEXT_NEUTRAL_600: TextColor = TextColor(Color::Srgba(NEUTRAL_600));
pub const TEXT_NEUTRAL_700: TextColor = TextColor(Color::Srgba(NEUTRAL_700));
pub const TEXT_NEUTRAL_800: TextColor = TextColor(Color::Srgba(NEUTRAL_800));
pub const TEXT_NEUTRAL_900: TextColor = TextColor(Color::Srgba(NEUTRAL_900));
pub const TEXT_NEUTRAL_950: TextColor = TextColor(Color::Srgba(NEUTRAL_950));
pub const TEXT_STONE_50: TextColor = TextColor(Color::Srgba(STONE_50));
pub const TEXT_STONE_100: TextColor = TextColor(Color::Srgba(STONE_100));
pub const TEXT_STONE_200: TextColor = TextColor(Color::Srgba(STONE_200));
pub const TEXT_STONE_300: TextColor = TextColor(Color::Srgba(STONE_300));
pub const TEXT_STONE_400: TextColor = TextColor(Color::Srgba(STONE_400));
pub const TEXT_STONE_500: TextColor = TextColor(Color::Srgba(STONE_500));
pub const TEXT_STONE_600: TextColor = TextColor(Color::Srgba(STONE_600));
pub const TEXT_STONE_700: TextColor = TextColor(Color::Srgba(STONE_700));
pub const TEXT_STONE_800: TextColor = TextColor(Color::Srgba(STONE_800));
pub const TEXT_STONE_900: TextColor = TextColor(Color::Srgba(STONE_900));
pub const TEXT_STONE_950: TextColor = TextColor(Color::Srgba(STONE_950));
pub const TEXT_RED_50: TextColor = TextColor(Color::Srgba(RED_50));
pub const TEXT_RED_100: TextColor = TextColor(Color::Srgba(RED_100));
pub const TEXT_RED_200: TextColor = TextColor(Color::Srgba(RED_200));
pub const TEXT_RED_300: TextColor = TextColor(Color::Srgba(RED_300));
pub const TEXT_RED_400: TextColor = TextColor(Color::Srgba(RED_400));
pub const TEXT_RED_500: TextColor = TextColor(Color::Srgba(RED_500));
pub const TEXT_RED_600: TextColor = TextColor(Color::Srgba(RED_600));
pub const TEXT_RED_700: TextColor = TextColor(Color::Srgba(RED_700));
pub const TEXT_RED_800: TextColor = TextColor(Color::Srgba(RED_800));
pub const TEXT_RED_900: TextColor = TextColor(Color::Srgba(RED_900));
pub const TEXT_RED_950: TextColor = TextColor(Color::Srgba(RED_950));
pub const TEXT_ORANGE_50: TextColor = TextColor(Color::Srgba(ORANGE_50));
pub const TEXT_ORANGE_100: TextColor = TextColor(Color::Srgba(ORANGE_100));
pub const TEXT_ORANGE_200: TextColor = TextColor(Color::Srgba(ORANGE_200));
pub const TEXT_ORANGE_300: TextColor = TextColor(Color::Srgba(ORANGE_300));
pub const TEXT_ORANGE_400: TextColor = TextColor(Color::Srgba(ORANGE_400));
pub const TEXT_ORANGE_500: TextColor = TextColor(Color::Srgba(ORANGE_500));
pub const TEXT_ORANGE_600: TextColor = TextColor(Color::Srgba(ORANGE_600));
pub const TEXT_ORANGE_700: TextColor = TextColor(Color::Srgba(ORANGE_700));
pub const TEXT_ORANGE_800: TextColor = TextColor(Color::Srgba(ORANGE_800));
pub const TEXT_ORANGE_900: TextColor = TextColor(Color::Srgba(ORANGE_900));
pub const TEXT_ORANGE_950: TextColor = TextColor(Color::Srgba(ORANGE_950));
pub const TEXT_AMBER_50: TextColor = TextColor(Color::Srgba(AMBER_50));
pub const TEXT_AMBER_100: TextColor = TextColor(Color::Srgba(AMBER_100));
pub const TEXT_AMBER_200: TextColor = TextColor(Color::Srgba(AMBER_200));
pub const TEXT_AMBER_300: TextColor = TextColor(Color::Srgba(AMBER_300));
pub const TEXT_AMBER_400: TextColor = TextColor(Color::Srgba(AMBER_400));
pub const TEXT_AMBER_500: TextColor = TextColor(Color::Srgba(AMBER_500));
pub const TEXT_AMBER_600: TextColor = TextColor(Color::Srgba(AMBER_600));
pub const TEXT_AMBER_700: TextColor = TextColor(Color::Srgba(AMBER_700));
pub const TEXT_AMBER_800: TextColor = TextColor(Color::Srgba(AMBER_800));
pub const TEXT_AMBER_900: TextColor = TextColor(Color::Srgba(AMBER_900));
pub const TEXT_AMBER_950: TextColor = TextColor(Color::Srgba(AMBER_950));
pub const TEXT_YELLOW_50: TextColor = TextColor(Color::Srgba(YELLOW_50));
pub const TEXT_YELLOW_100: TextColor = TextColor(Color::Srgba(YELLOW_100));
pub const TEXT_YELLOW_200: TextColor = TextColor(Color::Srgba(YELLOW_200));
pub const TEXT_YELLOW_300: TextColor = TextColor(Color::Srgba(YELLOW_300));
pub const TEXT_YELLOW_400: TextColor = TextColor(Color::Srgba(YELLOW_400));
pub const TEXT_YELLOW_500: TextColor = TextColor(Color::Srgba(YELLOW_500));
pub const TEXT_YELLOW_600: TextColor = TextColor(Color::Srgba(YELLOW_600));
pub const TEXT_YELLOW_700: TextColor = TextColor(Color::Srgba(YELLOW_700));
pub const TEXT_YELLOW_800: TextColor = TextColor(Color::Srgba(YELLOW_800));
pub const TEXT_YELLOW_900: TextColor = TextColor(Color::Srgba(YELLOW_900));
pub const TEXT_YELLOW_950: TextColor = TextColor(Color::Srgba(YELLOW_950));
pub const TEXT_LIME_50: TextColor = TextColor(Color::Srgba(LIME_50));
pub const TEXT_LIME_100: TextColor = TextColor(Color::Srgba(LIME_100));
pub const TEXT_LIME_200: TextColor = TextColor(Color::Srgba(LIME_200));
pub const TEXT_LIME_300: TextColor = TextColor(Color::Srgba(LIME_300));
pub const TEXT_LIME_400: TextColor = TextColor(Color::Srgba(LIME_400));
pub const TEXT_LIME_500: TextColor = TextColor(Color::Srgba(LIME_500));
pub const TEXT_LIME_600: TextColor = TextColor(Color::Srgba(LIME_600));
pub const TEXT_LIME_700: TextColor = TextColor(Color::Srgba(LIME_700));
pub const TEXT_LIME_800: TextColor = TextColor(Color::Srgba(LIME_800));
pub const TEXT_LIME_900: TextColor = TextColor(Color::Srgba(LIME_900));
pub const TEXT_LIME_950: TextColor = TextColor(Color::Srgba(LIME_950));
pub const TEXT_GREEN_50: TextColor = TextColor(Color::Srgba(GREEN_50));
pub const TEXT_GREEN_100: TextColor = TextColor(Color::Srgba(GREEN_100));
pub const TEXT_GREEN_200: TextColor = TextColor(Color::Srgba(GREEN_200));
pub const TEXT_GREEN_300: TextColor = TextColor(Color::Srgba(GREEN_300));
pub const TEXT_GREEN_400: TextColor = TextColor(Color::Srgba(GREEN_400));
pub const TEXT_GREEN_500: TextColor = TextColor(Color::Srgba(GREEN_500));
pub const TEXT_GREEN_600: TextColor = TextColor(Color::Srgba(GREEN_600));
pub const TEXT_GREEN_700: TextColor = TextColor(Color::Srgba(GREEN_700));
pub const TEXT_GREEN_800: TextColor = TextColor(Color::Srgba(GREEN_800));
pub const TEXT_GREEN_900: TextColor = TextColor(Color::Srgba(GREEN_900));
pub const TEXT_GREEN_950: TextColor = TextColor(Color::Srgba(GREEN_950));
pub const TEXT_EMERALD_50: TextColor = TextColor(Color::Srgba(EMERALD_50));
pub const TEXT_EMERALD_100: TextColor = TextColor(Color::Srgba(EMERALD_100));
pub const TEXT_EMERALD_200: TextColor = TextColor(Color::Srgba(EMERALD_200));
pub const TEXT_EMERALD_300: TextColor = TextColor(Color::Srgba(EMERALD_300));
pub const TEXT_EMERALD_400: TextColor = TextColor(Color::Srgba(EMERALD_400));
pub const TEXT_EMERALD_500: TextColor = TextColor(Color::Srgba(EMERALD_500));
pub const TEXT_EMERALD_600: TextColor = TextColor(Color::Srgba(EMERALD_600));
pub const TEXT_EMERALD_700: TextColor = TextColor(Color::Srgba(EMERALD_700));
pub const TEXT_EMERALD_800: TextColor = TextColor(Color::Srgba(EMERALD_800));
pub const TEXT_EMERALD_900: TextColor = TextColor(Color::Srgba(EMERALD_900));
pub const TEXT_EMERALD_950: TextColor = TextColor(Color::Srgba(EMERALD_950));
pub const TEXT_TEAL_50: TextColor = TextColor(Color::Srgba(TEAL_50));
pub const TEXT_TEAL_100: TextColor = TextColor(Color::Srgba(TEAL_100));
pub const TEXT_TEAL_200: TextColor = TextColor(Color::Srgba(TEAL_200));
pub const TEXT_TEAL_300: TextColor = TextColor(Color::Srgba(TEAL_300));
pub const TEXT_TEAL_400: TextColor = TextColor(Color::Srgba(TEAL_400));
pub const TEXT_TEAL_500: TextColor = TextColor(Color::Srgba(TEAL_500));
pub const TEXT_TEAL_600: TextColor = TextColor(Color::Srgba(TEAL_600));
pub const TEXT_TEAL_700: TextColor = TextColor(Color::Srgba(TEAL_700));
pub const TEXT_TEAL_800: TextColor = TextColor(Color::Srgba(TEAL_800));
pub const TEXT_TEAL_900: TextColor = TextColor(Color::Srgba(TEAL_900));
pub const TEXT_TEAL_950: TextColor = TextColor(Color::Srgba(TEAL_950));
pub const TEXT_CYAN_50: TextColor = TextColor(Color::Srgba(CYAN_50));
pub const TEXT_CYAN_100: TextColor = TextColor(Color::Srgba(CYAN_100));
pub const TEXT_CYAN_200: TextColor = TextColor(Color::Srgba(CYAN_200));
pub const TEXT_CYAN_300: TextColor = TextColor(Color::Srgba(CYAN_300));
pub const TEXT_CYAN_400: TextColor = TextColor(Color::Srgba(CYAN_400));
pub const TEXT_CYAN_500: TextColor = TextColor(Color::Srgba(CYAN_500));
pub const TEXT_CYAN_600: TextColor = TextColor(Color::Srgba(CYAN_600));
pub const TEXT_CYAN_700: TextColor = TextColor(Color::Srgba(CYAN_700));
pub const TEXT_CYAN_800: TextColor = TextColor(Color::Srgba(CYAN_800));
pub const TEXT_CYAN_900: TextColor = TextColor(Color::Srgba(CYAN_900));
pub const TEXT_CYAN_950: TextColor = TextColor(Color::Srgba(CYAN_950));
pub const TEXT_SKY_50: TextColor = TextColor(Color::Srgba(SKY_50));
pub const TEXT_SKY_100: TextColor = TextColor(Color::Srgba(SKY_100));
pub const TEXT_SKY_200: TextColor = TextColor(Color::Srgba(SKY_200));
pub const TEXT_SKY_300: TextColor = TextColor(Color::Srgba(SKY_300));
pub const TEXT_SKY_400: TextColor = TextColor(Color::Srgba(SKY_400));
pub const TEXT_SKY_500: TextColor = TextColor(Color::Srgba(SKY_500));
pub const TEXT_SKY_600: TextColor = TextColor(Color::Srgba(SKY_600));
pub const TEXT_SKY_700: TextColor = TextColor(Color::Srgba(SKY_700));
pub const TEXT_SKY_800: TextColor = TextColor(Color::Srgba(SKY_800));
pub const TEXT_SKY_900: TextColor = TextColor(Color::Srgba(SKY_900));
pub const TEXT_SKY_950: TextColor = TextColor(Color::Srgba(SKY_950));
pub const TEXT_BLUE_50: TextColor = TextColor(Color::Srgba(BLUE_50));
pub const TEXT_BLUE_100: TextColor = TextColor(Color::Srgba(BLUE_100));
pub const TEXT_BLUE_200: TextColor = TextColor(Color::Srgba(BLUE_200));
pub const TEXT_BLUE_300: TextColor = TextColor(Color::Srgba(BLUE_300));
pub const TEXT_BLUE_400: TextColor = TextColor(Color::Srgba(BLUE_400));
pub const TEXT_BLUE_500: TextColor = TextColor(Color::Srgba(BLUE_500));
pub const TEXT_BLUE_600: TextColor = TextColor(Color::Srgba(BLUE_600));
pub const TEXT_BLUE_700: TextColor = TextColor(Color::Srgba(BLUE_700));
pub const TEXT_BLUE_800: TextColor = TextColor(Color::Srgba(BLUE_800));
pub const TEXT_BLUE_900: TextColor = TextColor(Color::Srgba(BLUE_900));
pub const TEXT_BLUE_950: TextColor = TextColor(Color::Srgba(BLUE_950));
pub const TEXT_INDIGO_50: TextColor = TextColor(Color::Srgba(INDIGO_50));
pub const TEXT_INDIGO_100: TextColor = TextColor(Color::Srgba(INDIGO_100));
pub const TEXT_INDIGO_200: TextColor = TextColor(Color::Srgba(INDIGO_200));
pub const TEXT_INDIGO_300: TextColor = TextColor(Color::Srgba(INDIGO_300));
pub const TEXT_INDIGO_400: TextColor = TextColor(Color::Srgba(INDIGO_400));
pub const TEXT_INDIGO_500: TextColor = TextColor(Color::Srgba(INDIGO_500));
pub const TEXT_INDIGO_600: TextColor = TextColor(Color::Srgba(INDIGO_600));
pub const TEXT_INDIGO_700: TextColor = TextColor(Color::Srgba(INDIGO_700));
pub const TEXT_INDIGO_800: TextColor = TextColor(Color::Srgba(INDIGO_800));
pub const TEXT_INDIGO_900: TextColor = TextColor(Color::Srgba(INDIGO_900));
pub const TEXT_INDIGO_950: TextColor = TextColor(Color::Srgba(INDIGO_950));
pub const TEXT_VIOLET_50: TextColor = TextColor(Color::Srgba(VIOLET_50));
pub const TEXT_VIOLET_100: TextColor = TextColor(Color::Srgba(VIOLET_100));
pub const TEXT_VIOLET_200: TextColor = TextColor(Color::Srgba(VIOLET_200));
pub const TEXT_VIOLET_300: TextColor = TextColor(Color::Srgba(VIOLET_300));
pub const TEXT_VIOLET_400: TextColor = TextColor(Color::Srgba(VIOLET_400));
pub const TEXT_VIOLET_500: TextColor = TextColor(Color::Srgba(VIOLET_500));
pub const TEXT_VIOLET_600: TextColor = TextColor(Color::Srgba(VIOLET_600));
pub const TEXT_VIOLET_700: TextColor = TextColor(Color::Srgba(VIOLET_700));
pub const TEXT_VIOLET_800: TextColor = TextColor(Color::Srgba(VIOLET_800));
pub const TEXT_VIOLET_900: TextColor = TextColor(Color::Srgba(VIOLET_900));
pub const TEXT_VIOLET_950: TextColor = TextColor(Color::Srgba(VIOLET_950));
pub const TEXT_PURPLE_50: TextColor = TextColor(Color::Srgba(PURPLE_50));
pub const TEXT_PURPLE_100: TextColor = TextColor(Color::Srgba(PURPLE_100));
pub const TEXT_PURPLE_200: TextColor = TextColor(Color::Srgba(PURPLE_200));
pub const TEXT_PURPLE_300: TextColor = TextColor(Color::Srgba(PURPLE_300));
pub const TEXT_PURPLE_400: TextColor = TextColor(Color::Srgba(PURPLE_400));
pub const TEXT_PURPLE_500: TextColor = TextColor(Color::Srgba(PURPLE_500));
pub const TEXT_PURPLE_600: TextColor = TextColor(Color::Srgba(PURPLE_600));
pub const TEXT_PURPLE_700: TextColor = TextColor(Color::Srgba(PURPLE_700));
pub const TEXT_PURPLE_800: TextColor = TextColor(Color::Srgba(PURPLE_800));
pub const TEXT_PURPLE_900: TextColor = TextColor(Color::Srgba(PURPLE_900));
pub const TEXT_PURPLE_950: TextColor = TextColor(Color::Srgba(PURPLE_950));
pub const TEXT_FUCHSIA_50: TextColor = TextColor(Color::Srgba(FUCHSIA_50));
pub const TEXT_FUCHSIA_100: TextColor = TextColor(Color::Srgba(FUCHSIA_100));
pub const TEXT_FUCHSIA_200: TextColor = TextColor(Color::Srgba(FUCHSIA_200));
pub const TEXT_FUCHSIA_300: TextColor = TextColor(Color::Srgba(FUCHSIA_300));
pub const TEXT_FUCHSIA_400: TextColor = TextColor(Color::Srgba(FUCHSIA_400));
pub const TEXT_FUCHSIA_500: TextColor = TextColor(Color::Srgba(FUCHSIA_500));
pub const TEXT_FUCHSIA_600: TextColor = TextColor(Color::Srgba(FUCHSIA_600));
pub const TEXT_FUCHSIA_700: TextColor = TextColor(Color::Srgba(FUCHSIA_700));
pub const TEXT_FUCHSIA_800: TextColor = TextColor(Color::Srgba(FUCHSIA_800));
pub const TEXT_FUCHSIA_900: TextColor = TextColor(Color::Srgba(FUCHSIA_900));
pub const TEXT_FUCHSIA_950: TextColor = TextColor(Color::Srgba(FUCHSIA_950));
pub const TEXT_PINK_50: TextColor = TextColor(Color::Srgba(PINK_50));
pub const TEXT_PINK_100: TextColor = TextColor(Color::Srgba(PINK_100));
pub const TEXT_PINK_200: TextColor = TextColor(Color::Srgba(PINK_200));
pub const TEXT_PINK_300: TextColor = TextColor(Color::Srgba(PINK_300));
pub const TEXT_PINK_400: TextColor = TextColor(Color::Srgba(PINK_400));
pub const TEXT_PINK_500: TextColor = TextColor(Color::Srgba(PINK_500));
pub const TEXT_PINK_600: TextColor = TextColor(Color::Srgba(PINK_600));
pub const TEXT_PINK_700: TextColor = TextColor(Color::Srgba(PINK_700));
pub const TEXT_PINK_800: TextColor = TextColor(Color::Srgba(PINK_800));
pub const TEXT_PINK_900: TextColor = TextColor(Color::Srgba(PINK_900));
pub const TEXT_PINK_950: TextColor = TextColor(Color::Srgba(PINK_950));
pub const TEXT_ROSE_50: TextColor = TextColor(Color::Srgba(ROSE_50));
pub const TEXT_ROSE_100: TextColor = TextColor(Color::Srgba(ROSE_100));
pub const TEXT_ROSE_200: TextColor = TextColor(Color::Srgba(ROSE_200));
pub const TEXT_ROSE_300: TextColor = TextColor(Color::Srgba(ROSE_300));
pub const TEXT_ROSE_400: TextColor = TextColor(Color::Srgba(ROSE_400));
pub const TEXT_ROSE_500: TextColor = TextColor(Color::Srgba(ROSE_500));
pub const TEXT_ROSE_600: TextColor = TextColor(Color::Srgba(ROSE_600));
pub const TEXT_ROSE_700: TextColor = TextColor(Color::Srgba(ROSE_700));
pub const TEXT_ROSE_800: TextColor = TextColor(Color::Srgba(ROSE_800));
pub const TEXT_ROSE_900: TextColor = TextColor(Color::Srgba(ROSE_900));
pub const TEXT_ROSE_950: TextColor = TextColor(Color::Srgba(ROSE_950));

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TextColor(Color);

impl ApplyClass<TextColor> for Text {
    fn apply_class(&mut self, class: &TextColor) {
        self.sections[0].style.color = class.0;
    }
}
