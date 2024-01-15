use crate::class::ApplyClass;
use bevy::{
    prelude::Color,
    ui::{BackgroundColor, Interaction},
};
use std::ops::Div;

pub const BG_TRANSPARENT: BgColor = BgColor(Color::NONE);
pub const BG_BLACK: BgColor = BgColor(Color::BLACK);
pub const BG_WHITE: BgColor = BgColor(Color::WHITE);
pub const BG_SLATE_50: BgColor = BgColor(Color::rgb(248.0 / 255.0, 250.0 / 255.0, 252.0 / 255.0));
pub const BG_SLATE_100: BgColor = BgColor(Color::rgb(241.0 / 255.0, 245.0 / 255.0, 249.0 / 255.0));
pub const BG_SLATE_200: BgColor = BgColor(Color::rgb(226.0 / 255.0, 232.0 / 255.0, 240.0 / 255.0));
pub const BG_SLATE_300: BgColor = BgColor(Color::rgb(203.0 / 255.0, 213.0 / 255.0, 225.0 / 255.0));
pub const BG_SLATE_400: BgColor = BgColor(Color::rgb(148.0 / 255.0, 163.0 / 255.0, 184.0 / 255.0));
pub const BG_SLATE_500: BgColor = BgColor(Color::rgb(100.0 / 255.0, 116.0 / 255.0, 139.0 / 255.0));
pub const BG_SLATE_600: BgColor = BgColor(Color::rgb(71.0 / 255.0, 85.0 / 255.0, 105.0 / 255.0));
pub const BG_SLATE_700: BgColor = BgColor(Color::rgb(51.0 / 255.0, 65.0 / 255.0, 85.0 / 255.0));
pub const BG_SLATE_800: BgColor = BgColor(Color::rgb(30.0 / 255.0, 41.0 / 255.0, 59.0 / 255.0));
pub const BG_SLATE_900: BgColor = BgColor(Color::rgb(15.0 / 255.0, 23.0 / 255.0, 42.0 / 255.0));
pub const BG_SLATE_950: BgColor = BgColor(Color::rgb(2.0 / 255.0, 6.0 / 255.0, 23.0 / 255.0));
pub const BG_GRAY_50: BgColor = BgColor(Color::rgb(249.0 / 255.0, 250.0 / 255.0, 251.0 / 255.0));
pub const BG_GRAY_100: BgColor = BgColor(Color::rgb(243.0 / 255.0, 244.0 / 255.0, 246.0 / 255.0));
pub const BG_GRAY_200: BgColor = BgColor(Color::rgb(229.0 / 255.0, 231.0 / 255.0, 235.0 / 255.0));
pub const BG_GRAY_300: BgColor = BgColor(Color::rgb(209.0 / 255.0, 213.0 / 255.0, 219.0 / 255.0));
pub const BG_GRAY_400: BgColor = BgColor(Color::rgb(156.0 / 255.0, 163.0 / 255.0, 175.0 / 255.0));
pub const BG_GRAY_500: BgColor = BgColor(Color::rgb(107.0 / 255.0, 114.0 / 255.0, 128.0 / 255.0));
pub const BG_GRAY_600: BgColor = BgColor(Color::rgb(75.0 / 255.0, 85.0 / 255.0, 99.0 / 255.0));
pub const BG_GRAY_700: BgColor = BgColor(Color::rgb(55.0 / 255.0, 65.0 / 255.0, 81.0 / 255.0));
pub const BG_GRAY_800: BgColor = BgColor(Color::rgb(31.0 / 255.0, 41.0 / 255.0, 55.0 / 255.0));
pub const BG_GRAY_900: BgColor = BgColor(Color::rgb(17.0 / 255.0, 24.0 / 255.0, 39.0 / 255.0));
pub const BG_GRAY_950: BgColor = BgColor(Color::rgb(3.0 / 255.0, 7.0 / 255.0, 18.0 / 255.0));
pub const BG_ZINC_50: BgColor = BgColor(Color::rgb(250.0 / 255.0, 250.0 / 255.0, 250.0 / 255.0));
pub const BG_ZINC_100: BgColor = BgColor(Color::rgb(244.0 / 255.0, 244.0 / 255.0, 245.0 / 255.0));
pub const BG_ZINC_200: BgColor = BgColor(Color::rgb(228.0 / 255.0, 228.0 / 255.0, 231.0 / 255.0));
pub const BG_ZINC_300: BgColor = BgColor(Color::rgb(212.0 / 255.0, 212.0 / 255.0, 216.0 / 255.0));
pub const BG_ZINC_400: BgColor = BgColor(Color::rgb(161.0 / 255.0, 161.0 / 255.0, 170.0 / 255.0));
pub const BG_ZINC_500: BgColor = BgColor(Color::rgb(113.0 / 255.0, 113.0 / 255.0, 122.0 / 255.0));
pub const BG_ZINC_600: BgColor = BgColor(Color::rgb(82.0 / 255.0, 82.0 / 255.0, 91.0 / 255.0));
pub const BG_ZINC_700: BgColor = BgColor(Color::rgb(63.0 / 255.0, 63.0 / 255.0, 70.0 / 255.0));
pub const BG_ZINC_800: BgColor = BgColor(Color::rgb(39.0 / 255.0, 39.0 / 255.0, 42.0 / 255.0));
pub const BG_ZINC_900: BgColor = BgColor(Color::rgb(24.0 / 255.0, 24.0 / 255.0, 27.0 / 255.0));
pub const BG_ZINC_950: BgColor = BgColor(Color::rgb(9.0 / 255.0, 9.0 / 255.0, 11.0 / 255.0));
pub const BG_NEUTRAL_50: BgColor = BgColor(Color::rgb(250.0 / 255.0, 250.0 / 255.0, 250.0 / 255.0));
pub const BG_NEUTRAL_100: BgColor =
    BgColor(Color::rgb(245.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0));
