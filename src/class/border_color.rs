use crate::class::ApplyClass;
use bevy::{color::Alpha, prelude::Color, ui::BorderColor};
use std::ops::Div;

pub const BORDER_TRANSPARENT: BorderColorClass = BorderColorClass(Color::NONE);
pub const BORDER_BLACK: BorderColorClass = BorderColorClass(Color::BLACK);
pub const BORDER_WHITE: BorderColorClass = BorderColorClass(Color::WHITE);
pub const BORDER_SLATE_50: BorderColorClass =
    BorderColorClass(Color::srgb(248.0 / 255.0, 250.0 / 255.0, 252.0 / 255.0));
pub const BORDER_SLATE_100: BorderColorClass =
    BorderColorClass(Color::srgb(241.0 / 255.0, 245.0 / 255.0, 249.0 / 255.0));
pub const BORDER_SLATE_200: BorderColorClass =
    BorderColorClass(Color::srgb(226.0 / 255.0, 232.0 / 255.0, 240.0 / 255.0));
pub const BORDER_SLATE_300: BorderColorClass =
    BorderColorClass(Color::srgb(203.0 / 255.0, 213.0 / 255.0, 225.0 / 255.0));
pub const BORDER_SLATE_400: BorderColorClass =
    BorderColorClass(Color::srgb(148.0 / 255.0, 163.0 / 255.0, 184.0 / 255.0));
pub const BORDER_SLATE_500: BorderColorClass =
    BorderColorClass(Color::srgb(100.0 / 255.0, 116.0 / 255.0, 139.0 / 255.0));
pub const BORDER_SLATE_600: BorderColorClass =
    BorderColorClass(Color::srgb(71.0 / 255.0, 85.0 / 255.0, 105.0 / 255.0));
pub const BORDER_SLATE_700: BorderColorClass =
    BorderColorClass(Color::srgb(51.0 / 255.0, 65.0 / 255.0, 85.0 / 255.0));
pub const BORDER_SLATE_800: BorderColorClass =
    BorderColorClass(Color::srgb(30.0 / 255.0, 41.0 / 255.0, 59.0 / 255.0));
pub const BORDER_SLATE_900: BorderColorClass =
    BorderColorClass(Color::srgb(15.0 / 255.0, 23.0 / 255.0, 42.0 / 255.0));
pub const BORDER_SLATE_950: BorderColorClass =
    BorderColorClass(Color::srgb(2.0 / 255.0, 6.0 / 255.0, 23.0 / 255.0));
pub const BORDER_GRAY_50: BorderColorClass =
    BorderColorClass(Color::srgb(249.0 / 255.0, 250.0 / 255.0, 251.0 / 255.0));
pub const BORDER_GRAY_100: BorderColorClass =
    BorderColorClass(Color::srgb(243.0 / 255.0, 244.0 / 255.0, 246.0 / 255.0));
pub const BORDER_GRAY_200: BorderColorClass =
    BorderColorClass(Color::srgb(229.0 / 255.0, 231.0 / 255.0, 235.0 / 255.0));
pub const BORDER_GRAY_300: BorderColorClass =
    BorderColorClass(Color::srgb(209.0 / 255.0, 213.0 / 255.0, 219.0 / 255.0));
pub const BORDER_GRAY_400: BorderColorClass =
    BorderColorClass(Color::srgb(156.0 / 255.0, 163.0 / 255.0, 175.0 / 255.0));
pub const BORDER_GRAY_500: BorderColorClass =
    BorderColorClass(Color::srgb(107.0 / 255.0, 114.0 / 255.0, 128.0 / 255.0));
pub const BORDER_GRAY_600: BorderColorClass =
    BorderColorClass(Color::srgb(75.0 / 255.0, 85.0 / 255.0, 99.0 / 255.0));
pub const BORDER_GRAY_700: BorderColorClass =
    BorderColorClass(Color::srgb(55.0 / 255.0, 65.0 / 255.0, 81.0 / 255.0));
pub const BORDER_GRAY_800: BorderColorClass =
    BorderColorClass(Color::srgb(31.0 / 255.0, 41.0 / 255.0, 55.0 / 255.0));
pub const BORDER_GRAY_900: BorderColorClass =
    BorderColorClass(Color::srgb(17.0 / 255.0, 24.0 / 255.0, 39.0 / 255.0));
pub const BORDER_GRAY_950: BorderColorClass =
    BorderColorClass(Color::srgb(3.0 / 255.0, 7.0 / 255.0, 18.0 / 255.0));
pub const BORDER_ZINC_50: BorderColorClass =
    BorderColorClass(Color::srgb(250.0 / 255.0, 250.0 / 255.0, 250.0 / 255.0));
pub const BORDER_ZINC_100: BorderColorClass =
    BorderColorClass(Color::srgb(244.0 / 255.0, 244.0 / 255.0, 245.0 / 255.0));
pub const BORDER_ZINC_200: BorderColorClass =
    BorderColorClass(Color::srgb(228.0 / 255.0, 228.0 / 255.0, 231.0 / 255.0));
pub const BORDER_ZINC_300: BorderColorClass =
    BorderColorClass(Color::srgb(212.0 / 255.0, 212.0 / 255.0, 216.0 / 255.0));
pub const BORDER_ZINC_400: BorderColorClass =
    BorderColorClass(Color::srgb(161.0 / 255.0, 161.0 / 255.0, 170.0 / 255.0));
pub const BORDER_ZINC_500: BorderColorClass =
    BorderColorClass(Color::srgb(113.0 / 255.0, 113.0 / 255.0, 122.0 / 255.0));
