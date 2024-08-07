use crate::class::ApplyClass;
use bevy_color::{palettes::tailwind::*, Alpha, Color};
use bevy_ui::BackgroundColor;
use std::ops::Div;

pub fn bg_color(color: impl Into<Color>) -> BackgroundColorClass {
    BackgroundColorClass(color.into())
}

pub const BG_TRANSPARENT: BackgroundColorClass = BackgroundColorClass(Color::NONE);
pub const BG_BLACK: BackgroundColorClass = BackgroundColorClass(Color::BLACK);
pub const BG_WHITE: BackgroundColorClass = BackgroundColorClass(Color::WHITE);
pub const BG_SLATE_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SLATE_50));
pub const BG_SLATE_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SLATE_100));
pub const BG_SLATE_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SLATE_200));
pub const BG_SLATE_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SLATE_300));
pub const BG_SLATE_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SLATE_400));
pub const BG_SLATE_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SLATE_500));
pub const BG_SLATE_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SLATE_600));
pub const BG_SLATE_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SLATE_700));
pub const BG_SLATE_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SLATE_800));
pub const BG_SLATE_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SLATE_900));
pub const BG_SLATE_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SLATE_950));
pub const BG_GRAY_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GRAY_50));
pub const BG_GRAY_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GRAY_100));
pub const BG_GRAY_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GRAY_200));
pub const BG_GRAY_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GRAY_300));
pub const BG_GRAY_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GRAY_400));
pub const BG_GRAY_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GRAY_500));
pub const BG_GRAY_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GRAY_600));
pub const BG_GRAY_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GRAY_700));
pub const BG_GRAY_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GRAY_800));
pub const BG_GRAY_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GRAY_900));
pub const BG_GRAY_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GRAY_950));
pub const BG_ZINC_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ZINC_50));
pub const BG_ZINC_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ZINC_100));
pub const BG_ZINC_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ZINC_200));
pub const BG_ZINC_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ZINC_300));
pub const BG_ZINC_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ZINC_400));
pub const BG_ZINC_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ZINC_500));
pub const BG_ZINC_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ZINC_600));
pub const BG_ZINC_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ZINC_700));
pub const BG_ZINC_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ZINC_800));
pub const BG_ZINC_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ZINC_900));
pub const BG_ZINC_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ZINC_950));
pub const BG_NEUTRAL_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(NEUTRAL_50));
pub const BG_NEUTRAL_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(NEUTRAL_100));
pub const BG_NEUTRAL_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(NEUTRAL_200));
pub const BG_NEUTRAL_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(NEUTRAL_300));
pub const BG_NEUTRAL_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(NEUTRAL_400));
pub const BG_NEUTRAL_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(NEUTRAL_500));
pub const BG_NEUTRAL_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(NEUTRAL_600));
pub const BG_NEUTRAL_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(NEUTRAL_700));
pub const BG_NEUTRAL_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(NEUTRAL_800));
pub const BG_NEUTRAL_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(NEUTRAL_900));
pub const BG_NEUTRAL_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(NEUTRAL_950));
pub const BG_STONE_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(STONE_50));
pub const BG_STONE_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(STONE_100));
pub const BG_STONE_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(STONE_200));
pub const BG_STONE_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(STONE_300));
pub const BG_STONE_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(STONE_400));
pub const BG_STONE_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(STONE_500));
pub const BG_STONE_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(STONE_600));
pub const BG_STONE_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(STONE_700));
pub const BG_STONE_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(STONE_800));
pub const BG_STONE_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(STONE_900));
pub const BG_STONE_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(STONE_950));
pub const BG_RED_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(RED_50));
pub const BG_RED_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(RED_100));
pub const BG_RED_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(RED_200));
pub const BG_RED_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(RED_300));
pub const BG_RED_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(RED_400));
pub const BG_RED_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(RED_500));
pub const BG_RED_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(RED_600));
pub const BG_RED_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(RED_700));
pub const BG_RED_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(RED_800));
pub const BG_RED_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(RED_900));
pub const BG_RED_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(RED_950));
pub const BG_ORANGE_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ORANGE_50));
pub const BG_ORANGE_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ORANGE_100));
pub const BG_ORANGE_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ORANGE_200));
pub const BG_ORANGE_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ORANGE_300));
pub const BG_ORANGE_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ORANGE_400));
pub const BG_ORANGE_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ORANGE_500));
pub const BG_ORANGE_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ORANGE_600));
pub const BG_ORANGE_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ORANGE_700));
pub const BG_ORANGE_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ORANGE_800));
pub const BG_ORANGE_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ORANGE_900));
pub const BG_ORANGE_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ORANGE_950));
pub const BG_AMBER_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(AMBER_50));
pub const BG_AMBER_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(AMBER_100));
pub const BG_AMBER_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(AMBER_200));
pub const BG_AMBER_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(AMBER_300));
pub const BG_AMBER_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(AMBER_400));
pub const BG_AMBER_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(AMBER_500));
pub const BG_AMBER_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(AMBER_600));
pub const BG_AMBER_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(AMBER_700));
pub const BG_AMBER_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(AMBER_800));
pub const BG_AMBER_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(AMBER_900));
pub const BG_AMBER_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(AMBER_950));
pub const BG_YELLOW_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(YELLOW_50));
pub const BG_YELLOW_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(YELLOW_100));
pub const BG_YELLOW_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(YELLOW_200));
pub const BG_YELLOW_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(YELLOW_300));
pub const BG_YELLOW_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(YELLOW_400));
pub const BG_YELLOW_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(YELLOW_500));
pub const BG_YELLOW_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(YELLOW_600));
pub const BG_YELLOW_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(YELLOW_700));
pub const BG_YELLOW_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(YELLOW_800));
pub const BG_YELLOW_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(YELLOW_900));
pub const BG_YELLOW_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(YELLOW_950));
pub const BG_LIME_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(LIME_50));
pub const BG_LIME_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(LIME_100));
pub const BG_LIME_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(LIME_200));
pub const BG_LIME_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(LIME_300));
pub const BG_LIME_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(LIME_400));
pub const BG_LIME_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(LIME_500));
pub const BG_LIME_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(LIME_600));
pub const BG_LIME_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(LIME_700));
pub const BG_LIME_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(LIME_800));
pub const BG_LIME_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(LIME_900));
pub const BG_LIME_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(LIME_950));
pub const BG_GREEN_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GREEN_50));
pub const BG_GREEN_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GREEN_100));
pub const BG_GREEN_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GREEN_200));
pub const BG_GREEN_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GREEN_300));
pub const BG_GREEN_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GREEN_400));
pub const BG_GREEN_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GREEN_500));
pub const BG_GREEN_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GREEN_600));
pub const BG_GREEN_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GREEN_700));
pub const BG_GREEN_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GREEN_800));
pub const BG_GREEN_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GREEN_900));
pub const BG_GREEN_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(GREEN_950));
pub const BG_EMERALD_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(EMERALD_50));
pub const BG_EMERALD_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(EMERALD_100));
pub const BG_EMERALD_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(EMERALD_200));
pub const BG_EMERALD_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(EMERALD_300));
pub const BG_EMERALD_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(EMERALD_400));
pub const BG_EMERALD_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(EMERALD_500));
pub const BG_EMERALD_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(EMERALD_600));
pub const BG_EMERALD_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(EMERALD_700));
pub const BG_EMERALD_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(EMERALD_800));
pub const BG_EMERALD_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(EMERALD_900));
pub const BG_EMERALD_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(EMERALD_950));
pub const BG_TEAL_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(TEAL_50));
pub const BG_TEAL_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(TEAL_100));
pub const BG_TEAL_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(TEAL_200));
pub const BG_TEAL_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(TEAL_300));
pub const BG_TEAL_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(TEAL_400));
pub const BG_TEAL_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(TEAL_500));
pub const BG_TEAL_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(TEAL_600));
pub const BG_TEAL_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(TEAL_700));
pub const BG_TEAL_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(TEAL_800));
pub const BG_TEAL_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(TEAL_900));
pub const BG_TEAL_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(TEAL_950));
pub const BG_CYAN_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(CYAN_50));
pub const BG_CYAN_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(CYAN_100));
pub const BG_CYAN_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(CYAN_200));
pub const BG_CYAN_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(CYAN_300));
pub const BG_CYAN_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(CYAN_400));
pub const BG_CYAN_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(CYAN_500));
pub const BG_CYAN_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(CYAN_600));
pub const BG_CYAN_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(CYAN_700));
pub const BG_CYAN_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(CYAN_800));
pub const BG_CYAN_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(CYAN_900));
pub const BG_CYAN_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(CYAN_950));
pub const BG_SKY_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SKY_50));
pub const BG_SKY_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SKY_100));
pub const BG_SKY_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SKY_200));
pub const BG_SKY_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SKY_300));
pub const BG_SKY_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SKY_400));
pub const BG_SKY_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SKY_500));
pub const BG_SKY_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SKY_600));
pub const BG_SKY_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SKY_700));
pub const BG_SKY_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SKY_800));
pub const BG_SKY_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SKY_900));
pub const BG_SKY_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(SKY_950));
pub const BG_BLUE_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(BLUE_50));
pub const BG_BLUE_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(BLUE_100));
pub const BG_BLUE_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(BLUE_200));
pub const BG_BLUE_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(BLUE_300));
pub const BG_BLUE_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(BLUE_400));
pub const BG_BLUE_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(BLUE_500));
pub const BG_BLUE_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(BLUE_600));
pub const BG_BLUE_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(BLUE_700));
pub const BG_BLUE_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(BLUE_800));
pub const BG_BLUE_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(BLUE_900));
pub const BG_BLUE_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(BLUE_950));
pub const BG_INDIGO_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(INDIGO_50));
pub const BG_INDIGO_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(INDIGO_100));
pub const BG_INDIGO_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(INDIGO_200));
pub const BG_INDIGO_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(INDIGO_300));
pub const BG_INDIGO_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(INDIGO_400));
pub const BG_INDIGO_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(INDIGO_500));
pub const BG_INDIGO_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(INDIGO_600));
pub const BG_INDIGO_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(INDIGO_700));
pub const BG_INDIGO_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(INDIGO_800));
pub const BG_INDIGO_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(INDIGO_900));
pub const BG_INDIGO_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(INDIGO_950));
pub const BG_VIOLET_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(VIOLET_50));
pub const BG_VIOLET_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(VIOLET_100));
pub const BG_VIOLET_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(VIOLET_200));
pub const BG_VIOLET_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(VIOLET_300));
pub const BG_VIOLET_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(VIOLET_400));
pub const BG_VIOLET_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(VIOLET_500));
pub const BG_VIOLET_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(VIOLET_600));
pub const BG_VIOLET_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(VIOLET_700));
pub const BG_VIOLET_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(VIOLET_800));
pub const BG_VIOLET_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(VIOLET_900));
pub const BG_VIOLET_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(VIOLET_950));
pub const BG_PURPLE_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PURPLE_50));
pub const BG_PURPLE_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PURPLE_100));
pub const BG_PURPLE_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PURPLE_200));
pub const BG_PURPLE_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PURPLE_300));
pub const BG_PURPLE_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PURPLE_400));
pub const BG_PURPLE_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PURPLE_500));
pub const BG_PURPLE_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PURPLE_600));
pub const BG_PURPLE_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PURPLE_700));
pub const BG_PURPLE_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PURPLE_800));
pub const BG_PURPLE_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PURPLE_900));
pub const BG_PURPLE_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PURPLE_950));
pub const BG_FUCHSIA_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(FUCHSIA_50));
pub const BG_FUCHSIA_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(FUCHSIA_100));
pub const BG_FUCHSIA_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(FUCHSIA_200));
pub const BG_FUCHSIA_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(FUCHSIA_300));
pub const BG_FUCHSIA_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(FUCHSIA_400));
pub const BG_FUCHSIA_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(FUCHSIA_500));
pub const BG_FUCHSIA_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(FUCHSIA_600));
pub const BG_FUCHSIA_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(FUCHSIA_700));
pub const BG_FUCHSIA_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(FUCHSIA_800));
pub const BG_FUCHSIA_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(FUCHSIA_900));
pub const BG_FUCHSIA_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(FUCHSIA_950));
pub const BG_PINK_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PINK_50));
pub const BG_PINK_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PINK_100));
pub const BG_PINK_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PINK_200));
pub const BG_PINK_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PINK_300));
pub const BG_PINK_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PINK_400));
pub const BG_PINK_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PINK_500));
pub const BG_PINK_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PINK_600));
pub const BG_PINK_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PINK_700));
pub const BG_PINK_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PINK_800));
pub const BG_PINK_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PINK_900));
pub const BG_PINK_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(PINK_950));
pub const BG_ROSE_50: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ROSE_50));
pub const BG_ROSE_100: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ROSE_100));
pub const BG_ROSE_200: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ROSE_200));
pub const BG_ROSE_300: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ROSE_300));
pub const BG_ROSE_400: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ROSE_400));
pub const BG_ROSE_500: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ROSE_500));
pub const BG_ROSE_600: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ROSE_600));
pub const BG_ROSE_700: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ROSE_700));
pub const BG_ROSE_800: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ROSE_800));
pub const BG_ROSE_900: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ROSE_900));
pub const BG_ROSE_950: BackgroundColorClass = BackgroundColorClass(Color::Srgba(ROSE_950));

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct BackgroundColorClass(Color);

impl Default for BackgroundColorClass {
    fn default() -> Self {
        Self(Color::WHITE)
    }
}

impl Div<f32> for BackgroundColorClass {
    type Output = BackgroundColorClass;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0.with_alpha(rhs))
    }
}

impl From<Color> for BackgroundColorClass {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

impl From<BackgroundColor> for BackgroundColorClass {
    fn from(color: BackgroundColor) -> Self {
        Self(color.0)
    }
}

impl ApplyClass<BackgroundColorClass> for BackgroundColor {
    fn apply_class(&mut self, class: &BackgroundColorClass) {
        self.0 = class.0;
    }
}