pub const BG_NEUTRAL_200: BgColor =
    BgColor(Color::rgb(229.0 / 255.0, 229.0 / 255.0, 229.0 / 255.0));
pub const BG_NEUTRAL_300: BgColor =
    BgColor(Color::rgb(212.0 / 255.0, 212.0 / 255.0, 212.0 / 255.0));
pub const BG_NEUTRAL_400: BgColor =
    BgColor(Color::rgb(163.0 / 255.0, 163.0 / 255.0, 163.0 / 255.0));
pub const BG_NEUTRAL_500: BgColor =
    BgColor(Color::rgb(115.0 / 255.0, 115.0 / 255.0, 115.0 / 255.0));
pub const BG_NEUTRAL_600: BgColor = BgColor(Color::rgb(82.0 / 255.0, 82.0 / 255.0, 82.0 / 255.0));
pub const BG_NEUTRAL_700: BgColor = BgColor(Color::rgb(64.0 / 255.0, 64.0 / 255.0, 64.0 / 255.0));
pub const BG_NEUTRAL_800: BgColor = BgColor(Color::rgb(38.0 / 255.0, 38.0 / 255.0, 38.0 / 255.0));
pub const BG_NEUTRAL_900: BgColor = BgColor(Color::rgb(23.0 / 255.0, 23.0 / 255.0, 23.0 / 255.0));
pub const BG_NEUTRAL_950: BgColor = BgColor(Color::rgb(10.0 / 255.0, 10.0 / 255.0, 10.0 / 255.0));
pub const BG_STONE_50: BgColor = BgColor(Color::rgb(250.0 / 255.0, 250.0 / 255.0, 249.0 / 255.0));
pub const BG_STONE_100: BgColor = BgColor(Color::rgb(245.0 / 255.0, 245.0 / 255.0, 244.0 / 255.0));
pub const BG_STONE_200: BgColor = BgColor(Color::rgb(231.0 / 255.0, 229.0 / 255.0, 228.0 / 255.0));
pub const BG_STONE_300: BgColor = BgColor(Color::rgb(214.0 / 255.0, 211.0 / 255.0, 209.0 / 255.0));
pub const BG_STONE_400: BgColor = BgColor(Color::rgb(168.0 / 255.0, 162.0 / 255.0, 158.0 / 255.0));
pub const BG_STONE_500: BgColor = BgColor(Color::rgb(120.0 / 255.0, 113.0 / 255.0, 108.0 / 255.0));
pub const BG_STONE_600: BgColor = BgColor(Color::rgb(87.0 / 255.0, 83.0 / 255.0, 78.0 / 255.0));
pub const BG_STONE_700: BgColor = BgColor(Color::rgb(68.0 / 255.0, 64.0 / 255.0, 60.0 / 255.0));
pub const BG_STONE_800: BgColor = BgColor(Color::rgb(41.0 / 255.0, 37.0 / 255.0, 36.0 / 255.0));
pub const BG_STONE_900: BgColor = BgColor(Color::rgb(28.0 / 255.0, 25.0 / 255.0, 23.0 / 255.0));
pub const BG_STONE_950: BgColor = BgColor(Color::rgb(12.0 / 255.0, 10.0 / 255.0, 9.0 / 255.0));
pub const BG_RED_50: BgColor = BgColor(Color::rgb(254.0 / 255.0, 242.0 / 255.0, 242.0 / 255.0));
pub const BG_RED_100: BgColor = BgColor(Color::rgb(254.0 / 255.0, 226.0 / 255.0, 226.0 / 255.0));
pub const BG_RED_200: BgColor = BgColor(Color::rgb(254.0 / 255.0, 202.0 / 255.0, 202.0 / 255.0));
pub const BG_RED_300: BgColor = BgColor(Color::rgb(252.0 / 255.0, 165.0 / 255.0, 165.0 / 255.0));
pub const BG_RED_400: BgColor = BgColor(Color::rgb(248.0 / 255.0, 113.0 / 255.0, 113.0 / 255.0));
pub const BG_RED_500: BgColor = BgColor(Color::rgb(239.0 / 255.0, 68.0 / 255.0, 68.0 / 255.0));
pub const BG_RED_600: BgColor = BgColor(Color::rgb(220.0 / 255.0, 38.0 / 255.0, 38.0 / 255.0));
pub const BG_RED_700: BgColor = BgColor(Color::rgb(185.0 / 255.0, 28.0 / 255.0, 28.0 / 255.0));
pub const BG_RED_800: BgColor = BgColor(Color::rgb(153.0 / 255.0, 27.0 / 255.0, 27.0 / 255.0));
pub const BG_RED_900: BgColor = BgColor(Color::rgb(127.0 / 255.0, 29.0 / 255.0, 29.0 / 255.0));
pub const BG_RED_950: BgColor = BgColor(Color::rgb(69.0 / 255.0, 10.0 / 255.0, 10.0 / 255.0));
pub const BG_ORANGE_50: BgColor = BgColor(Color::rgb(255.0 / 255.0, 247.0 / 255.0, 237.0 / 255.0));
pub const BG_ORANGE_100: BgColor = BgColor(Color::rgb(255.0 / 255.0, 237.0 / 255.0, 213.0 / 255.0));
pub const BG_ORANGE_200: BgColor = BgColor(Color::rgb(254.0 / 255.0, 215.0 / 255.0, 170.0 / 255.0));
pub const BG_ORANGE_300: BgColor = BgColor(Color::rgb(253.0 / 255.0, 186.0 / 255.0, 116.0 / 255.0));
pub const BG_ORANGE_400: BgColor = BgColor(Color::rgb(251.0 / 255.0, 146.0 / 255.0, 60.0 / 255.0));
pub const BG_ORANGE_500: BgColor = BgColor(Color::rgb(249.0 / 255.0, 115.0 / 255.0, 22.0 / 255.0));
pub const BG_ORANGE_600: BgColor = BgColor(Color::rgb(234.0 / 255.0, 88.0 / 255.0, 12.0 / 255.0));
pub const BG_ORANGE_700: BgColor = BgColor(Color::rgb(194.0 / 255.0, 65.0 / 255.0, 12.0 / 255.0));
pub const BG_ORANGE_800: BgColor = BgColor(Color::rgb(154.0 / 255.0, 52.0 / 255.0, 18.0 / 255.0));
pub const BG_ORANGE_900: BgColor = BgColor(Color::rgb(124.0 / 255.0, 45.0 / 255.0, 18.0 / 255.0));
pub const BG_ORANGE_950: BgColor = BgColor(Color::rgb(67.0 / 255.0, 20.0 / 255.0, 7.0 / 255.0));
pub const BG_AMBER_50: BgColor = BgColor(Color::rgb(255.0 / 255.0, 251.0 / 255.0, 235.0 / 255.0));
pub const BG_AMBER_100: BgColor = BgColor(Color::rgb(254.0 / 255.0, 243.0 / 255.0, 199.0 / 255.0));
pub const BG_AMBER_200: BgColor = BgColor(Color::rgb(253.0 / 255.0, 230.0 / 255.0, 138.0 / 255.0));
pub const BG_AMBER_300: BgColor = BgColor(Color::rgb(252.0 / 255.0, 211.0 / 255.0, 77.0 / 255.0));
pub const BG_AMBER_400: BgColor = BgColor(Color::rgb(251.0 / 255.0, 191.0 / 255.0, 36.0 / 255.0));
pub const BG_AMBER_500: BgColor = BgColor(Color::rgb(245.0 / 255.0, 158.0 / 255.0, 11.0 / 255.0));
pub const BG_AMBER_600: BgColor = BgColor(Color::rgb(217.0 / 255.0, 119.0 / 255.0, 6.0 / 255.0));
pub const BG_AMBER_700: BgColor = BgColor(Color::rgb(180.0 / 255.0, 83.0 / 255.0, 9.0 / 255.0));
pub const BG_AMBER_800: BgColor = BgColor(Color::rgb(146.0 / 255.0, 64.0 / 255.0, 14.0 / 255.0));
pub const BG_AMBER_900: BgColor = BgColor(Color::rgb(120.0 / 255.0, 53.0 / 255.0, 15.0 / 255.0));
pub const BG_AMBER_950: BgColor = BgColor(Color::rgb(69.0 / 255.0, 26.0 / 255.0, 3.0 / 255.0));
pub const BG_YELLOW_50: BgColor = BgColor(Color::rgb(254.0 / 255.0, 252.0 / 255.0, 232.0 / 255.0));
pub const BG_YELLOW_100: BgColor = BgColor(Color::rgb(254.0 / 255.0, 249.0 / 255.0, 195.0 / 255.0));
pub const BG_YELLOW_200: BgColor = BgColor(Color::rgb(254.0 / 255.0, 240.0 / 255.0, 138.0 / 255.0));
pub const BG_YELLOW_300: BgColor = BgColor(Color::rgb(253.0 / 255.0, 224.0 / 255.0, 71.0 / 255.0));
pub const BG_YELLOW_400: BgColor = BgColor(Color::rgb(250.0 / 255.0, 204.0 / 255.0, 21.0 / 255.0));
pub const BG_YELLOW_500: BgColor = BgColor(Color::rgb(234.0 / 255.0, 179.0 / 255.0, 8.0 / 255.0));
pub const BG_YELLOW_600: BgColor = BgColor(Color::rgb(202.0 / 255.0, 138.0 / 255.0, 4.0 / 255.0));
pub const BG_YELLOW_700: BgColor = BgColor(Color::rgb(161.0 / 255.0, 98.0 / 255.0, 7.0 / 255.0));
pub const BG_YELLOW_800: BgColor = BgColor(Color::rgb(133.0 / 255.0, 77.0 / 255.0, 14.0 / 255.0));
pub const BG_YELLOW_900: BgColor = BgColor(Color::rgb(113.0 / 255.0, 63.0 / 255.0, 18.0 / 255.0));
pub const BG_YELLOW_950: BgColor = BgColor(Color::rgb(66.0 / 255.0, 32.0 / 255.0, 6.0 / 255.0));
pub const BG_LIME_50: BgColor = BgColor(Color::rgb(247.0 / 255.0, 254.0 / 255.0, 231.0 / 255.0));
pub const BG_LIME_100: BgColor = BgColor(Color::rgb(236.0 / 255.0, 252.0 / 255.0, 203.0 / 255.0));
pub const BG_LIME_200: BgColor = BgColor(Color::rgb(217.0 / 255.0, 249.0 / 255.0, 157.0 / 255.0));
pub const BG_LIME_300: BgColor = BgColor(Color::rgb(190.0 / 255.0, 242.0 / 255.0, 100.0 / 255.0));
pub const BG_LIME_400: BgColor = BgColor(Color::rgb(163.0 / 255.0, 230.0 / 255.0, 53.0 / 255.0));
pub const BG_LIME_500: BgColor = BgColor(Color::rgb(132.0 / 255.0, 204.0 / 255.0, 22.0 / 255.0));
pub const BG_LIME_600: BgColor = BgColor(Color::rgb(101.0 / 255.0, 163.0 / 255.0, 13.0 / 255.0));
pub const BG_LIME_700: BgColor = BgColor(Color::rgb(77.0 / 255.0, 124.0 / 255.0, 15.0 / 255.0));
pub const BG_LIME_800: BgColor = BgColor(Color::rgb(63.0 / 255.0, 98.0 / 255.0, 18.0 / 255.0));
pub const BG_LIME_900: BgColor = BgColor(Color::rgb(54.0 / 255.0, 83.0 / 255.0, 20.0 / 255.0));
pub const BG_LIME_950: BgColor = BgColor(Color::rgb(26.0 / 255.0, 46.0 / 255.0, 5.0 / 255.0));
pub const BG_GREEN_50: BgColor = BgColor(Color::rgb(240.0 / 255.0, 253.0 / 255.0, 244.0 / 255.0));
pub const BG_GREEN_100: BgColor = BgColor(Color::rgb(220.0 / 255.0, 252.0 / 255.0, 231.0 / 255.0));
pub const BG_GREEN_200: BgColor = BgColor(Color::rgb(187.0 / 255.0, 247.0 / 255.0, 208.0 / 255.0));
pub const BG_GREEN_300: BgColor = BgColor(Color::rgb(134.0 / 255.0, 239.0 / 255.0, 172.0 / 255.0));
pub const BG_GREEN_400: BgColor = BgColor(Color::rgb(74.0 / 255.0, 222.0 / 255.0, 128.0 / 255.0));
pub const BG_GREEN_500: BgColor = BgColor(Color::rgb(34.0 / 255.0, 197.0 / 255.0, 94.0 / 255.0));
pub const BG_GREEN_600: BgColor = BgColor(Color::rgb(22.0 / 255.0, 163.0 / 255.0, 74.0 / 255.0));
pub const BG_GREEN_700: BgColor = BgColor(Color::rgb(21.0 / 255.0, 128.0 / 255.0, 61.0 / 255.0));
pub const BG_GREEN_800: BgColor = BgColor(Color::rgb(22.0 / 255.0, 101.0 / 255.0, 52.0 / 255.0));
pub const BG_GREEN_900: BgColor = BgColor(Color::rgb(20.0 / 255.0, 83.0 / 255.0, 45.0 / 255.0));
pub const BG_GREEN_950: BgColor = BgColor(Color::rgb(5.0 / 255.0, 46.0 / 255.0, 22.0 / 255.0));
pub const BG_EMERALD_50: BgColor = BgColor(Color::rgb(236.0 / 255.0, 253.0 / 255.0, 245.0 / 255.0));
pub const BG_EMERALD_100: BgColor =
    BgColor(Color::rgb(209.0 / 255.0, 250.0 / 255.0, 229.0 / 255.0));