pub const BORDER_ZINC_600: BorderColorClass =
    BorderColorClass(Color::srgb(82.0 / 255.0, 82.0 / 255.0, 91.0 / 255.0));
pub const BORDER_ZINC_700: BorderColorClass =
    BorderColorClass(Color::srgb(63.0 / 255.0, 63.0 / 255.0, 70.0 / 255.0));
pub const BORDER_ZINC_800: BorderColorClass =
    BorderColorClass(Color::srgb(39.0 / 255.0, 39.0 / 255.0, 42.0 / 255.0));
pub const BORDER_ZINC_900: BorderColorClass =
    BorderColorClass(Color::srgb(24.0 / 255.0, 24.0 / 255.0, 27.0 / 255.0));
pub const BORDER_ZINC_950: BorderColorClass =
    BorderColorClass(Color::srgb(9.0 / 255.0, 9.0 / 255.0, 11.0 / 255.0));
pub const BORDER_NEUTRAL_50: BorderColorClass =
    BorderColorClass(Color::srgb(250.0 / 255.0, 250.0 / 255.0, 250.0 / 255.0));
pub const BORDER_NEUTRAL_100: BorderColorClass =
    BorderColorClass(Color::srgb(245.0 / 255.0, 245.0 / 255.0, 245.0 / 255.0));
pub const BORDER_NEUTRAL_200: BorderColorClass =
    BorderColorClass(Color::srgb(229.0 / 255.0, 229.0 / 255.0, 229.0 / 255.0));
pub const BORDER_NEUTRAL_300: BorderColorClass =
    BorderColorClass(Color::srgb(212.0 / 255.0, 212.0 / 255.0, 212.0 / 255.0));
pub const BORDER_NEUTRAL_400: BorderColorClass =
    BorderColorClass(Color::srgb(163.0 / 255.0, 163.0 / 255.0, 163.0 / 255.0));
pub const BORDER_NEUTRAL_500: BorderColorClass =
    BorderColorClass(Color::srgb(115.0 / 255.0, 115.0 / 255.0, 115.0 / 255.0));
pub const BORDER_NEUTRAL_600: BorderColorClass =
    BorderColorClass(Color::srgb(82.0 / 255.0, 82.0 / 255.0, 82.0 / 255.0));
pub const BORDER_NEUTRAL_700: BorderColorClass =
    BorderColorClass(Color::srgb(64.0 / 255.0, 64.0 / 255.0, 64.0 / 255.0));
pub const BORDER_NEUTRAL_800: BorderColorClass =
    BorderColorClass(Color::srgb(38.0 / 255.0, 38.0 / 255.0, 38.0 / 255.0));
pub const BORDER_NEUTRAL_900: BorderColorClass =
    BorderColorClass(Color::srgb(23.0 / 255.0, 23.0 / 255.0, 23.0 / 255.0));
pub const BORDER_NEUTRAL_950: BorderColorClass =
    BorderColorClass(Color::srgb(10.0 / 255.0, 10.0 / 255.0, 10.0 / 255.0));
pub const BORDER_STONE_50: BorderColorClass =
    BorderColorClass(Color::srgb(250.0 / 255.0, 250.0 / 255.0, 249.0 / 255.0));
pub const BORDER_STONE_100: BorderColorClass =
    BorderColorClass(Color::srgb(245.0 / 255.0, 245.0 / 255.0, 244.0 / 255.0));
pub const BORDER_STONE_200: BorderColorClass =
    BorderColorClass(Color::srgb(231.0 / 255.0, 229.0 / 255.0, 228.0 / 255.0));
pub const BORDER_STONE_300: BorderColorClass =
    BorderColorClass(Color::srgb(214.0 / 255.0, 211.0 / 255.0, 209.0 / 255.0));
pub const BORDER_STONE_400: BorderColorClass =
    BorderColorClass(Color::srgb(168.0 / 255.0, 162.0 / 255.0, 158.0 / 255.0));
pub const BORDER_STONE_500: BorderColorClass =
    BorderColorClass(Color::srgb(120.0 / 255.0, 113.0 / 255.0, 108.0 / 255.0));
pub const BORDER_STONE_600: BorderColorClass =
    BorderColorClass(Color::srgb(87.0 / 255.0, 83.0 / 255.0, 78.0 / 255.0));
pub const BORDER_STONE_700: BorderColorClass =
    BorderColorClass(Color::srgb(68.0 / 255.0, 64.0 / 255.0, 60.0 / 255.0));
pub const BORDER_STONE_800: BorderColorClass =
    BorderColorClass(Color::srgb(41.0 / 255.0, 37.0 / 255.0, 36.0 / 255.0));
pub const BORDER_STONE_900: BorderColorClass =
    BorderColorClass(Color::srgb(28.0 / 255.0, 25.0 / 255.0, 23.0 / 255.0));
pub const BORDER_STONE_950: BorderColorClass =
    BorderColorClass(Color::srgb(12.0 / 255.0, 10.0 / 255.0, 9.0 / 255.0));
pub const BORDER_RED_50: BorderColorClass =
    BorderColorClass(Color::srgb(254.0 / 255.0, 242.0 / 255.0, 242.0 / 255.0));
pub const BORDER_RED_100: BorderColorClass =
    BorderColorClass(Color::srgb(254.0 / 255.0, 226.0 / 255.0, 226.0 / 255.0));
pub const BORDER_RED_200: BorderColorClass =
    BorderColorClass(Color::srgb(254.0 / 255.0, 202.0 / 255.0, 202.0 / 255.0));
pub const BORDER_RED_300: BorderColorClass =
    BorderColorClass(Color::srgb(252.0 / 255.0, 165.0 / 255.0, 165.0 / 255.0));
