use bevy::ecs::system::Commands;
use bevy::prelude::{App, Camera2dBundle, Component, Startup};
use bevy::DefaultPlugins;
use bevy_bsml::class::background_color::{BG_BLUE_400, BG_BLUE_600, BG_WHITE};
use bevy_bsml::class::styles::flexbox_grid::align_items::ITEMS_CENTER;
use bevy_bsml::class::styles::flexbox_grid::flex_direction::FLEX_COL;
use bevy_bsml::class::styles::flexbox_grid::gap::gap_y;
use bevy_bsml::class::styles::sizing::{h_px, h_vh, w_px, w_vw};
use bevy_bsml::class::text::TEXT_BASE;
use bevy_bsml::class::{
    hovered, pressed,
    styles::{
        flexbox_grid::justify_content::JUSTIFY_CENTER,
        sizing::{H_FULL, W_FULL},
    },
};
use bevy_bsml::{bsml, BsmlPlugin, SpawnBsml};

#[derive(Debug, Clone, Component)]
pub struct Menu;

bsml! {Menu;
    (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_WHITE]) {
        (node class=[FLEX_COL, gap_y(20.0)]) {
            (MenuItem i={0} name={"Continue".to_owned()})
            (MenuItem i={1} name={"Setting".to_owned()})
            (MenuItem i={2} name={"Exit".to_owned()})
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct MenuItem {
    pub i: u8,
    pub name: String,
}

bsml! {MenuItem;
    (node
        class=[w_px(200.0), h_px(75.0), BG_BLUE_400, hovered(BG_BLUE_600), pressed(BG_BLUE_600 / 0.5), JUSTIFY_CENTER, ITEMS_CENTER]
    ) {
        (text class=[TEXT_BASE]) { "{}: {}", i, name }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn_bsml(Menu);
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BsmlPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_ui)
        .run();
}