pub const BG_EMERALD_200: BgColor =
    BgColor(Color::rgb(167.0 / 255.0, 243.0 / 255.0, 208.0 / 255.0));
pub const BG_EMERALD_300: BgColor =
    BgColor(Color::rgb(110.0 / 255.0, 231.0 / 255.0, 183.0 / 255.0));
pub const BG_EMERALD_400: BgColor = BgColor(Color::rgb(52.0 / 255.0, 211.0 / 255.0, 153.0 / 255.0));
pub const BG_EMERALD_500: BgColor = BgColor(Color::rgb(16.0 / 255.0, 185.0 / 255.0, 129.0 / 255.0));
pub const BG_EMERALD_600: BgColor = BgColor(Color::rgb(5.0 / 255.0, 150.0 / 255.0, 105.0 / 255.0));
pub const BG_EMERALD_700: BgColor = BgColor(Color::rgb(4.0 / 255.0, 120.0 / 255.0, 87.0 / 255.0));
pub const BG_EMERALD_800: BgColor = BgColor(Color::rgb(6.0 / 255.0, 95.0 / 255.0, 70.0 / 255.0));
pub const BG_EMERALD_900: BgColor = BgColor(Color::rgb(6.0 / 255.0, 78.0 / 255.0, 59.0 / 255.0));
pub const BG_EMERALD_950: BgColor = BgColor(Color::rgb(2.0 / 255.0, 44.0 / 255.0, 34.0 / 255.0));
pub const BG_TEAL_50: BgColor = BgColor(Color::rgb(240.0 / 255.0, 253.0 / 255.0, 250.0 / 255.0));
pub const BG_TEAL_100: BgColor = BgColor(Color::rgb(204.0 / 255.0, 251.0 / 255.0, 241.0 / 255.0));
pub const BG_TEAL_200: BgColor = BgColor(Color::rgb(153.0 / 255.0, 246.0 / 255.0, 228.0 / 255.0));
pub const BG_TEAL_300: BgColor = BgColor(Color::rgb(94.0 / 255.0, 234.0 / 255.0, 212.0 / 255.0));
pub const BG_TEAL_400: BgColor = BgColor(Color::rgb(45.0 / 255.0, 212.0 / 255.0, 191.0 / 255.0));
pub const BG_TEAL_500: BgColor = BgColor(Color::rgb(20.0 / 255.0, 184.0 / 255.0, 166.0 / 255.0));
pub const BG_TEAL_600: BgColor = BgColor(Color::rgb(13.0 / 255.0, 148.0 / 255.0, 136.0 / 255.0));
pub const BG_TEAL_700: BgColor = BgColor(Color::rgb(15.0 / 255.0, 118.0 / 255.0, 110.0 / 255.0));
pub const BG_TEAL_800: BgColor = BgColor(Color::rgb(17.0 / 255.0, 94.0 / 255.0, 89.0 / 255.0));
pub const BG_TEAL_900: BgColor = BgColor(Color::rgb(19.0 / 255.0, 78.0 / 255.0, 74.0 / 255.0));
pub const BG_TEAL_950: BgColor = BgColor(Color::rgb(4.0 / 255.0, 47.0 / 255.0, 46.0 / 255.0));
pub const BG_CYAN_50: BgColor = BgColor(Color::rgb(236.0 / 255.0, 254.0 / 255.0, 255.0 / 255.0));
pub const BG_CYAN_100: BgColor = BgColor(Color::rgb(207.0 / 255.0, 250.0 / 255.0, 254.0 / 255.0));
pub const BG_CYAN_200: BgColor = BgColor(Color::rgb(165.0 / 255.0, 243.0 / 255.0, 252.0 / 255.0));
pub const BG_CYAN_300: BgColor = BgColor(Color::rgb(103.0 / 255.0, 232.0 / 255.0, 249.0 / 255.0));
pub const BG_CYAN_400: BgColor = BgColor(Color::rgb(34.0 / 255.0, 211.0 / 255.0, 238.0 / 255.0));
pub const BG_CYAN_500: BgColor = BgColor(Color::rgb(6.0 / 255.0, 182.0 / 255.0, 212.0 / 255.0));
pub const BG_CYAN_600: BgColor = BgColor(Color::rgb(8.0 / 255.0, 145.0 / 255.0, 178.0 / 255.0));
pub const BG_CYAN_700: BgColor = BgColor(Color::rgb(14.0 / 255.0, 116.0 / 255.0, 144.0 / 255.0));
pub const BG_CYAN_800: BgColor = BgColor(Color::rgb(21.0 / 255.0, 94.0 / 255.0, 117.0 / 255.0));
pub const BG_CYAN_900: BgColor = BgColor(Color::rgb(22.0 / 255.0, 78.0 / 255.0, 99.0 / 255.0));
pub const BG_CYAN_950: BgColor = BgColor(Color::rgb(8.0 / 255.0, 51.0 / 255.0, 68.0 / 255.0));
pub const BG_SKY_50: BgColor = BgColor(Color::rgb(240.0 / 255.0, 249.0 / 255.0, 255.0 / 255.0));
pub const BG_SKY_100: BgColor = BgColor(Color::rgb(224.0 / 255.0, 242.0 / 255.0, 254.0 / 255.0));
pub const BG_SKY_200: BgColor = BgColor(Color::rgb(186.0 / 255.0, 230.0 / 255.0, 253.0 / 255.0));
pub const BG_SKY_300: BgColor = BgColor(Color::rgb(125.0 / 255.0, 211.0 / 255.0, 252.0 / 255.0));
pub const BG_SKY_400: BgColor = BgColor(Color::rgb(56.0 / 255.0, 189.0 / 255.0, 248.0 / 255.0));
pub const BG_SKY_500: BgColor = BgColor(Color::rgb(14.0 / 255.0, 165.0 / 255.0, 233.0 / 255.0));
pub const BG_SKY_600: BgColor = BgColor(Color::rgb(2.0 / 255.0, 132.0 / 255.0, 199.0 / 255.0));
pub const BG_SKY_700: BgColor = BgColor(Color::rgb(3.0 / 255.0, 105.0 / 255.0, 161.0 / 255.0));
pub const BG_SKY_800: BgColor = BgColor(Color::rgb(7.0 / 255.0, 89.0 / 255.0, 133.0 / 255.0));
pub const BG_SKY_900: BgColor = BgColor(Color::rgb(12.0 / 255.0, 74.0 / 255.0, 110.0 / 255.0));
pub const BG_SKY_950: BgColor = BgColor(Color::rgb(8.0 / 255.0, 47.0 / 255.0, 73.0 / 255.0));
pub const BG_BLUE_50: BgColor = BgColor(Color::rgb(239.0 / 255.0, 246.0 / 255.0, 255.0 / 255.0));
pub const BG_BLUE_100: BgColor = BgColor(Color::rgb(219.0 / 255.0, 234.0 / 255.0, 254.0 / 255.0));
pub const BG_BLUE_200: BgColor = BgColor(Color::rgb(191.0 / 255.0, 219.0 / 255.0, 254.0 / 255.0));
pub const BG_BLUE_300: BgColor = BgColor(Color::rgb(147.0 / 255.0, 197.0 / 255.0, 253.0 / 255.0));
pub const BG_BLUE_400: BgColor = BgColor(Color::rgb(96.0 / 255.0, 165.0 / 255.0, 250.0 / 255.0));
pub const BG_BLUE_500: BgColor = BgColor(Color::rgb(59.0 / 255.0, 130.0 / 255.0, 246.0 / 255.0));
pub const BG_BLUE_600: BgColor = BgColor(Color::rgb(37.0 / 255.0, 99.0 / 255.0, 235.0 / 255.0));
pub const BG_BLUE_700: BgColor = BgColor(Color::rgb(29.0 / 255.0, 78.0 / 255.0, 216.0 / 255.0));
pub const BG_BLUE_800: BgColor = BgColor(Color::rgb(30.0 / 255.0, 64.0 / 255.0, 175.0 / 255.0));
pub const BG_BLUE_900: BgColor = BgColor(Color::rgb(30.0 / 255.0, 58.0 / 255.0, 138.0 / 255.0));
pub const BG_BLUE_950: BgColor = BgColor(Color::rgb(23.0 / 255.0, 37.0 / 255.0, 84.0 / 255.0));
pub const BG_INDIGO_50: BgColor = BgColor(Color::rgb(238.0 / 255.0, 242.0 / 255.0, 255.0 / 255.0));
pub const BG_INDIGO_100: BgColor = BgColor(Color::rgb(224.0 / 255.0, 231.0 / 255.0, 255.0 / 255.0));
pub const BG_INDIGO_200: BgColor = BgColor(Color::rgb(199.0 / 255.0, 210.0 / 255.0, 254.0 / 255.0));
pub const BG_INDIGO_300: BgColor = BgColor(Color::rgb(165.0 / 255.0, 180.0 / 255.0, 252.0 / 255.0));
pub const BG_INDIGO_400: BgColor = BgColor(Color::rgb(129.0 / 255.0, 140.0 / 255.0, 248.0 / 255.0));
pub const BG_INDIGO_500: BgColor = BgColor(Color::rgb(99.0 / 255.0, 102.0 / 255.0, 241.0 / 255.0));
pub const BG_INDIGO_600: BgColor = BgColor(Color::rgb(79.0 / 255.0, 70.0 / 255.0, 229.0 / 255.0));
pub const BG_INDIGO_700: BgColor = BgColor(Color::rgb(67.0 / 255.0, 56.0 / 255.0, 202.0 / 255.0));
pub const BG_INDIGO_800: BgColor = BgColor(Color::rgb(55.0 / 255.0, 48.0 / 255.0, 163.0 / 255.0));
pub const BG_INDIGO_900: BgColor = BgColor(Color::rgb(49.0 / 255.0, 46.0 / 255.0, 129.0 / 255.0));
pub const BG_INDIGO_950: BgColor = BgColor(Color::rgb(30.0 / 255.0, 27.0 / 255.0, 75.0 / 255.0));
pub const BG_VIOLET_50: BgColor = BgColor(Color::rgb(245.0 / 255.0, 243.0 / 255.0, 255.0 / 255.0));
pub const BG_VIOLET_100: BgColor = BgColor(Color::rgb(237.0 / 255.0, 233.0 / 255.0, 254.0 / 255.0));
pub const BG_VIOLET_200: BgColor = BgColor(Color::rgb(221.0 / 255.0, 214.0 / 255.0, 254.0 / 255.0));
pub const BG_VIOLET_300: BgColor = BgColor(Color::rgb(196.0 / 255.0, 181.0 / 255.0, 253.0 / 255.0));
pub const BG_VIOLET_400: BgColor = BgColor(Color::rgb(167.0 / 255.0, 139.0 / 255.0, 250.0 / 255.0));
pub const BG_VIOLET_500: BgColor = BgColor(Color::rgb(139.0 / 255.0, 92.0 / 255.0, 246.0 / 255.0));
pub const BG_VIOLET_600: BgColor = BgColor(Color::rgb(124.0 / 255.0, 58.0 / 255.0, 237.0 / 255.0));
pub const BG_VIOLET_700: BgColor = BgColor(Color::rgb(109.0 / 255.0, 40.0 / 255.0, 217.0 / 255.0));
pub const BG_VIOLET_800: BgColor = BgColor(Color::rgb(91.0 / 255.0, 33.0 / 255.0, 182.0 / 255.0));
pub const BG_VIOLET_900: BgColor = BgColor(Color::rgb(76.0 / 255.0, 29.0 / 255.0, 149.0 / 255.0));
pub const BG_VIOLET_950: BgColor = BgColor(Color::rgb(46.0 / 255.0, 16.0 / 255.0, 101.0 / 255.0));
pub const BG_PURPLE_50: BgColor = BgColor(Color::rgb(250.0 / 255.0, 245.0 / 255.0, 255.0 / 255.0));
pub const BG_PURPLE_100: BgColor = BgColor(Color::rgb(243.0 / 255.0, 232.0 / 255.0, 255.0 / 255.0));
pub const BG_PURPLE_200: BgColor = BgColor(Color::rgb(233.0 / 255.0, 213.0 / 255.0, 255.0 / 255.0));
pub const BG_PURPLE_300: BgColor = BgColor(Color::rgb(216.0 / 255.0, 180.0 / 255.0, 254.0 / 255.0));
pub const BG_PURPLE_400: BgColor = BgColor(Color::rgb(192.0 / 255.0, 132.0 / 255.0, 252.0 / 255.0));
pub const BG_PURPLE_500: BgColor = BgColor(Color::rgb(168.0 / 255.0, 85.0 / 255.0, 247.0 / 255.0));
pub const BG_PURPLE_600: BgColor = BgColor(Color::rgb(147.0 / 255.0, 51.0 / 255.0, 234.0 / 255.0));
pub const BG_PURPLE_700: BgColor = BgColor(Color::rgb(126.0 / 255.0, 34.0 / 255.0, 206.0 / 255.0));
pub const BG_PURPLE_800: BgColor = BgColor(Color::rgb(107.0 / 255.0, 33.0 / 255.0, 168.0 / 255.0));
pub const BG_PURPLE_900: BgColor = BgColor(Color::rgb(88.0 / 255.0, 28.0 / 255.0, 135.0 / 255.0));
pub const BG_PURPLE_950: BgColor = BgColor(Color::rgb(59.0 / 255.0, 7.0 / 255.0, 100.0 / 255.0));
pub const BG_FUCHSIA_50: BgColor = BgColor(Color::rgb(253.0 / 255.0, 244.0 / 255.0, 255.0 / 255.0));
pub const BG_FUCHSIA_100: BgColor =
    BgColor(Color::rgb(250.0 / 255.0, 232.0 / 255.0, 255.0 / 255.0));