pub const BORDER_RED_400: BorderColorClass =
    BorderColorClass(Color::srgb(248.0 / 255.0, 113.0 / 255.0, 113.0 / 255.0));
pub const BORDER_RED_500: BorderColorClass =
    BorderColorClass(Color::srgb(239.0 / 255.0, 68.0 / 255.0, 68.0 / 255.0));
pub const BORDER_RED_600: BorderColorClass =
    BorderColorClass(Color::srgb(220.0 / 255.0, 38.0 / 255.0, 38.0 / 255.0));
pub const BORDER_RED_700: BorderColorClass =
    BorderColorClass(Color::srgb(185.0 / 255.0, 28.0 / 255.0, 28.0 / 255.0));
pub const BORDER_RED_800: BorderColorClass =
    BorderColorClass(Color::srgb(153.0 / 255.0, 27.0 / 255.0, 27.0 / 255.0));
pub const BORDER_RED_900: BorderColorClass =
    BorderColorClass(Color::srgb(127.0 / 255.0, 29.0 / 255.0, 29.0 / 255.0));
pub const BORDER_RED_950: BorderColorClass =
    BorderColorClass(Color::srgb(69.0 / 255.0, 10.0 / 255.0, 10.0 / 255.0));
pub const BORDER_ORANGE_50: BorderColorClass =
    BorderColorClass(Color::srgb(255.0 / 255.0, 247.0 / 255.0, 237.0 / 255.0));
pub const BORDER_ORANGE_100: BorderColorClass =
    BorderColorClass(Color::srgb(255.0 / 255.0, 237.0 / 255.0, 213.0 / 255.0));
pub const BORDER_ORANGE_200: BorderColorClass =
    BorderColorClass(Color::srgb(254.0 / 255.0, 215.0 / 255.0, 170.0 / 255.0));
pub const BORDER_ORANGE_300: BorderColorClass =
    BorderColorClass(Color::srgb(253.0 / 255.0, 186.0 / 255.0, 116.0 / 255.0));
pub const BORDER_ORANGE_400: BorderColorClass =
    BorderColorClass(Color::srgb(251.0 / 255.0, 146.0 / 255.0, 60.0 / 255.0));
pub const BORDER_ORANGE_500: BorderColorClass =
    BorderColorClass(Color::srgb(249.0 / 255.0, 115.0 / 255.0, 22.0 / 255.0));
pub const BORDER_ORANGE_600: BorderColorClass =
    BorderColorClass(Color::srgb(234.0 / 255.0, 88.0 / 255.0, 12.0 / 255.0));
pub const BORDER_ORANGE_700: BorderColorClass =
    BorderColorClass(Color::srgb(194.0 / 255.0, 65.0 / 255.0, 12.0 / 255.0));
pub const BORDER_ORANGE_800: BorderColorClass =
    BorderColorClass(Color::srgb(154.0 / 255.0, 52.0 / 255.0, 18.0 / 255.0));
pub const BORDER_ORANGE_900: BorderColorClass =
    BorderColorClass(Color::srgb(124.0 / 255.0, 45.0 / 255.0, 18.0 / 255.0));
pub const BORDER_ORANGE_950: BorderColorClass =
    BorderColorClass(Color::srgb(67.0 / 255.0, 20.0 / 255.0, 7.0 / 255.0));
pub const BORDER_AMBER_50: BorderColorClass =
    BorderColorClass(Color::srgb(255.0 / 255.0, 251.0 / 255.0, 235.0 / 255.0));
pub const BORDER_AMBER_100: BorderColorClass =
    BorderColorClass(Color::srgb(254.0 / 255.0, 243.0 / 255.0, 199.0 / 255.0));
pub const BORDER_AMBER_200: BorderColorClass =
    BorderColorClass(Color::srgb(253.0 / 255.0, 230.0 / 255.0, 138.0 / 255.0));
pub const BORDER_AMBER_300: BorderColorClass =
    BorderColorClass(Color::srgb(252.0 / 255.0, 211.0 / 255.0, 77.0 / 255.0));
pub const BORDER_AMBER_400: BorderColorClass =
    BorderColorClass(Color::srgb(251.0 / 255.0, 191.0 / 255.0, 36.0 / 255.0));
pub const BORDER_AMBER_500: BorderColorClass =
    BorderColorClass(Color::srgb(245.0 / 255.0, 158.0 / 255.0, 11.0 / 255.0));
pub const BORDER_AMBER_600: BorderColorClass =
    BorderColorClass(Color::srgb(217.0 / 255.0, 119.0 / 255.0, 6.0 / 255.0));
pub const BORDER_AMBER_700: BorderColorClass =
    BorderColorClass(Color::srgb(180.0 / 255.0, 83.0 / 255.0, 9.0 / 255.0));
pub const BORDER_AMBER_800: BorderColorClass =
    BorderColorClass(Color::srgb(146.0 / 255.0, 64.0 / 255.0, 14.0 / 255.0));
pub const BORDER_AMBER_900: BorderColorClass =
    BorderColorClass(Color::srgb(120.0 / 255.0, 53.0 / 255.0, 15.0 / 255.0));
pub const BORDER_AMBER_950: BorderColorClass =
    BorderColorClass(Color::srgb(69.0 / 255.0, 26.0 / 255.0, 3.0 / 255.0));
pub const BORDER_YELLOW_50: BorderColorClass =
    BorderColorClass(Color::srgb(254.0 / 255.0, 252.0 / 255.0, 232.0 / 255.0));
pub const BORDER_YELLOW_100: BorderColorClass =
    BorderColorClass(Color::srgb(254.0 / 255.0, 249.0 / 255.0, 195.0 / 255.0));
