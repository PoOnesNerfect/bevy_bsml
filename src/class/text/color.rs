use super::{ApplyClass, TextClass};
use bevy::{prelude::Color, text::Text};

pub const TEXT_TRANSPARENT: TextClass = TextClass::TextColor(TextColor(Color::NONE));
pub const TEXT_BLACK: TextClass = TextClass::TextColor(TextColor(Color::BLACK));
pub const TEXT_WHITE: TextClass = TextClass::TextColor(TextColor(Color::WHITE));
pub const TEXT_SLATE_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    248.0 / 255.0,
    250.0 / 255.0,
    252.0 / 255.0,
)));
pub const TEXT_SLATE_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    241.0 / 255.0,
    245.0 / 255.0,
    249.0 / 255.0,
)));
pub const TEXT_SLATE_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    226.0 / 255.0,
    232.0 / 255.0,
    240.0 / 255.0,
)));
pub const TEXT_SLATE_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    203.0 / 255.0,
    213.0 / 255.0,
    225.0 / 255.0,
)));
pub const TEXT_SLATE_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    148.0 / 255.0,
    163.0 / 255.0,
    184.0 / 255.0,
)));
pub const TEXT_SLATE_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    100.0 / 255.0,
    116.0 / 255.0,
    139.0 / 255.0,
)));
pub const TEXT_SLATE_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    71.0 / 255.0,
    85.0 / 255.0,
    105.0 / 255.0,
)));
pub const TEXT_SLATE_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    51.0 / 255.0,
    65.0 / 255.0,
    85.0 / 255.0,
)));
pub const TEXT_SLATE_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    30.0 / 255.0,
    41.0 / 255.0,
    59.0 / 255.0,
)));
pub const TEXT_SLATE_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    15.0 / 255.0,
    23.0 / 255.0,
    42.0 / 255.0,
)));
pub const TEXT_SLATE_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    2.0 / 255.0,
    6.0 / 255.0,
    23.0 / 255.0,
)));
pub const TEXT_GRAY_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    249.0 / 255.0,
    250.0 / 255.0,
    251.0 / 255.0,
)));
pub const TEXT_GRAY_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    243.0 / 255.0,
    244.0 / 255.0,
    246.0 / 255.0,
)));
pub const TEXT_GRAY_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    229.0 / 255.0,
    231.0 / 255.0,
    235.0 / 255.0,
)));
pub const TEXT_GRAY_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    209.0 / 255.0,
    213.0 / 255.0,
    219.0 / 255.0,
)));
pub const TEXT_GRAY_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    156.0 / 255.0,
    163.0 / 255.0,
    175.0 / 255.0,
)));
pub const TEXT_GRAY_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    107.0 / 255.0,
    114.0 / 255.0,
    128.0 / 255.0,
)));
pub const TEXT_GRAY_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    75.0 / 255.0,
    85.0 / 255.0,
    99.0 / 255.0,
)));
pub const TEXT_GRAY_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    55.0 / 255.0,
    65.0 / 255.0,
    81.0 / 255.0,
)));
pub const TEXT_GRAY_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    31.0 / 255.0,
    41.0 / 255.0,
    55.0 / 255.0,
)));
pub const TEXT_GRAY_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    17.0 / 255.0,
    24.0 / 255.0,
    39.0 / 255.0,
)));
pub const TEXT_GRAY_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    3.0 / 255.0,
    7.0 / 255.0,
    18.0 / 255.0,
)));
pub const TEXT_ZINC_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    250.0 / 255.0,
    250.0 / 255.0,
    250.0 / 255.0,
)));
pub const TEXT_ZINC_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    244.0 / 255.0,
    244.0 / 255.0,
    245.0 / 255.0,
)));
pub const TEXT_ZINC_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    228.0 / 255.0,
    228.0 / 255.0,
    231.0 / 255.0,
)));
pub const TEXT_ZINC_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    212.0 / 255.0,
    212.0 / 255.0,
    216.0 / 255.0,
)));
pub const TEXT_ZINC_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    161.0 / 255.0,
    161.0 / 255.0,
    170.0 / 255.0,
)));
pub const TEXT_ZINC_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    113.0 / 255.0,
    113.0 / 255.0,
    122.0 / 255.0,
)));
pub const TEXT_ZINC_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    82.0 / 255.0,
    82.0 / 255.0,
    91.0 / 255.0,
)));
pub const TEXT_ZINC_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    63.0 / 255.0,
    63.0 / 255.0,
    70.0 / 255.0,
)));
pub const TEXT_ZINC_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    39.0 / 255.0,
    39.0 / 255.0,
    42.0 / 255.0,
)));
pub const TEXT_ZINC_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    24.0 / 255.0,
    24.0 / 255.0,
    27.0 / 255.0,
)));
pub const TEXT_ZINC_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    9.0 / 255.0,
    9.0 / 255.0,
    11.0 / 255.0,
)));
pub const TEXT_NEUTRAL_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    250.0 / 255.0,
    250.0 / 255.0,
    250.0 / 255.0,
)));
pub const TEXT_NEUTRAL_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    245.0 / 255.0,
    245.0 / 255.0,
    245.0 / 255.0,
)));
pub const TEXT_NEUTRAL_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    229.0 / 255.0,
    229.0 / 255.0,
    229.0 / 255.0,
)));
pub const TEXT_NEUTRAL_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    212.0 / 255.0,
    212.0 / 255.0,
    212.0 / 255.0,
)));
pub const TEXT_NEUTRAL_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    163.0 / 255.0,
    163.0 / 255.0,
    163.0 / 255.0,
)));
pub const TEXT_NEUTRAL_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    115.0 / 255.0,
    115.0 / 255.0,
    115.0 / 255.0,
)));
pub const TEXT_NEUTRAL_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    82.0 / 255.0,
    82.0 / 255.0,
    82.0 / 255.0,
)));
pub const TEXT_NEUTRAL_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    64.0 / 255.0,
    64.0 / 255.0,
    64.0 / 255.0,
)));
pub const TEXT_NEUTRAL_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    38.0 / 255.0,
    38.0 / 255.0,
    38.0 / 255.0,
)));
pub const TEXT_NEUTRAL_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    23.0 / 255.0,
    23.0 / 255.0,
    23.0 / 255.0,
)));
pub const TEXT_NEUTRAL_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    10.0 / 255.0,
    10.0 / 255.0,
    10.0 / 255.0,
)));
pub const TEXT_STONE_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    250.0 / 255.0,
    250.0 / 255.0,
    249.0 / 255.0,
)));
pub const TEXT_STONE_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    245.0 / 255.0,
    245.0 / 255.0,
    244.0 / 255.0,
)));
pub const TEXT_STONE_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    231.0 / 255.0,
    229.0 / 255.0,
    228.0 / 255.0,
)));
pub const TEXT_STONE_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    214.0 / 255.0,
    211.0 / 255.0,
    209.0 / 255.0,
)));
pub const TEXT_STONE_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    168.0 / 255.0,
    162.0 / 255.0,
    158.0 / 255.0,
)));
pub const TEXT_STONE_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    120.0 / 255.0,
    113.0 / 255.0,
    108.0 / 255.0,
)));
pub const TEXT_STONE_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    87.0 / 255.0,
    83.0 / 255.0,
    78.0 / 255.0,
)));
pub const TEXT_STONE_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    68.0 / 255.0,
    64.0 / 255.0,
    60.0 / 255.0,
)));
pub const TEXT_STONE_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    41.0 / 255.0,
    37.0 / 255.0,
    36.0 / 255.0,
)));
pub const TEXT_STONE_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    28.0 / 255.0,
    25.0 / 255.0,
    23.0 / 255.0,
)));
pub const TEXT_STONE_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    12.0 / 255.0,
    10.0 / 255.0,
    9.0 / 255.0,
)));
pub const TEXT_RED_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    254.0 / 255.0,
    242.0 / 255.0,
    242.0 / 255.0,
)));
pub const TEXT_RED_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    254.0 / 255.0,
    226.0 / 255.0,
    226.0 / 255.0,
)));
pub const TEXT_RED_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    254.0 / 255.0,
    202.0 / 255.0,
    202.0 / 255.0,
)));
pub const TEXT_RED_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    252.0 / 255.0,
    165.0 / 255.0,
    165.0 / 255.0,
)));
pub const TEXT_RED_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    248.0 / 255.0,
    113.0 / 255.0,
    113.0 / 255.0,
)));
pub const TEXT_RED_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    239.0 / 255.0,
    68.0 / 255.0,
    68.0 / 255.0,
)));
pub const TEXT_RED_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    220.0 / 255.0,
    38.0 / 255.0,
    38.0 / 255.0,
)));
pub const TEXT_RED_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    185.0 / 255.0,
    28.0 / 255.0,
    28.0 / 255.0,
)));
pub const TEXT_RED_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    153.0 / 255.0,
    27.0 / 255.0,
    27.0 / 255.0,
)));
pub const TEXT_RED_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    127.0 / 255.0,
    29.0 / 255.0,
    29.0 / 255.0,
)));
pub const TEXT_RED_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    69.0 / 255.0,
    10.0 / 255.0,
    10.0 / 255.0,
)));
pub const TEXT_ORANGE_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    255.0 / 255.0,
    247.0 / 255.0,
    237.0 / 255.0,
)));
pub const TEXT_ORANGE_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    255.0 / 255.0,
    237.0 / 255.0,
    213.0 / 255.0,
)));
pub const TEXT_ORANGE_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    254.0 / 255.0,
    215.0 / 255.0,
    170.0 / 255.0,
)));
pub const TEXT_ORANGE_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    253.0 / 255.0,
    186.0 / 255.0,
    116.0 / 255.0,
)));
pub const TEXT_ORANGE_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    251.0 / 255.0,
    146.0 / 255.0,
    60.0 / 255.0,
)));
pub const TEXT_ORANGE_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    249.0 / 255.0,
    115.0 / 255.0,
    22.0 / 255.0,
)));
pub const TEXT_ORANGE_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    234.0 / 255.0,
    88.0 / 255.0,
    12.0 / 255.0,
)));
pub const TEXT_ORANGE_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    194.0 / 255.0,
    65.0 / 255.0,
    12.0 / 255.0,
)));
pub const TEXT_ORANGE_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    154.0 / 255.0,
    52.0 / 255.0,
    18.0 / 255.0,
)));
pub const TEXT_ORANGE_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    124.0 / 255.0,
    45.0 / 255.0,
    18.0 / 255.0,
)));
pub const TEXT_ORANGE_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    67.0 / 255.0,
    20.0 / 255.0,
    7.0 / 255.0,
)));
pub const TEXT_AMBER_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    255.0 / 255.0,
    251.0 / 255.0,
    235.0 / 255.0,
)));
pub const TEXT_AMBER_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    254.0 / 255.0,
    243.0 / 255.0,
    199.0 / 255.0,
)));
pub const TEXT_AMBER_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    253.0 / 255.0,
    230.0 / 255.0,
    138.0 / 255.0,
)));
pub const TEXT_AMBER_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    252.0 / 255.0,
    211.0 / 255.0,
    77.0 / 255.0,
)));
pub const TEXT_AMBER_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    251.0 / 255.0,
    191.0 / 255.0,
    36.0 / 255.0,
)));
pub const TEXT_AMBER_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    245.0 / 255.0,
    158.0 / 255.0,
    11.0 / 255.0,
)));
pub const TEXT_AMBER_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    217.0 / 255.0,
    119.0 / 255.0,
    6.0 / 255.0,
)));
pub const TEXT_AMBER_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    180.0 / 255.0,
    83.0 / 255.0,
    9.0 / 255.0,
)));
pub const TEXT_AMBER_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    146.0 / 255.0,
    64.0 / 255.0,
    14.0 / 255.0,
)));
pub const TEXT_AMBER_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    120.0 / 255.0,
    53.0 / 255.0,
    15.0 / 255.0,
)));
pub const TEXT_AMBER_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    69.0 / 255.0,
    26.0 / 255.0,
    3.0 / 255.0,
)));
pub const TEXT_YELLOW_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    254.0 / 255.0,
    252.0 / 255.0,
    232.0 / 255.0,
)));
pub const TEXT_YELLOW_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    254.0 / 255.0,
    249.0 / 255.0,
    195.0 / 255.0,
)));
pub const TEXT_YELLOW_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    254.0 / 255.0,
    240.0 / 255.0,
    138.0 / 255.0,
)));
pub const TEXT_YELLOW_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    253.0 / 255.0,
    224.0 / 255.0,
    71.0 / 255.0,
)));
pub const TEXT_YELLOW_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    250.0 / 255.0,
    204.0 / 255.0,
    21.0 / 255.0,
)));
pub const TEXT_YELLOW_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    234.0 / 255.0,
    179.0 / 255.0,
    8.0 / 255.0,
)));
pub const TEXT_YELLOW_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    202.0 / 255.0,
    138.0 / 255.0,
    4.0 / 255.0,
)));
pub const TEXT_YELLOW_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    161.0 / 255.0,
    98.0 / 255.0,
    7.0 / 255.0,
)));
pub const TEXT_YELLOW_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    133.0 / 255.0,
    77.0 / 255.0,
    14.0 / 255.0,
)));
pub const TEXT_YELLOW_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    113.0 / 255.0,
    63.0 / 255.0,
    18.0 / 255.0,
)));
pub const TEXT_YELLOW_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    66.0 / 255.0,
    32.0 / 255.0,
    6.0 / 255.0,
)));
pub const TEXT_LIME_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    247.0 / 255.0,
    254.0 / 255.0,
    231.0 / 255.0,
)));
pub const TEXT_LIME_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    236.0 / 255.0,
    252.0 / 255.0,
    203.0 / 255.0,
)));
pub const TEXT_LIME_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    217.0 / 255.0,
    249.0 / 255.0,
    157.0 / 255.0,
)));
pub const TEXT_LIME_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    190.0 / 255.0,
    242.0 / 255.0,
    100.0 / 255.0,
)));
pub const TEXT_LIME_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    163.0 / 255.0,
    230.0 / 255.0,
    53.0 / 255.0,
)));
pub const TEXT_LIME_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    132.0 / 255.0,
    204.0 / 255.0,
    22.0 / 255.0,
)));
pub const TEXT_LIME_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    101.0 / 255.0,
    163.0 / 255.0,
    13.0 / 255.0,
)));
pub const TEXT_LIME_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    77.0 / 255.0,
    124.0 / 255.0,
    15.0 / 255.0,
)));
pub const TEXT_LIME_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    63.0 / 255.0,
    98.0 / 255.0,
    18.0 / 255.0,
)));
pub const TEXT_LIME_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    54.0 / 255.0,
    83.0 / 255.0,
    20.0 / 255.0,
)));
pub const TEXT_LIME_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    26.0 / 255.0,
    46.0 / 255.0,
    5.0 / 255.0,
)));
pub const TEXT_GREEN_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    240.0 / 255.0,
    253.0 / 255.0,
    244.0 / 255.0,
)));
pub const TEXT_GREEN_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    220.0 / 255.0,
    252.0 / 255.0,
    231.0 / 255.0,
)));
pub const TEXT_GREEN_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    187.0 / 255.0,
    247.0 / 255.0,
    208.0 / 255.0,
)));
pub const TEXT_GREEN_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    134.0 / 255.0,
    239.0 / 255.0,
    172.0 / 255.0,
)));
pub const TEXT_GREEN_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    74.0 / 255.0,
    222.0 / 255.0,
    128.0 / 255.0,
)));
pub const TEXT_GREEN_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    34.0 / 255.0,
    197.0 / 255.0,
    94.0 / 255.0,
)));
pub const TEXT_GREEN_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    22.0 / 255.0,
    163.0 / 255.0,
    74.0 / 255.0,
)));
pub const TEXT_GREEN_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    21.0 / 255.0,
    128.0 / 255.0,
    61.0 / 255.0,
)));
pub const TEXT_GREEN_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    22.0 / 255.0,
    101.0 / 255.0,
    52.0 / 255.0,
)));
pub const TEXT_GREEN_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    20.0 / 255.0,
    83.0 / 255.0,
    45.0 / 255.0,
)));
pub const TEXT_GREEN_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    5.0 / 255.0,
    46.0 / 255.0,
    22.0 / 255.0,
)));
pub const TEXT_EMERALD_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    236.0 / 255.0,
    253.0 / 255.0,
    245.0 / 255.0,
)));
pub const TEXT_EMERALD_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    209.0 / 255.0,
    250.0 / 255.0,
    229.0 / 255.0,
)));
pub const TEXT_EMERALD_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    167.0 / 255.0,
    243.0 / 255.0,
    208.0 / 255.0,
)));
pub const TEXT_EMERALD_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    110.0 / 255.0,
    231.0 / 255.0,
    183.0 / 255.0,
)));
pub const TEXT_EMERALD_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    52.0 / 255.0,
    211.0 / 255.0,
    153.0 / 255.0,
)));
pub const TEXT_EMERALD_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    16.0 / 255.0,
    185.0 / 255.0,
    129.0 / 255.0,
)));
pub const TEXT_EMERALD_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    5.0 / 255.0,
    150.0 / 255.0,
    105.0 / 255.0,
)));
pub const TEXT_EMERALD_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    4.0 / 255.0,
    120.0 / 255.0,
    87.0 / 255.0,
)));
pub const TEXT_EMERALD_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    6.0 / 255.0,
    95.0 / 255.0,
    70.0 / 255.0,
)));
pub const TEXT_EMERALD_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    6.0 / 255.0,
    78.0 / 255.0,
    59.0 / 255.0,
)));
pub const TEXT_EMERALD_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    2.0 / 255.0,
    44.0 / 255.0,
    34.0 / 255.0,
)));
pub const TEXT_TEAL_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    240.0 / 255.0,
    253.0 / 255.0,
    250.0 / 255.0,
)));
pub const TEXT_TEAL_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    204.0 / 255.0,
    251.0 / 255.0,
    241.0 / 255.0,
)));
pub const TEXT_TEAL_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    153.0 / 255.0,
    246.0 / 255.0,
    228.0 / 255.0,
)));
pub const TEXT_TEAL_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    94.0 / 255.0,
    234.0 / 255.0,
    212.0 / 255.0,
)));
pub const TEXT_TEAL_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    45.0 / 255.0,
    212.0 / 255.0,
    191.0 / 255.0,
)));
pub const TEXT_TEAL_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    20.0 / 255.0,
    184.0 / 255.0,
    166.0 / 255.0,
)));
pub const TEXT_TEAL_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    13.0 / 255.0,
    148.0 / 255.0,
    136.0 / 255.0,
)));
pub const TEXT_TEAL_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    15.0 / 255.0,
    118.0 / 255.0,
    110.0 / 255.0,
)));
pub const TEXT_TEAL_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    17.0 / 255.0,
    94.0 / 255.0,
    89.0 / 255.0,
)));
pub const TEXT_TEAL_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    19.0 / 255.0,
    78.0 / 255.0,
    74.0 / 255.0,
)));
pub const TEXT_TEAL_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    4.0 / 255.0,
    47.0 / 255.0,
    46.0 / 255.0,
)));
pub const TEXT_CYAN_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    236.0 / 255.0,
    254.0 / 255.0,
    255.0 / 255.0,
)));
pub const TEXT_CYAN_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    207.0 / 255.0,
    250.0 / 255.0,
    254.0 / 255.0,
)));
pub const TEXT_CYAN_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    165.0 / 255.0,
    243.0 / 255.0,
    252.0 / 255.0,
)));
pub const TEXT_CYAN_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    103.0 / 255.0,
    232.0 / 255.0,
    249.0 / 255.0,
)));
pub const TEXT_CYAN_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    34.0 / 255.0,
    211.0 / 255.0,
    238.0 / 255.0,
)));
pub const TEXT_CYAN_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    6.0 / 255.0,
    182.0 / 255.0,
    212.0 / 255.0,
)));
pub const TEXT_CYAN_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    8.0 / 255.0,
    145.0 / 255.0,
    178.0 / 255.0,
)));
pub const TEXT_CYAN_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    14.0 / 255.0,
    116.0 / 255.0,
    144.0 / 255.0,
)));
pub const TEXT_CYAN_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    21.0 / 255.0,
    94.0 / 255.0,
    117.0 / 255.0,
)));
pub const TEXT_CYAN_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    22.0 / 255.0,
    78.0 / 255.0,
    99.0 / 255.0,
)));
pub const TEXT_CYAN_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    8.0 / 255.0,
    51.0 / 255.0,
    68.0 / 255.0,
)));
pub const TEXT_SKY_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    240.0 / 255.0,
    249.0 / 255.0,
    255.0 / 255.0,
)));
pub const TEXT_SKY_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    224.0 / 255.0,
    242.0 / 255.0,
    254.0 / 255.0,
)));
pub const TEXT_SKY_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    186.0 / 255.0,
    230.0 / 255.0,
    253.0 / 255.0,
)));
pub const TEXT_SKY_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    125.0 / 255.0,
    211.0 / 255.0,
    252.0 / 255.0,
)));
pub const TEXT_SKY_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    56.0 / 255.0,
    189.0 / 255.0,
    248.0 / 255.0,
)));
pub const TEXT_SKY_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    14.0 / 255.0,
    165.0 / 255.0,
    233.0 / 255.0,
)));
pub const TEXT_SKY_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    2.0 / 255.0,
    132.0 / 255.0,
    199.0 / 255.0,
)));
pub const TEXT_SKY_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    3.0 / 255.0,
    105.0 / 255.0,
    161.0 / 255.0,
)));
pub const TEXT_SKY_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    7.0 / 255.0,
    89.0 / 255.0,
    133.0 / 255.0,
)));
pub const TEXT_SKY_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    12.0 / 255.0,
    74.0 / 255.0,
    110.0 / 255.0,
)));
pub const TEXT_SKY_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    8.0 / 255.0,
    47.0 / 255.0,
    73.0 / 255.0,
)));
pub const TEXT_BLUE_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    239.0 / 255.0,
    246.0 / 255.0,
    255.0 / 255.0,
)));
pub const TEXT_BLUE_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    219.0 / 255.0,
    234.0 / 255.0,
    254.0 / 255.0,
)));
pub const TEXT_BLUE_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    191.0 / 255.0,
    219.0 / 255.0,
    254.0 / 255.0,
)));
pub const TEXT_BLUE_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    147.0 / 255.0,
    197.0 / 255.0,
    253.0 / 255.0,
)));
pub const TEXT_BLUE_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    96.0 / 255.0,
    165.0 / 255.0,
    250.0 / 255.0,
)));
pub const TEXT_BLUE_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    59.0 / 255.0,
    130.0 / 255.0,
    246.0 / 255.0,
)));
pub const TEXT_BLUE_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    37.0 / 255.0,
    99.0 / 255.0,
    235.0 / 255.0,
)));
pub const TEXT_BLUE_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    29.0 / 255.0,
    78.0 / 255.0,
    216.0 / 255.0,
)));
pub const TEXT_BLUE_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    30.0 / 255.0,
    64.0 / 255.0,
    175.0 / 255.0,
)));
pub const TEXT_BLUE_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    30.0 / 255.0,
    58.0 / 255.0,
    138.0 / 255.0,
)));
pub const TEXT_BLUE_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    23.0 / 255.0,
    37.0 / 255.0,
    84.0 / 255.0,
)));
pub const TEXT_INDIGO_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    238.0 / 255.0,
    242.0 / 255.0,
    255.0 / 255.0,
)));
pub const TEXT_INDIGO_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    224.0 / 255.0,
    231.0 / 255.0,
    255.0 / 255.0,
)));
pub const TEXT_INDIGO_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    199.0 / 255.0,
    210.0 / 255.0,
    254.0 / 255.0,
)));
pub const TEXT_INDIGO_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    165.0 / 255.0,
    180.0 / 255.0,
    252.0 / 255.0,
)));
pub const TEXT_INDIGO_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    129.0 / 255.0,
    140.0 / 255.0,
    248.0 / 255.0,
)));
pub const TEXT_INDIGO_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    99.0 / 255.0,
    102.0 / 255.0,
    241.0 / 255.0,
)));
pub const TEXT_INDIGO_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    79.0 / 255.0,
    70.0 / 255.0,
    229.0 / 255.0,
)));
pub const TEXT_INDIGO_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    67.0 / 255.0,
    56.0 / 255.0,
    202.0 / 255.0,
)));
pub const TEXT_INDIGO_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    55.0 / 255.0,
    48.0 / 255.0,
    163.0 / 255.0,
)));
pub const TEXT_INDIGO_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    49.0 / 255.0,
    46.0 / 255.0,
    129.0 / 255.0,
)));
pub const TEXT_INDIGO_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    30.0 / 255.0,
    27.0 / 255.0,
    75.0 / 255.0,
)));
pub const TEXT_VIOLET_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    245.0 / 255.0,
    243.0 / 255.0,
    255.0 / 255.0,
)));
pub const TEXT_VIOLET_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    237.0 / 255.0,
    233.0 / 255.0,
    254.0 / 255.0,
)));
pub const TEXT_VIOLET_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    221.0 / 255.0,
    214.0 / 255.0,
    254.0 / 255.0,
)));
pub const TEXT_VIOLET_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    196.0 / 255.0,
    181.0 / 255.0,
    253.0 / 255.0,
)));
pub const TEXT_VIOLET_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    167.0 / 255.0,
    139.0 / 255.0,
    250.0 / 255.0,
)));
pub const TEXT_VIOLET_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    139.0 / 255.0,
    92.0 / 255.0,
    246.0 / 255.0,
)));
pub const TEXT_VIOLET_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    124.0 / 255.0,
    58.0 / 255.0,
    237.0 / 255.0,
)));
pub const TEXT_VIOLET_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    109.0 / 255.0,
    40.0 / 255.0,
    217.0 / 255.0,
)));
pub const TEXT_VIOLET_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    91.0 / 255.0,
    33.0 / 255.0,
    182.0 / 255.0,
)));
pub const TEXT_VIOLET_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    76.0 / 255.0,
    29.0 / 255.0,
    149.0 / 255.0,
)));
pub const TEXT_VIOLET_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    46.0 / 255.0,
    16.0 / 255.0,
    101.0 / 255.0,
)));
pub const TEXT_PURPLE_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    250.0 / 255.0,
    245.0 / 255.0,
    255.0 / 255.0,
)));
pub const TEXT_PURPLE_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    243.0 / 255.0,
    232.0 / 255.0,
    255.0 / 255.0,
)));
pub const TEXT_PURPLE_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    233.0 / 255.0,
    213.0 / 255.0,
    255.0 / 255.0,
)));
pub const TEXT_PURPLE_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    216.0 / 255.0,
    180.0 / 255.0,
    254.0 / 255.0,
)));
pub const TEXT_PURPLE_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    192.0 / 255.0,
    132.0 / 255.0,
    252.0 / 255.0,
)));
pub const TEXT_PURPLE_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    168.0 / 255.0,
    85.0 / 255.0,
    247.0 / 255.0,
)));
pub const TEXT_PURPLE_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    147.0 / 255.0,
    51.0 / 255.0,
    234.0 / 255.0,
)));
pub const TEXT_PURPLE_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    126.0 / 255.0,
    34.0 / 255.0,
    206.0 / 255.0,
)));
pub const TEXT_PURPLE_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    107.0 / 255.0,
    33.0 / 255.0,
    168.0 / 255.0,
)));
pub const TEXT_PURPLE_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    88.0 / 255.0,
    28.0 / 255.0,
    135.0 / 255.0,
)));
pub const TEXT_PURPLE_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    59.0 / 255.0,
    7.0 / 255.0,
    100.0 / 255.0,
)));
pub const TEXT_FUCHSIA_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    253.0 / 255.0,
    244.0 / 255.0,
    255.0 / 255.0,
)));
pub const TEXT_FUCHSIA_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    250.0 / 255.0,
    232.0 / 255.0,
    255.0 / 255.0,
)));
pub const TEXT_FUCHSIA_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    245.0 / 255.0,
    208.0 / 255.0,
    254.0 / 255.0,
)));
pub const TEXT_FUCHSIA_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    240.0 / 255.0,
    171.0 / 255.0,
    252.0 / 255.0,
)));
pub const TEXT_FUCHSIA_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    232.0 / 255.0,
    121.0 / 255.0,
    249.0 / 255.0,
)));
pub const TEXT_FUCHSIA_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    217.0 / 255.0,
    70.0 / 255.0,
    239.0 / 255.0,
)));
pub const TEXT_FUCHSIA_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    192.0 / 255.0,
    38.0 / 255.0,
    211.0 / 255.0,
)));
pub const TEXT_FUCHSIA_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    162.0 / 255.0,
    28.0 / 255.0,
    175.0 / 255.0,
)));
pub const TEXT_FUCHSIA_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    134.0 / 255.0,
    25.0 / 255.0,
    143.0 / 255.0,
)));
pub const TEXT_FUCHSIA_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    112.0 / 255.0,
    26.0 / 255.0,
    117.0 / 255.0,
)));
pub const TEXT_FUCHSIA_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    74.0 / 255.0,
    4.0 / 255.0,
    78.0 / 255.0,
)));
pub const TEXT_PINK_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    253.0 / 255.0,
    242.0 / 255.0,
    248.0 / 255.0,
)));
pub const TEXT_PINK_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    252.0 / 255.0,
    231.0 / 255.0,
    243.0 / 255.0,
)));
pub const TEXT_PINK_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    251.0 / 255.0,
    207.0 / 255.0,
    232.0 / 255.0,
)));
pub const TEXT_PINK_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    249.0 / 255.0,
    168.0 / 255.0,
    212.0 / 255.0,
)));
pub const TEXT_PINK_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    244.0 / 255.0,
    114.0 / 255.0,
    182.0 / 255.0,
)));
pub const TEXT_PINK_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    236.0 / 255.0,
    72.0 / 255.0,
    153.0 / 255.0,
)));
pub const TEXT_PINK_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    219.0 / 255.0,
    39.0 / 255.0,
    119.0 / 255.0,
)));
pub const TEXT_PINK_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    190.0 / 255.0,
    24.0 / 255.0,
    93.0 / 255.0,
)));
pub const TEXT_PINK_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    157.0 / 255.0,
    23.0 / 255.0,
    77.0 / 255.0,
)));
pub const TEXT_PINK_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    131.0 / 255.0,
    24.0 / 255.0,
    67.0 / 255.0,
)));
pub const TEXT_PINK_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    80.0 / 255.0,
    7.0 / 255.0,
    36.0 / 255.0,
)));
pub const TEXT_ROSE_50: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    255.0 / 255.0,
    241.0 / 255.0,
    242.0 / 255.0,
)));
pub const TEXT_ROSE_100: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    255.0 / 255.0,
    228.0 / 255.0,
    230.0 / 255.0,
)));
pub const TEXT_ROSE_200: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    254.0 / 255.0,
    205.0 / 255.0,
    211.0 / 255.0,
)));
pub const TEXT_ROSE_300: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    253.0 / 255.0,
    164.0 / 255.0,
    175.0 / 255.0,
)));
pub const TEXT_ROSE_400: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    251.0 / 255.0,
    113.0 / 255.0,
    133.0 / 255.0,
)));
pub const TEXT_ROSE_500: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    244.0 / 255.0,
    63.0 / 255.0,
    94.0 / 255.0,
)));
pub const TEXT_ROSE_600: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    225.0 / 255.0,
    29.0 / 255.0,
    72.0 / 255.0,
)));
pub const TEXT_ROSE_700: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    190.0 / 255.0,
    18.0 / 255.0,
    60.0 / 255.0,
)));
pub const TEXT_ROSE_800: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    159.0 / 255.0,
    18.0 / 255.0,
    57.0 / 255.0,
)));
pub const TEXT_ROSE_900: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    136.0 / 255.0,
    19.0 / 255.0,
    55.0 / 255.0,
)));
pub const TEXT_ROSE_950: TextClass = TextClass::TextColor(TextColor(Color::srgb(
    76.0 / 255.0,
    5.0 / 255.0,
    25.0 / 255.0,
)));

#[derive(Debug, Clone, Copy)]
pub struct TextColor(Color);

impl ApplyClass for TextColor {
    type Component = Text;

    fn apply_class(&self, component: &mut Self::Component) {
        component.sections[0].style.color = self.0;
    }
}