pub const BG_FUCHSIA_200: BgColor =
    BgColor(Color::rgb(245.0 / 255.0, 208.0 / 255.0, 254.0 / 255.0));
pub const BG_FUCHSIA_300: BgColor =
    BgColor(Color::rgb(240.0 / 255.0, 171.0 / 255.0, 252.0 / 255.0));
pub const BG_FUCHSIA_400: BgColor =
    BgColor(Color::rgb(232.0 / 255.0, 121.0 / 255.0, 249.0 / 255.0));
pub const BG_FUCHSIA_500: BgColor = BgColor(Color::rgb(217.0 / 255.0, 70.0 / 255.0, 239.0 / 255.0));
pub const BG_FUCHSIA_600: BgColor = BgColor(Color::rgb(192.0 / 255.0, 38.0 / 255.0, 211.0 / 255.0));
pub const BG_FUCHSIA_700: BgColor = BgColor(Color::rgb(162.0 / 255.0, 28.0 / 255.0, 175.0 / 255.0));
pub const BG_FUCHSIA_800: BgColor = BgColor(Color::rgb(134.0 / 255.0, 25.0 / 255.0, 143.0 / 255.0));
pub const BG_FUCHSIA_900: BgColor = BgColor(Color::rgb(112.0 / 255.0, 26.0 / 255.0, 117.0 / 255.0));
pub const BG_FUCHSIA_950: BgColor = BgColor(Color::rgb(74.0 / 255.0, 4.0 / 255.0, 78.0 / 255.0));
pub const BG_PINK_50: BgColor = BgColor(Color::rgb(253.0 / 255.0, 242.0 / 255.0, 248.0 / 255.0));
pub const BG_PINK_100: BgColor = BgColor(Color::rgb(252.0 / 255.0, 231.0 / 255.0, 243.0 / 255.0));
pub const BG_PINK_200: BgColor = BgColor(Color::rgb(251.0 / 255.0, 207.0 / 255.0, 232.0 / 255.0));
pub const BG_PINK_300: BgColor = BgColor(Color::rgb(249.0 / 255.0, 168.0 / 255.0, 212.0 / 255.0));
pub const BG_PINK_400: BgColor = BgColor(Color::rgb(244.0 / 255.0, 114.0 / 255.0, 182.0 / 255.0));
pub const BG_PINK_500: BgColor = BgColor(Color::rgb(236.0 / 255.0, 72.0 / 255.0, 153.0 / 255.0));
pub const BG_PINK_600: BgColor = BgColor(Color::rgb(219.0 / 255.0, 39.0 / 255.0, 119.0 / 255.0));
pub const BG_PINK_700: BgColor = BgColor(Color::rgb(190.0 / 255.0, 24.0 / 255.0, 93.0 / 255.0));
pub const BG_PINK_800: BgColor = BgColor(Color::rgb(157.0 / 255.0, 23.0 / 255.0, 77.0 / 255.0));
pub const BG_PINK_900: BgColor = BgColor(Color::rgb(131.0 / 255.0, 24.0 / 255.0, 67.0 / 255.0));
pub const BG_PINK_950: BgColor = BgColor(Color::rgb(80.0 / 255.0, 7.0 / 255.0, 36.0 / 255.0));
pub const BG_ROSE_50: BgColor = BgColor(Color::rgb(255.0 / 255.0, 241.0 / 255.0, 242.0 / 255.0));
pub const BG_ROSE_100: BgColor = BgColor(Color::rgb(255.0 / 255.0, 228.0 / 255.0, 230.0 / 255.0));
pub const BG_ROSE_200: BgColor = BgColor(Color::rgb(254.0 / 255.0, 205.0 / 255.0, 211.0 / 255.0));
pub const BG_ROSE_300: BgColor = BgColor(Color::rgb(253.0 / 255.0, 164.0 / 255.0, 175.0 / 255.0));
pub const BG_ROSE_400: BgColor = BgColor(Color::rgb(251.0 / 255.0, 113.0 / 255.0, 133.0 / 255.0));
pub const BG_ROSE_500: BgColor = BgColor(Color::rgb(244.0 / 255.0, 63.0 / 255.0, 94.0 / 255.0));
pub const BG_ROSE_600: BgColor = BgColor(Color::rgb(225.0 / 255.0, 29.0 / 255.0, 72.0 / 255.0));
pub const BG_ROSE_700: BgColor = BgColor(Color::rgb(190.0 / 255.0, 18.0 / 255.0, 60.0 / 255.0));
pub const BG_ROSE_800: BgColor = BgColor(Color::rgb(159.0 / 255.0, 18.0 / 255.0, 57.0 / 255.0));
pub const BG_ROSE_900: BgColor = BgColor(Color::rgb(136.0 / 255.0, 19.0 / 255.0, 55.0 / 255.0));
pub const BG_ROSE_950: BgColor = BgColor(Color::rgb(76.0 / 255.0, 5.0 / 255.0, 25.0 / 255.0));

#[derive(Debug, Clone, Copy)]
pub struct BgColor(Color);

impl Default for BgColor {
    fn default() -> Self {
        Self(Color::WHITE)
    }
}

impl Div<f32> for BgColor {
    type Output = BgColor;

    fn div(self, rhs: f32) -> Self::Output {
        Self(self.0.with_a(rhs))
    }
}

impl From<Color> for BgColor {
    fn from(color: Color) -> Self {
        Self(color)
    }
}

impl From<BackgroundColor> for BgColor {
    fn from(color: BackgroundColor) -> Self {
        Self(color.0)
    }
}

impl ApplyClass for BgColor {
    type Component = BackgroundColor;

    fn apply_class(&self, _: Interaction, component: &mut Self::Component) {
        component.0 = self.0;
    }
}