pub const BORDER_YELLOW_200: BorderColorClass =
    BorderColorClass(Color::srgb(254.0 / 255.0, 240.0 / 255.0, 138.0 / 255.0));
pub const BORDER_YELLOW_300: BorderColorClass =
    BorderColorClass(Color::srgb(253.0 / 255.0, 224.0 / 255.0, 71.0 / 255.0));
pub const BORDER_YELLOW_400: BorderColorClass =
    BorderColorClass(Color::srgb(250.0 / 255.0, 204.0 / 255.0, 21.0 / 255.0));
pub const BORDER_YELLOW_500: BorderColorClass =
    BorderColorClass(Color::srgb(234.0 / 255.0, 179.0 / 255.0, 8.0 / 255.0));
pub const BORDER_YELLOW_600: BorderColorClass =
    BorderColorClass(Color::srgb(202.0 / 255.0, 138.0 / 255.0, 4.0 / 255.0));
pub const BORDER_YELLOW_700: BorderColorClass =
    BorderColorClass(Color::srgb(161.0 / 255.0, 98.0 / 255.0, 7.0 / 255.0));
pub const BORDER_YELLOW_800: BorderColorClass =
    BorderColorClass(Color::srgb(133.0 / 255.0, 77.0 / 255.0, 14.0 / 255.0));
pub const BORDER_YELLOW_900: BorderColorClass =
    BorderColorClass(Color::srgb(113.0 / 255.0, 63.0 / 255.0, 18.0 / 255.0));
pub const BORDER_YELLOW_950: BorderColorClass =
    BorderColorClass(Color::srgb(66.0 / 255.0, 32.0 / 255.0, 6.0 / 255.0));
pub const BORDER_LIME_50: BorderColorClass =
    BorderColorClass(Color::srgb(247.0 / 255.0, 254.0 / 255.0, 231.0 / 255.0));
pub const BORDER_LIME_100: BorderColorClass =
    BorderColorClass(Color::srgb(236.0 / 255.0, 252.0 / 255.0, 203.0 / 255.0));
pub const BORDER_LIME_200: BorderColorClass =
    BorderColorClass(Color::srgb(217.0 / 255.0, 249.0 / 255.0, 157.0 / 255.0));
pub const BORDER_LIME_300: BorderColorClass =
    BorderColorClass(Color::srgb(190.0 / 255.0, 242.0 / 255.0, 100.0 / 255.0));
pub const BORDER_LIME_400: BorderColorClass =
    BorderColorClass(Color::srgb(163.0 / 255.0, 230.0 / 255.0, 53.0 / 255.0));
pub const BORDER_LIME_500: BorderColorClass =
    BorderColorClass(Color::srgb(132.0 / 255.0, 204.0 / 255.0, 22.0 / 255.0));
pub const BORDER_LIME_600: BorderColorClass =
    BorderColorClass(Color::srgb(101.0 / 255.0, 163.0 / 255.0, 13.0 / 255.0));
pub const BORDER_LIME_700: BorderColorClass =
    BorderColorClass(Color::srgb(77.0 / 255.0, 124.0 / 255.0, 15.0 / 255.0));
pub const BORDER_LIME_800: BorderColorClass =
    BorderColorClass(Color::srgb(63.0 / 255.0, 98.0 / 255.0, 18.0 / 255.0));
pub const BORDER_LIME_900: BorderColorClass =
    BorderColorClass(Color::srgb(54.0 / 255.0, 83.0 / 255.0, 20.0 / 255.0));
pub const BORDER_LIME_950: BorderColorClass =
    BorderColorClass(Color::srgb(26.0 / 255.0, 46.0 / 255.0, 5.0 / 255.0));
pub const BORDER_GREEN_50: BorderColorClass =
    BorderColorClass(Color::srgb(240.0 / 255.0, 253.0 / 255.0, 244.0 / 255.0));
pub const BORDER_GREEN_100: BorderColorClass =
    BorderColorClass(Color::srgb(220.0 / 255.0, 252.0 / 255.0, 231.0 / 255.0));
pub const BORDER_GREEN_200: BorderColorClass =
    BorderColorClass(Color::srgb(187.0 / 255.0, 247.0 / 255.0, 208.0 / 255.0));
pub const BORDER_GREEN_300: BorderColorClass =
    BorderColorClass(Color::srgb(134.0 / 255.0, 239.0 / 255.0, 172.0 / 255.0));
pub const BORDER_GREEN_400: BorderColorClass =
    BorderColorClass(Color::srgb(74.0 / 255.0, 222.0 / 255.0, 128.0 / 255.0));
pub const BORDER_GREEN_500: BorderColorClass =
    BorderColorClass(Color::srgb(34.0 / 255.0, 197.0 / 255.0, 94.0 / 255.0));
pub const BORDER_GREEN_600: BorderColorClass =
    BorderColorClass(Color::srgb(22.0 / 255.0, 163.0 / 255.0, 74.0 / 255.0));
pub const BORDER_GREEN_700: BorderColorClass =
    BorderColorClass(Color::srgb(21.0 / 255.0, 128.0 / 255.0, 61.0 / 255.0));
pub const BORDER_GREEN_800: BorderColorClass =
    BorderColorClass(Color::srgb(22.0 / 255.0, 101.0 / 255.0, 52.0 / 255.0));
pub const BORDER_GREEN_900: BorderColorClass =
    BorderColorClass(Color::srgb(20.0 / 255.0, 83.0 / 255.0, 45.0 / 255.0));
pub const BORDER_GREEN_950: BorderColorClass =
    BorderColorClass(Color::srgb(5.0 / 255.0, 46.0 / 255.0, 22.0 / 255.0));
