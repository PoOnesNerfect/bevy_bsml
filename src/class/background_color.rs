use crate::class::ApplyClass;
use bevy::{prelude::Color, ui::BackgroundColor};
use std::ops::Div;

pub fn bg_color<C: Into<Color>>(color: C) -> BackgroundColorClass {
    BackgroundColorClass(color.into())
}

pub const BG_TRANSPARENT: BackgroundColorClass = BackgroundColorClass(Color::NONE);
pub const BG_BLACK: BackgroundColorClass = BackgroundColorClass(Color::BLACK);
pub const BG_WHITE: BackgroundColorClass = BackgroundColorClass(Color::WHITE);
pub const BG_SLATE_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(248.0 / 255.0, 250.0 / 255.0, 252.0 / 255.0));
pub const BG_SLATE_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(241.0 / 255.0, 245.0 / 255.0, 249.0 / 255.0));
pub const BG_SLATE_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(226.0 / 255.0, 232.0 / 255.0, 240.0 / 255.0));
pub const BG_SLATE_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(203.0 / 255.0, 213.0 / 255.0, 225.0 / 255.0));
pub const BG_SLATE_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(148.0 / 255.0, 163.0 / 255.0, 184.0 / 255.0));
pub const BG_SLATE_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(100.0 / 255.0, 116.0 / 255.0, 139.0 / 255.0));
pub const BG_SLATE_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(71.0 / 255.0, 85.0 / 255.0, 105.0 / 255.0));
pub const BG_SLATE_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(51.0 / 255.0, 65.0 / 255.0, 85.0 / 255.0));
pub const BG_SLATE_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(30.0 / 255.0, 41.0 / 255.0, 59.0 / 255.0));
pub const BG_SLATE_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(15.0 / 255.0, 23.0 / 255.0, 42.0 / 255.0));
pub const BG_SLATE_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(2.0 / 255.0, 6.0 / 255.0, 23.0 / 255.0));
pub const BG_GRAY_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(249.0 / 255.0, 250.0 / 255.0, 251.0 / 255.0));
pub const BG_GRAY_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(243.0 / 255.0, 244.0 / 255.0, 246.0 / 255.0));
pub const BG_GRAY_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(229.0 / 255.0, 231.0 / 255.0, 235.0 / 255.0));
pub const BG_GRAY_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(209.0 / 255.0, 213.0 / 255.0, 219.0 / 255.0));
pub const BG_GRAY_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(156.0 / 255.0, 163.0 / 255.0, 175.0 / 255.0));
pub const BG_GRAY_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(107.0 / 255.0, 114.0 / 255.0, 128.0 / 255.0));
pub const BG_GRAY_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(75.0 / 255.0, 85.0 / 255.0, 99.0 / 255.0));
pub const BG_GRAY_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(55.0 / 255.0, 65.0 / 255.0, 81.0 / 255.0));
pub const BG_GRAY_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(31.0 / 255.0, 41.0 / 255.0, 55.0 / 255.0));
pub const BG_GRAY_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(17.0 / 255.0, 24.0 / 255.0, 39.0 / 255.0));
pub const BG_GRAY_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(3.0 / 255.0, 7.0 / 255.0, 18.0 / 255.0));
pub const BG_ZINC_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(250.0 / 255.0, 250.0 / 255.0, 250.0 / 255.0));
pub const BG_ZINC_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(244.0 / 255.0, 244.0 / 255.0, 245.0 / 255.0));
pub const BG_ZINC_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(228.0 / 255.0, 228.0 / 255.0, 231.0 / 255.0));
pub const BG_ZINC_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(212.0 / 255.0, 212.0 / 255.0, 216.0 / 255.0));
pub const BG_ZINC_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(161.0 / 255.0, 161.0 / 255.0, 170.0 / 255.0));
pub const BG_ZINC_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(113.0 / 255.0, 113.0 / 255.0, 122.0 / 255.0));
pub const BG_ZINC_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(82.0 / 255.0, 82.0 / 255.0, 91.0 / 255.0));
pub const BG_ZINC_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(63.0 / 255.0, 63.0 / 255.0, 70.0 / 255.0));
pub const BG_ZINC_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(39.0 / 255.0, 39.0 / 255.0, 42.0 / 255.0));
pub const BG_ZINC_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(24.0 / 255.0, 24.0 / 255.0, 27.0 / 255.0));
pub const BG_ZINC_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(9.0 / 255.0, 9.0 / 255.0, 11.0 / 255.0));
pub const BG_NEUTRAL_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(250.0 / 255.0, 250.0 / 255.0, 250.0 / 255.0));
pub const BG_NEUTRAL_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(245.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0));
pub const BG_NEUTRAL_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(229.0 / 255.0, 229.0 / 255.0, 229.0 / 255.0));
pub const BG_NEUTRAL_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(212.0 / 255.0, 212.0 / 255.0, 212.0 / 255.0));
pub const BG_NEUTRAL_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(163.0 / 255.0, 163.0 / 255.0, 163.0 / 255.0));
pub const BG_NEUTRAL_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(115.0 / 255.0, 115.0 / 255.0, 115.0 / 255.0));
pub const BG_NEUTRAL_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(82.0 / 255.0, 82.0 / 255.0, 82.0 / 255.0));
pub const BG_NEUTRAL_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(64.0 / 255.0, 64.0 / 255.0, 64.0 / 255.0));
pub const BG_NEUTRAL_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(38.0 / 255.0, 38.0 / 255.0, 38.0 / 255.0));
pub const BG_NEUTRAL_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(23.0 / 255.0, 23.0 / 255.0, 23.0 / 255.0));
pub const BG_NEUTRAL_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(10.0 / 255.0, 10.0 / 255.0, 10.0 / 255.0));
pub const BG_STONE_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(250.0 / 255.0, 250.0 / 255.0, 249.0 / 255.0));
pub const BG_STONE_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(245.0 / 255.0, 245.0 / 255.0, 244.0 / 255.0));
pub const BG_STONE_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(231.0 / 255.0, 229.0 / 255.0, 228.0 / 255.0));
pub const BG_STONE_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(214.0 / 255.0, 211.0 / 255.0, 209.0 / 255.0));
pub const BG_STONE_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(168.0 / 255.0, 162.0 / 255.0, 158.0 / 255.0));
pub const BG_STONE_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(120.0 / 255.0, 113.0 / 255.0, 108.0 / 255.0));
pub const BG_STONE_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(87.0 / 255.0, 83.0 / 255.0, 78.0 / 255.0));
pub const BG_STONE_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(68.0 / 255.0, 64.0 / 255.0, 60.0 / 255.0));
pub const BG_STONE_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(41.0 / 255.0, 37.0 / 255.0, 36.0 / 255.0));
pub const BG_STONE_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(28.0 / 255.0, 25.0 / 255.0, 23.0 / 255.0));
pub const BG_STONE_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(12.0 / 255.0, 10.0 / 255.0, 9.0 / 255.0));
pub const BG_RED_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(254.0 / 255.0, 242.0 / 255.0, 242.0 / 255.0));
pub const BG_RED_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(254.0 / 255.0, 226.0 / 255.0, 226.0 / 255.0));
pub const BG_RED_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(254.0 / 255.0, 202.0 / 255.0, 202.0 / 255.0));
pub const BG_RED_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(252.0 / 255.0, 165.0 / 255.0, 165.0 / 255.0));
pub const BG_RED_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(248.0 / 255.0, 113.0 / 255.0, 113.0 / 255.0));
pub const BG_RED_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(239.0 / 255.0, 68.0 / 255.0, 68.0 / 255.0));
pub const BG_RED_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(220.0 / 255.0, 38.0 / 255.0, 38.0 / 255.0));
pub const BG_RED_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(185.0 / 255.0, 28.0 / 255.0, 28.0 / 255.0));
pub const BG_RED_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(153.0 / 255.0, 27.0 / 255.0, 27.0 / 255.0));
pub const BG_RED_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(127.0 / 255.0, 29.0 / 255.0, 29.0 / 255.0));
pub const BG_RED_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(69.0 / 255.0, 10.0 / 255.0, 10.0 / 255.0));
pub const BG_ORANGE_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(255.0 / 255.0, 247.0 / 255.0, 237.0 / 255.0));
pub const BG_ORANGE_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(255.0 / 255.0, 237.0 / 255.0, 213.0 / 255.0));
pub const BG_ORANGE_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(254.0 / 255.0, 215.0 / 255.0, 170.0 / 255.0));
pub const BG_ORANGE_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(253.0 / 255.0, 186.0 / 255.0, 116.0 / 255.0));
pub const BG_ORANGE_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(251.0 / 255.0, 146.0 / 255.0, 60.0 / 255.0));
pub const BG_ORANGE_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(249.0 / 255.0, 115.0 / 255.0, 22.0 / 255.0));
pub const BG_ORANGE_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(234.0 / 255.0, 88.0 / 255.0, 12.0 / 255.0));
pub const BG_ORANGE_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(194.0 / 255.0, 65.0 / 255.0, 12.0 / 255.0));
pub const BG_ORANGE_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(154.0 / 255.0, 52.0 / 255.0, 18.0 / 255.0));
pub const BG_ORANGE_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(124.0 / 255.0, 45.0 / 255.0, 18.0 / 255.0));
pub const BG_ORANGE_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(67.0 / 255.0, 20.0 / 255.0, 7.0 / 255.0));
pub const BG_AMBER_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(255.0 / 255.0, 251.0 / 255.0, 235.0 / 255.0));
pub const BG_AMBER_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(254.0 / 255.0, 243.0 / 255.0, 199.0 / 255.0));
pub const BG_AMBER_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(253.0 / 255.0, 230.0 / 255.0, 138.0 / 255.0));
pub const BG_AMBER_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(252.0 / 255.0, 211.0 / 255.0, 77.0 / 255.0));
pub const BG_AMBER_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(251.0 / 255.0, 191.0 / 255.0, 36.0 / 255.0));
pub const BG_AMBER_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(245.0 / 255.0, 158.0 / 255.0, 11.0 / 255.0));
pub const BG_AMBER_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(217.0 / 255.0, 119.0 / 255.0, 6.0 / 255.0));
pub const BG_AMBER_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(180.0 / 255.0, 83.0 / 255.0, 9.0 / 255.0));
pub const BG_AMBER_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(146.0 / 255.0, 64.0 / 255.0, 14.0 / 255.0));
pub const BG_AMBER_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(120.0 / 255.0, 53.0 / 255.0, 15.0 / 255.0));
pub const BG_AMBER_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(69.0 / 255.0, 26.0 / 255.0, 3.0 / 255.0));
pub const BG_YELLOW_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(254.0 / 255.0, 252.0 / 255.0, 232.0 / 255.0));
pub const BG_YELLOW_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(254.0 / 255.0, 249.0 / 255.0, 195.0 / 255.0));
pub const BG_YELLOW_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(254.0 / 255.0, 240.0 / 255.0, 138.0 / 255.0));
pub const BG_YELLOW_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(253.0 / 255.0, 224.0 / 255.0, 71.0 / 255.0));
pub const BG_YELLOW_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(250.0 / 255.0, 204.0 / 255.0, 21.0 / 255.0));
pub const BG_YELLOW_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(234.0 / 255.0, 179.0 / 255.0, 8.0 / 255.0));
pub const BG_YELLOW_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(202.0 / 255.0, 138.0 / 255.0, 4.0 / 255.0));
pub const BG_YELLOW_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(161.0 / 255.0, 98.0 / 255.0, 7.0 / 255.0));
pub const BG_YELLOW_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(133.0 / 255.0, 77.0 / 255.0, 14.0 / 255.0));
pub const BG_YELLOW_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(113.0 / 255.0, 63.0 / 255.0, 18.0 / 255.0));
pub const BG_YELLOW_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(66.0 / 255.0, 32.0 / 255.0, 6.0 / 255.0));
pub const BG_LIME_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(247.0 / 255.0, 254.0 / 255.0, 231.0 / 255.0));
pub const BG_LIME_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(236.0 / 255.0, 252.0 / 255.0, 203.0 / 255.0));
pub const BG_LIME_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(217.0 / 255.0, 249.0 / 255.0, 157.0 / 255.0));
pub const BG_LIME_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(190.0 / 255.0, 242.0 / 255.0, 100.0 / 255.0));
pub const BG_LIME_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(163.0 / 255.0, 230.0 / 255.0, 53.0 / 255.0));
pub const BG_LIME_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(132.0 / 255.0, 204.0 / 255.0, 22.0 / 255.0));
pub const BG_LIME_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(101.0 / 255.0, 163.0 / 255.0, 13.0 / 255.0));
pub const BG_LIME_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(77.0 / 255.0, 124.0 / 255.0, 15.0 / 255.0));
pub const BG_LIME_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(63.0 / 255.0, 98.0 / 255.0, 18.0 / 255.0));
pub const BG_LIME_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(54.0 / 255.0, 83.0 / 255.0, 20.0 / 255.0));
pub const BG_LIME_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(26.0 / 255.0, 46.0 / 255.0, 5.0 / 255.0));
pub const BG_GREEN_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(240.0 / 255.0, 253.0 / 255.0, 244.0 / 255.0));
pub const BG_GREEN_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(220.0 / 255.0, 252.0 / 255.0, 231.0 / 255.0));
pub const BG_GREEN_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(187.0 / 255.0, 247.0 / 255.0, 208.0 / 255.0));
pub const BG_GREEN_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(134.0 / 255.0, 239.0 / 255.0, 172.0 / 255.0));
pub const BG_GREEN_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(74.0 / 255.0, 222.0 / 255.0, 128.0 / 255.0));
pub const BG_GREEN_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(34.0 / 255.0, 197.0 / 255.0, 94.0 / 255.0));
pub const BG_GREEN_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(22.0 / 255.0, 163.0 / 255.0, 74.0 / 255.0));
pub const BG_GREEN_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(21.0 / 255.0, 128.0 / 255.0, 61.0 / 255.0));
pub const BG_GREEN_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(22.0 / 255.0, 101.0 / 255.0, 52.0 / 255.0));
pub const BG_GREEN_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(20.0 / 255.0, 83.0 / 255.0, 45.0 / 255.0));
pub const BG_GREEN_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(5.0 / 255.0, 46.0 / 255.0, 22.0 / 255.0));
pub const BG_EMERALD_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(236.0 / 255.0, 253.0 / 255.0, 245.0 / 255.0));
pub const BG_EMERALD_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(209.0 / 255.0, 250.0 / 255.0, 229.0 / 255.0));
pub const BG_EMERALD_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(167.0 / 255.0, 243.0 / 255.0, 208.0 / 255.0));
pub const BG_EMERALD_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(110.0 / 255.0, 231.0 / 255.0, 183.0 / 255.0));
pub const BG_EMERALD_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(52.0 / 255.0, 211.0 / 255.0, 153.0 / 255.0));
pub const BG_EMERALD_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(16.0 / 255.0, 185.0 / 255.0, 129.0 / 255.0));
pub const BG_EMERALD_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(5.0 / 255.0, 150.0 / 255.0, 105.0 / 255.0));
pub const BG_EMERALD_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(4.0 / 255.0, 120.0 / 255.0, 87.0 / 255.0));
pub const BG_EMERALD_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(6.0 / 255.0, 95.0 / 255.0, 70.0 / 255.0));
pub const BG_EMERALD_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(6.0 / 255.0, 78.0 / 255.0, 59.0 / 255.0));
pub const BG_EMERALD_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(2.0 / 255.0, 44.0 / 255.0, 34.0 / 255.0));
pub const BG_TEAL_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(240.0 / 255.0, 253.0 / 255.0, 250.0 / 255.0));
pub const BG_TEAL_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(204.0 / 255.0, 251.0 / 255.0, 241.0 / 255.0));
pub const BG_TEAL_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(153.0 / 255.0, 246.0 / 255.0, 228.0 / 255.0));
pub const BG_TEAL_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(94.0 / 255.0, 234.0 / 255.0, 212.0 / 255.0));
pub const BG_TEAL_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(45.0 / 255.0, 212.0 / 255.0, 191.0 / 255.0));
pub const BG_TEAL_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(20.0 / 255.0, 184.0 / 255.0, 166.0 / 255.0));
pub const BG_TEAL_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(13.0 / 255.0, 148.0 / 255.0, 136.0 / 255.0));
pub const BG_TEAL_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(15.0 / 255.0, 118.0 / 255.0, 110.0 / 255.0));
pub const BG_TEAL_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(17.0 / 255.0, 94.0 / 255.0, 89.0 / 255.0));
pub const BG_TEAL_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(19.0 / 255.0, 78.0 / 255.0, 74.0 / 255.0));
pub const BG_TEAL_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(4.0 / 255.0, 47.0 / 255.0, 46.0 / 255.0));
pub const BG_CYAN_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(236.0 / 255.0, 254.0 / 255.0, 255.0 / 255.0));
pub const BG_CYAN_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(207.0 / 255.0, 250.0 / 255.0, 254.0 / 255.0));
pub const BG_CYAN_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(165.0 / 255.0, 243.0 / 255.0, 252.0 / 255.0));
pub const BG_CYAN_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(103.0 / 255.0, 232.0 / 255.0, 249.0 / 255.0));
pub const BG_CYAN_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(34.0 / 255.0, 211.0 / 255.0, 238.0 / 255.0));
pub const BG_CYAN_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(6.0 / 255.0, 182.0 / 255.0, 212.0 / 255.0));
pub const BG_CYAN_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(8.0 / 255.0, 145.0 / 255.0, 178.0 / 255.0));
pub const BG_CYAN_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(14.0 / 255.0, 116.0 / 255.0, 144.0 / 255.0));
pub const BG_CYAN_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(21.0 / 255.0, 94.0 / 255.0, 117.0 / 255.0));
pub const BG_CYAN_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(22.0 / 255.0, 78.0 / 255.0, 99.0 / 255.0));
pub const BG_CYAN_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(8.0 / 255.0, 51.0 / 255.0, 68.0 / 255.0));
pub const BG_SKY_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(240.0 / 255.0, 249.0 / 255.0, 255.0 / 255.0));
pub const BG_SKY_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(224.0 / 255.0, 242.0 / 255.0, 254.0 / 255.0));
pub const BG_SKY_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(186.0 / 255.0, 230.0 / 255.0, 253.0 / 255.0));
pub const BG_SKY_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(125.0 / 255.0, 211.0 / 255.0, 252.0 / 255.0));
pub const BG_SKY_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(56.0 / 255.0, 189.0 / 255.0, 248.0 / 255.0));
pub const BG_SKY_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(14.0 / 255.0, 165.0 / 255.0, 233.0 / 255.0));
pub const BG_SKY_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(2.0 / 255.0, 132.0 / 255.0, 199.0 / 255.0));
pub const BG_SKY_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(3.0 / 255.0, 105.0 / 255.0, 161.0 / 255.0));
pub const BG_SKY_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(7.0 / 255.0, 89.0 / 255.0, 133.0 / 255.0));
pub const BG_SKY_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(12.0 / 255.0, 74.0 / 255.0, 110.0 / 255.0));
pub const BG_SKY_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(8.0 / 255.0, 47.0 / 255.0, 73.0 / 255.0));
pub const BG_BLUE_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(239.0 / 255.0, 246.0 / 255.0, 255.0 / 255.0));
pub const BG_BLUE_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(219.0 / 255.0, 234.0 / 255.0, 254.0 / 255.0));
pub const BG_BLUE_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(191.0 / 255.0, 219.0 / 255.0, 254.0 / 255.0));
pub const BG_BLUE_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(147.0 / 255.0, 197.0 / 255.0, 253.0 / 255.0));
pub const BG_BLUE_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(96.0 / 255.0, 165.0 / 255.0, 250.0 / 255.0));
pub const BG_BLUE_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(59.0 / 255.0, 130.0 / 255.0, 246.0 / 255.0));
pub const BG_BLUE_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(37.0 / 255.0, 99.0 / 255.0, 235.0 / 255.0));
pub const BG_BLUE_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(29.0 / 255.0, 78.0 / 255.0, 216.0 / 255.0));
pub const BG_BLUE_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(30.0 / 255.0, 64.0 / 255.0, 175.0 / 255.0));
pub const BG_BLUE_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(30.0 / 255.0, 58.0 / 255.0, 138.0 / 255.0));
pub const BG_BLUE_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(23.0 / 255.0, 37.0 / 255.0, 84.0 / 255.0));
pub const BG_INDIGO_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(238.0 / 255.0, 242.0 / 255.0, 255.0 / 255.0));
pub const BG_INDIGO_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(224.0 / 255.0, 231.0 / 255.0, 255.0 / 255.0));
pub const BG_INDIGO_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(199.0 / 255.0, 210.0 / 255.0, 254.0 / 255.0));
pub const BG_INDIGO_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(165.0 / 255.0, 180.0 / 255.0, 252.0 / 255.0));
pub const BG_INDIGO_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(129.0 / 255.0, 140.0 / 255.0, 248.0 / 255.0));
pub const BG_INDIGO_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(99.0 / 255.0, 102.0 / 255.0, 241.0 / 255.0));
pub const BG_INDIGO_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(79.0 / 255.0, 70.0 / 255.0, 229.0 / 255.0));
pub const BG_INDIGO_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(67.0 / 255.0, 56.0 / 255.0, 202.0 / 255.0));
pub const BG_INDIGO_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(55.0 / 255.0, 48.0 / 255.0, 163.0 / 255.0));
pub const BG_INDIGO_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(49.0 / 255.0, 46.0 / 255.0, 129.0 / 255.0));
pub const BG_INDIGO_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(30.0 / 255.0, 27.0 / 255.0, 75.0 / 255.0));
pub const BG_VIOLET_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(245.0 / 255.0, 243.0 / 255.0, 255.0 / 255.0));
pub const BG_VIOLET_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(237.0 / 255.0, 233.0 / 255.0, 254.0 / 255.0));
pub const BG_VIOLET_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(221.0 / 255.0, 214.0 / 255.0, 254.0 / 255.0));
pub const BG_VIOLET_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(196.0 / 255.0, 181.0 / 255.0, 253.0 / 255.0));
pub const BG_VIOLET_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(167.0 / 255.0, 139.0 / 255.0, 250.0 / 255.0));
pub const BG_VIOLET_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(139.0 / 255.0, 92.0 / 255.0, 246.0 / 255.0));
pub const BG_VIOLET_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(124.0 / 255.0, 58.0 / 255.0, 237.0 / 255.0));
pub const BG_VIOLET_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(109.0 / 255.0, 40.0 / 255.0, 217.0 / 255.0));
pub const BG_VIOLET_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(91.0 / 255.0, 33.0 / 255.0, 182.0 / 255.0));
pub const BG_VIOLET_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(76.0 / 255.0, 29.0 / 255.0, 149.0 / 255.0));
pub const BG_VIOLET_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(46.0 / 255.0, 16.0 / 255.0, 101.0 / 255.0));
pub const BG_PURPLE_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(250.0 / 255.0, 245.0 / 255.0, 255.0 / 255.0));
pub const BG_PURPLE_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(243.0 / 255.0, 232.0 / 255.0, 255.0 / 255.0));
pub const BG_PURPLE_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(233.0 / 255.0, 213.0 / 255.0, 255.0 / 255.0));
pub const BG_PURPLE_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(216.0 / 255.0, 180.0 / 255.0, 254.0 / 255.0));
pub const BG_PURPLE_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(192.0 / 255.0, 132.0 / 255.0, 252.0 / 255.0));
pub const BG_PURPLE_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(168.0 / 255.0, 85.0 / 255.0, 247.0 / 255.0));
pub const BG_PURPLE_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(147.0 / 255.0, 51.0 / 255.0, 234.0 / 255.0));
pub const BG_PURPLE_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(126.0 / 255.0, 34.0 / 255.0, 206.0 / 255.0));
pub const BG_PURPLE_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(107.0 / 255.0, 33.0 / 255.0, 168.0 / 255.0));
pub const BG_PURPLE_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(88.0 / 255.0, 28.0 / 255.0, 135.0 / 255.0));
pub const BG_PURPLE_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(59.0 / 255.0, 7.0 / 255.0, 100.0 / 255.0));
pub const BG_FUCHSIA_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(253.0 / 255.0, 244.0 / 255.0, 255.0 / 255.0));
pub const BG_FUCHSIA_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(250.0 / 255.0, 232.0 / 255.0, 255.0 / 255.0));
pub const BG_FUCHSIA_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(245.0 / 255.0, 208.0 / 255.0, 254.0 / 255.0));
pub const BG_FUCHSIA_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(240.0 / 255.0, 171.0 / 255.0, 252.0 / 255.0));
pub const BG_FUCHSIA_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(232.0 / 255.0, 121.0 / 255.0, 249.0 / 255.0));
pub const BG_FUCHSIA_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(217.0 / 255.0, 70.0 / 255.0, 239.0 / 255.0));
pub const BG_FUCHSIA_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(192.0 / 255.0, 38.0 / 255.0, 211.0 / 255.0));
pub const BG_FUCHSIA_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(162.0 / 255.0, 28.0 / 255.0, 175.0 / 255.0));
pub const BG_FUCHSIA_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(134.0 / 255.0, 25.0 / 255.0, 143.0 / 255.0));
pub const BG_FUCHSIA_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(112.0 / 255.0, 26.0 / 255.0, 117.0 / 255.0));
pub const BG_FUCHSIA_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(74.0 / 255.0, 4.0 / 255.0, 78.0 / 255.0));
pub const BG_PINK_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(253.0 / 255.0, 242.0 / 255.0, 248.0 / 255.0));
pub const BG_PINK_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(252.0 / 255.0, 231.0 / 255.0, 243.0 / 255.0));
pub const BG_PINK_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(251.0 / 255.0, 207.0 / 255.0, 232.0 / 255.0));
pub const BG_PINK_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(249.0 / 255.0, 168.0 / 255.0, 212.0 / 255.0));
pub const BG_PINK_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(244.0 / 255.0, 114.0 / 255.0, 182.0 / 255.0));
pub const BG_PINK_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(236.0 / 255.0, 72.0 / 255.0, 153.0 / 255.0));
pub const BG_PINK_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(219.0 / 255.0, 39.0 / 255.0, 119.0 / 255.0));
pub const BG_PINK_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(190.0 / 255.0, 24.0 / 255.0, 93.0 / 255.0));
pub const BG_PINK_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(157.0 / 255.0, 23.0 / 255.0, 77.0 / 255.0));
pub const BG_PINK_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(131.0 / 255.0, 24.0 / 255.0, 67.0 / 255.0));
pub const BG_PINK_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(80.0 / 255.0, 7.0 / 255.0, 36.0 / 255.0));
pub const BG_ROSE_50: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(255.0 / 255.0, 241.0 / 255.0, 242.0 / 255.0));
pub const BG_ROSE_100: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(255.0 / 255.0, 228.0 / 255.0, 230.0 / 255.0));
pub const BG_ROSE_200: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(254.0 / 255.0, 205.0 / 255.0, 211.0 / 255.0));
pub const BG_ROSE_300: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(253.0 / 255.0, 164.0 / 255.0, 175.0 / 255.0));
pub const BG_ROSE_400: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(251.0 / 255.0, 113.0 / 255.0, 133.0 / 255.0));
pub const BG_ROSE_500: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(244.0 / 255.0, 63.0 / 255.0, 94.0 / 255.0));
pub const BG_ROSE_600: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(225.0 / 255.0, 29.0 / 255.0, 72.0 / 255.0));
pub const BG_ROSE_700: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(190.0 / 255.0, 18.0 / 255.0, 60.0 / 255.0));
pub const BG_ROSE_800: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(159.0 / 255.0, 18.0 / 255.0, 57.0 / 255.0));
pub const BG_ROSE_900: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(136.0 / 255.0, 19.0 / 255.0, 55.0 / 255.0));
pub const BG_ROSE_950: BackgroundColorClass =
    BackgroundColorClass(Color::rgb(76.0 / 255.0, 5.0 / 255.0, 25.0 / 255.0));

#[derive(Debug, Clone, Copy)]
pub struct BackgroundColorClass(Color);

impl Default for BackgroundColorClass {
    fn default() -> Self {
        Self(Color::WHITE)
    }
}

impl Div<f32> for BackgroundColorClass {
    type Output = BackgroundColorClass;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0.with_a(rhs))
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

impl ApplyClass for BackgroundColorClass {
    type Component = BackgroundColor;

    fn apply_class(&self, component: &mut Self::Component) {
        component.0 = self.0;
    }
}