pub const BORDER_EMERALD_50: BorderColorClass =
    BorderColorClass(Color::srgb(236.0 / 255.0, 253.0 / 255.0, 245.0 / 255.0));
pub const BORDER_EMERALD_100: BorderColorClass =
    BorderColorClass(Color::srgb(209.0 / 255.0, 250.0 / 255.0, 229.0 / 255.0));
pub const BORDER_EMERALD_200: BorderColorClass =
    BorderColorClass(Color::srgb(167.0 / 255.0, 243.0 / 255.0, 208.0 / 255.0));
pub const BORDER_EMERALD_300: BorderColorClass =
    BorderColorClass(Color::srgb(110.0 / 255.0, 231.0 / 255.0, 183.0 / 255.0));
pub const BORDER_EMERALD_400: BorderColorClass =
    BorderColorClass(Color::srgb(52.0 / 255.0, 211.0 / 255.0, 153.0 / 255.0));
pub const BORDER_EMERALD_500: BorderColorClass =
    BorderColorClass(Color::srgb(16.0 / 255.0, 185.0 / 255.0, 129.0 / 255.0));
pub const BORDER_EMERALD_600: BorderColorClass =
    BorderColorClass(Color::srgb(5.0 / 255.0, 150.0 / 255.0, 105.0 / 255.0));
pub const BORDER_EMERALD_700: BorderColorClass =
    BorderColorClass(Color::srgb(4.0 / 255.0, 120.0 / 255.0, 87.0 / 255.0));
pub const BORDER_EMERALD_800: BorderColorClass =
    BorderColorClass(Color::srgb(6.0 / 255.0, 95.0 / 255.0, 70.0 / 255.0));
pub const BORDER_EMERALD_900: BorderColorClass =
    BorderColorClass(Color::srgb(6.0 / 255.0, 78.0 / 255.0, 59.0 / 255.0));
pub const BORDER_EMERALD_950: BorderColorClass =
    BorderColorClass(Color::srgb(2.0 / 255.0, 44.0 / 255.0, 34.0 / 255.0));
pub const BORDER_TEAL_50: BorderColorClass =
    BorderColorClass(Color::srgb(240.0 / 255.0, 253.0 / 255.0, 250.0 / 255.0));
pub const BORDER_TEAL_100: BorderColorClass =
    BorderColorClass(Color::srgb(204.0 / 255.0, 251.0 / 255.0, 241.0 / 255.0));
pub const BORDER_TEAL_200: BorderColorClass =
    BorderColorClass(Color::srgb(153.0 / 255.0, 246.0 / 255.0, 228.0 / 255.0));
pub const BORDER_TEAL_300: BorderColorClass =
    BorderColorClass(Color::srgb(94.0 / 255.0, 234.0 / 255.0, 212.0 / 255.0));
pub const BORDER_TEAL_400: BorderColorClass =
    BorderColorClass(Color::srgb(45.0 / 255.0, 212.0 / 255.0, 191.0 / 255.0));
pub const BORDER_TEAL_500: BorderColorClass =
    BorderColorClass(Color::srgb(20.0 / 255.0, 184.0 / 255.0, 166.0 / 255.0));
pub const BORDER_TEAL_600: BorderColorClass =
    BorderColorClass(Color::srgb(13.0 / 255.0, 148.0 / 255.0, 136.0 / 255.0));
pub const BORDER_TEAL_700: BorderColorClass =
    BorderColorClass(Color::srgb(15.0 / 255.0, 118.0 / 255.0, 110.0 / 255.0));
pub const BORDER_TEAL_800: BorderColorClass =
    BorderColorClass(Color::srgb(17.0 / 255.0, 94.0 / 255.0, 89.0 / 255.0));
pub const BORDER_TEAL_900: BorderColorClass =
    BorderColorClass(Color::srgb(19.0 / 255.0, 78.0 / 255.0, 74.0 / 255.0));
pub const BORDER_TEAL_950: BorderColorClass =
    BorderColorClass(Color::srgb(4.0 / 255.0, 47.0 / 255.0, 46.0 / 255.0));
pub const BORDER_CYAN_50: BorderColorClass =
    BorderColorClass(Color::srgb(236.0 / 255.0, 254.0 / 255.0, 255.0 / 255.0));
pub const BORDER_CYAN_100: BorderColorClass =
    BorderColorClass(Color::srgb(207.0 / 255.0, 250.0 / 255.0, 254.0 / 255.0));
pub const BORDER_CYAN_200: BorderColorClass =
    BorderColorClass(Color::srgb(165.0 / 255.0, 243.0 / 255.0, 252.0 / 255.0));
pub const BORDER_CYAN_300: BorderColorClass =
    BorderColorClass(Color::srgb(103.0 / 255.0, 232.0 / 255.0, 249.0 / 255.0));
pub const BORDER_CYAN_400: BorderColorClass =
    BorderColorClass(Color::srgb(34.0 / 255.0, 211.0 / 255.0, 238.0 / 255.0));
pub const BORDER_CYAN_500: BorderColorClass =
    BorderColorClass(Color::srgb(6.0 / 255.0, 182.0 / 255.0, 212.0 / 255.0));
pub const BORDER_CYAN_600: BorderColorClass =
    BorderColorClass(Color::srgb(8.0 / 255.0, 145.0 / 255.0, 178.0 / 255.0));
pub const BORDER_CYAN_700: BorderColorClass =
    BorderColorClass(Color::srgb(14.0 / 255.0, 116.0 / 255.0, 144.0 / 255.0));
pub const BORDER_CYAN_800: BorderColorClass =
    BorderColorClass(Color::srgb(21.0 / 255.0, 94.0 / 255.0, 117.0 / 255.0));
pub const BORDER_CYAN_900: BorderColorClass =
    BorderColorClass(Color::srgb(22.0 / 255.0, 78.0 / 255.0, 99.0 / 255.0));
pub const BORDER_CYAN_950: BorderColorClass =
    BorderColorClass(Color::srgb(8.0 / 255.0, 51.0 / 255.0, 68.0 / 255.0));
pub const BORDER_SKY_50: BorderColorClass =
    BorderColorClass(Color::srgb(240.0 / 255.0, 249.0 / 255.0, 255.0 / 255.0));
pub const BORDER_SKY_100: BorderColorClass =
    BorderColorClass(Color::srgb(224.0 / 255.0, 242.0 / 255.0, 254.0 / 255.0));
pub const BORDER_SKY_200: BorderColorClass =
    BorderColorClass(Color::srgb(186.0 / 255.0, 230.0 / 255.0, 253.0 / 255.0));
pub const BORDER_SKY_300: BorderColorClass =
    BorderColorClass(Color::srgb(125.0 / 255.0, 211.0 / 255.0, 252.0 / 255.0));
pub const BORDER_SKY_400: BorderColorClass =
    BorderColorClass(Color::srgb(56.0 / 255.0, 189.0 / 255.0, 248.0 / 255.0));
pub const BORDER_SKY_500: BorderColorClass =
    BorderColorClass(Color::srgb(14.0 / 255.0, 165.0 / 255.0, 233.0 / 255.0));
pub const BORDER_SKY_600: BorderColorClass =
    BorderColorClass(Color::srgb(2.0 / 255.0, 132.0 / 255.0, 199.0 / 255.0));
pub const BORDER_SKY_700: BorderColorClass =
    BorderColorClass(Color::srgb(3.0 / 255.0, 105.0 / 255.0, 161.0 / 255.0));
pub const BORDER_SKY_800: BorderColorClass =
    BorderColorClass(Color::srgb(7.0 / 255.0, 89.0 / 255.0, 133.0 / 255.0));
pub const BORDER_SKY_900: BorderColorClass =
    BorderColorClass(Color::srgb(12.0 / 255.0, 74.0 / 255.0, 110.0 / 255.0));
pub const BORDER_SKY_950: BorderColorClass =
    BorderColorClass(Color::srgb(8.0 / 255.0, 47.0 / 255.0, 73.0 / 255.0));
pub const BORDER_BLUE_50: BorderColorClass =
    BorderColorClass(Color::srgb(239.0 / 255.0, 246.0 / 255.0, 255.0 / 255.0));
pub const BORDER_BLUE_100: BorderColorClass =
    BorderColorClass(Color::srgb(219.0 / 255.0, 234.0 / 255.0, 254.0 / 255.0));
pub const BORDER_BLUE_200: BorderColorClass =
    BorderColorClass(Color::srgb(191.0 / 255.0, 219.0 / 255.0, 254.0 / 255.0));
pub const BORDER_BLUE_300: BorderColorClass =
    BorderColorClass(Color::srgb(147.0 / 255.0, 197.0 / 255.0, 253.0 / 255.0));
pub const BORDER_BLUE_400: BorderColorClass =
    BorderColorClass(Color::srgb(96.0 / 255.0, 165.0 / 255.0, 250.0 / 255.0));
pub const BORDER_BLUE_500: BorderColorClass =
    BorderColorClass(Color::srgb(59.0 / 255.0, 130.0 / 255.0, 246.0 / 255.0));
pub const BORDER_BLUE_600: BorderColorClass =
    BorderColorClass(Color::srgb(37.0 / 255.0, 99.0 / 255.0, 235.0 / 255.0));
pub const BORDER_BLUE_700: BorderColorClass =
    BorderColorClass(Color::srgb(29.0 / 255.0, 78.0 / 255.0, 216.0 / 255.0));
pub const BORDER_BLUE_800: BorderColorClass =
    BorderColorClass(Color::srgb(30.0 / 255.0, 64.0 / 255.0, 175.0 / 255.0));
pub const BORDER_BLUE_900: BorderColorClass =
    BorderColorClass(Color::srgb(30.0 / 255.0, 58.0 / 255.0, 138.0 / 255.0));
pub const BORDER_BLUE_950: BorderColorClass =
    BorderColorClass(Color::srgb(23.0 / 255.0, 37.0 / 255.0, 84.0 / 255.0));
pub const BORDER_INDIGO_50: BorderColorClass =
    BorderColorClass(Color::srgb(238.0 / 255.0, 242.0 / 255.0, 255.0 / 255.0));
pub const BORDER_INDIGO_100: BorderColorClass =
    BorderColorClass(Color::srgb(224.0 / 255.0, 231.0 / 255.0, 255.0 / 255.0));
pub const BORDER_INDIGO_200: BorderColorClass =
    BorderColorClass(Color::srgb(199.0 / 255.0, 210.0 / 255.0, 254.0 / 255.0));
pub const BORDER_INDIGO_300: BorderColorClass =
    BorderColorClass(Color::srgb(165.0 / 255.0, 180.0 / 255.0, 252.0 / 255.0));
pub const BORDER_INDIGO_400: BorderColorClass =
    BorderColorClass(Color::srgb(129.0 / 255.0, 140.0 / 255.0, 248.0 / 255.0));
pub const BORDER_INDIGO_500: BorderColorClass =
    BorderColorClass(Color::srgb(99.0 / 255.0, 102.0 / 255.0, 241.0 / 255.0));
pub const BORDER_INDIGO_600: BorderColorClass =
    BorderColorClass(Color::srgb(79.0 / 255.0, 70.0 / 255.0, 229.0 / 255.0));
pub const BORDER_INDIGO_700: BorderColorClass =
    BorderColorClass(Color::srgb(67.0 / 255.0, 56.0 / 255.0, 202.0 / 255.0));
pub const BORDER_INDIGO_800: BorderColorClass =
    BorderColorClass(Color::srgb(55.0 / 255.0, 48.0 / 255.0, 163.0 / 255.0));
pub const BORDER_INDIGO_900: BorderColorClass =
    BorderColorClass(Color::srgb(49.0 / 255.0, 46.0 / 255.0, 129.0 / 255.0));
pub const BORDER_INDIGO_950: BorderColorClass =
    BorderColorClass(Color::srgb(30.0 / 255.0, 27.0 / 255.0, 75.0 / 255.0));
pub const BORDER_VIOLET_50: BorderColorClass =
    BorderColorClass(Color::srgb(245.0 / 255.0, 243.0 / 255.0, 255.0 / 255.0));
pub const BORDER_VIOLET_100: BorderColorClass =
    BorderColorClass(Color::srgb(237.0 / 255.0, 233.0 / 255.0, 254.0 / 255.0));
pub const BORDER_VIOLET_200: BorderColorClass =
    BorderColorClass(Color::srgb(221.0 / 255.0, 214.0 / 255.0, 254.0 / 255.0));
pub const BORDER_VIOLET_300: BorderColorClass =
    BorderColorClass(Color::srgb(196.0 / 255.0, 181.0 / 255.0, 253.0 / 255.0));
pub const BORDER_VIOLET_400: BorderColorClass =
    BorderColorClass(Color::srgb(167.0 / 255.0, 139.0 / 255.0, 250.0 / 255.0));
pub const BORDER_VIOLET_500: BorderColorClass =
    BorderColorClass(Color::srgb(139.0 / 255.0, 92.0 / 255.0, 246.0 / 255.0));
pub const BORDER_VIOLET_600: BorderColorClass =
    BorderColorClass(Color::srgb(124.0 / 255.0, 58.0 / 255.0, 237.0 / 255.0));
pub const BORDER_VIOLET_700: BorderColorClass =
    BorderColorClass(Color::srgb(109.0 / 255.0, 40.0 / 255.0, 217.0 / 255.0));
pub const BORDER_VIOLET_800: BorderColorClass =
    BorderColorClass(Color::srgb(91.0 / 255.0, 33.0 / 255.0, 182.0 / 255.0));
pub const BORDER_VIOLET_900: BorderColorClass =
    BorderColorClass(Color::srgb(76.0 / 255.0, 29.0 / 255.0, 149.0 / 255.0));
pub const BORDER_VIOLET_950: BorderColorClass =
    BorderColorClass(Color::srgb(46.0 / 255.0, 16.0 / 255.0, 101.0 / 255.0));
pub const BORDER_PURPLE_50: BorderColorClass =
    BorderColorClass(Color::srgb(250.0 / 255.0, 245.0 / 255.0, 255.0 / 255.0));
pub const BORDER_PURPLE_100: BorderColorClass =
    BorderColorClass(Color::srgb(243.0 / 255.0, 232.0 / 255.0, 255.0 / 255.0));
pub const BORDER_PURPLE_200: BorderColorClass =
    BorderColorClass(Color::srgb(233.0 / 255.0, 213.0 / 255.0, 255.0 / 255.0));
pub const BORDER_PURPLE_300: BorderColorClass =
    BorderColorClass(Color::srgb(216.0 / 255.0, 180.0 / 255.0, 254.0 / 255.0));
pub const BORDER_PURPLE_400: BorderColorClass =
    BorderColorClass(Color::srgb(192.0 / 255.0, 132.0 / 255.0, 252.0 / 255.0));
pub const BORDER_PURPLE_500: BorderColorClass =
    BorderColorClass(Color::srgb(168.0 / 255.0, 85.0 / 255.0, 247.0 / 255.0));
pub const BORDER_PURPLE_600: BorderColorClass =
    BorderColorClass(Color::srgb(147.0 / 255.0, 51.0 / 255.0, 234.0 / 255.0));
pub const BORDER_PURPLE_700: BorderColorClass =
    BorderColorClass(Color::srgb(126.0 / 255.0, 34.0 / 255.0, 206.0 / 255.0));
pub const BORDER_PURPLE_800: BorderColorClass =
    BorderColorClass(Color::srgb(107.0 / 255.0, 33.0 / 255.0, 168.0 / 255.0));
pub const BORDER_PURPLE_900: BorderColorClass =
    BorderColorClass(Color::srgb(88.0 / 255.0, 28.0 / 255.0, 135.0 / 255.0));
pub const BORDER_PURPLE_950: BorderColorClass =
    BorderColorClass(Color::srgb(59.0 / 255.0, 7.0 / 255.0, 100.0 / 255.0));
pub const BORDER_FUCHSIA_50: BorderColorClass =
    BorderColorClass(Color::srgb(253.0 / 255.0, 244.0 / 255.0, 255.0 / 255.0));
pub const BORDER_FUCHSIA_100: BorderColorClass =
    BorderColorClass(Color::srgb(250.0 / 255.0, 232.0 / 255.0, 255.0 / 255.0));
pub const BORDER_FUCHSIA_200: BorderColorClass =
    BorderColorClass(Color::srgb(245.0 / 255.0, 208.0 / 255.0, 254.0 / 255.0));
pub const BORDER_FUCHSIA_300: BorderColorClass =
    BorderColorClass(Color::srgb(240.0 / 255.0, 171.0 / 255.0, 252.0 / 255.0));
pub const BORDER_FUCHSIA_400: BorderColorClass =
    BorderColorClass(Color::srgb(232.0 / 255.0, 121.0 / 255.0, 249.0 / 255.0));
pub const BORDER_FUCHSIA_500: BorderColorClass =
    BorderColorClass(Color::srgb(217.0 / 255.0, 70.0 / 255.0, 239.0 / 255.0));
pub const BORDER_FUCHSIA_600: BorderColorClass =
    BorderColorClass(Color::srgb(192.0 / 255.0, 38.0 / 255.0, 211.0 / 255.0));
pub const BORDER_FUCHSIA_700: BorderColorClass =
    BorderColorClass(Color::srgb(162.0 / 255.0, 28.0 / 255.0, 175.0 / 255.0));
pub const BORDER_FUCHSIA_800: BorderColorClass =
    BorderColorClass(Color::srgb(134.0 / 255.0, 25.0 / 255.0, 143.0 / 255.0));
pub const BORDER_FUCHSIA_900: BorderColorClass =
    BorderColorClass(Color::srgb(112.0 / 255.0, 26.0 / 255.0, 117.0 / 255.0));
pub const BORDER_FUCHSIA_950: BorderColorClass =
    BorderColorClass(Color::srgb(74.0 / 255.0, 4.0 / 255.0, 78.0 / 255.0));
pub const BORDER_PINK_50: BorderColorClass =
    BorderColorClass(Color::srgb(253.0 / 255.0, 242.0 / 255.0, 248.0 / 255.0));
pub const BORDER_PINK_100: BorderColorClass =
    BorderColorClass(Color::srgb(252.0 / 255.0, 231.0 / 255.0, 243.0 / 255.0));
pub const BORDER_PINK_200: BorderColorClass =
    BorderColorClass(Color::srgb(251.0 / 255.0, 207.0 / 255.0, 232.0 / 255.0));
pub const BORDER_PINK_300: BorderColorClass =
    BorderColorClass(Color::srgb(249.0 / 255.0, 168.0 / 255.0, 212.0 / 255.0));
pub const BORDER_PINK_400: BorderColorClass =
    BorderColorClass(Color::srgb(244.0 / 255.0, 114.0 / 255.0, 182.0 / 255.0));
pub const BORDER_PINK_500: BorderColorClass =
    BorderColorClass(Color::srgb(236.0 / 255.0, 72.0 / 255.0, 153.0 / 255.0));
pub const BORDER_PINK_600: BorderColorClass =
    BorderColorClass(Color::srgb(219.0 / 255.0, 39.0 / 255.0, 119.0 / 255.0));
pub const BORDER_PINK_700: BorderColorClass =
    BorderColorClass(Color::srgb(190.0 / 255.0, 24.0 / 255.0, 93.0 / 255.0));
pub const BORDER_PINK_800: BorderColorClass =
    BorderColorClass(Color::srgb(157.0 / 255.0, 23.0 / 255.0, 77.0 / 255.0));
pub const BORDER_PINK_900: BorderColorClass =
    BorderColorClass(Color::srgb(131.0 / 255.0, 24.0 / 255.0, 67.0 / 255.0));
pub const BORDER_PINK_950: BorderColorClass =
    BorderColorClass(Color::srgb(80.0 / 255.0, 7.0 / 255.0, 36.0 / 255.0));
pub const BORDER_ROSE_50: BorderColorClass =
    BorderColorClass(Color::srgb(255.0 / 255.0, 241.0 / 255.0, 242.0 / 255.0));
pub const BORDER_ROSE_100: BorderColorClass =
    BorderColorClass(Color::srgb(255.0 / 255.0, 228.0 / 255.0, 230.0 / 255.0));
pub const BORDER_ROSE_200: BorderColorClass =
    BorderColorClass(Color::srgb(254.0 / 255.0, 205.0 / 255.0, 211.0 / 255.0));
pub const BORDER_ROSE_300: BorderColorClass =
    BorderColorClass(Color::srgb(253.0 / 255.0, 164.0 / 255.0, 175.0 / 255.0));
pub const BORDER_ROSE_400: BorderColorClass =
    BorderColorClass(Color::srgb(251.0 / 255.0, 113.0 / 255.0, 133.0 / 255.0));
pub const BORDER_ROSE_500: BorderColorClass =
    BorderColorClass(Color::srgb(244.0 / 255.0, 63.0 / 255.0, 94.0 / 255.0));
pub const BORDER_ROSE_600: BorderColorClass =
    BorderColorClass(Color::srgb(225.0 / 255.0, 29.0 / 255.0, 72.0 / 255.0));
pub const BORDER_ROSE_700: BorderColorClass =
    BorderColorClass(Color::srgb(190.0 / 255.0, 18.0 / 255.0, 60.0 / 255.0));
pub const BORDER_ROSE_800: BorderColorClass =
    BorderColorClass(Color::srgb(159.0 / 255.0, 18.0 / 255.0, 57.0 / 255.0));
pub const BORDER_ROSE_900: BorderColorClass =
    BorderColorClass(Color::srgb(136.0 / 255.0, 19.0 / 255.0, 55.0 / 255.0));
pub const BORDER_ROSE_950: BorderColorClass =
    BorderColorClass(Color::srgb(76.0 / 255.0, 5.0 / 255.0, 25.0 / 255.0));

#[derive(Debug, Clone, Copy)]
pub struct BorderColorClass(Color);

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

impl ApplyClass for BorderColorClass {
    type Component = BorderColor;

    fn apply_class(&self, component: &mut Self::Component) {
        component.0 = self.0;
    }
}
