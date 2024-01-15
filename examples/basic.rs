use bevy::ecs::system::Commands;
use bevy::prelude::{App, Camera2dBundle, Color, Component, Startup};
use bevy::ui::BackgroundColor;
use bevy::DefaultPlugins;
use bevy_bsml::class::text::FontSize;
use bevy_bsml::class::{
    hovered, pressed,
    styles::{
        flexbox_grid::JUSTIFY_CENTER,
        sizing::{Height, Width, H_FULL, W_FULL},
    },
};
use bevy_bsml::{bsml, BsmlPlugin, SpawnBsml};

#[derive(Debug, Clone, Component)]
pub struct Menu;

bsml! {Menu;
    (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, BackgroundColor::DEFAULT, hovered(BackgroundColor::DEFAULT), pressed(BackgroundColor(Color::rgb(0.75, 0.75, 0.75)))]) {
        (MenuItem i={0} name={"Continue".to_owned()}) { (text) { Hello, world! } }
        (MenuItem i={1} name={"Setting".to_owned()})
        (MenuItem i={2} name={"Exit".to_owned()})
    }
}

#[derive(Debug, Clone, Component)]
pub struct MenuItem {
    pub i: u8,
    pub name: String,
}

bsml! {MenuItem;
    (slot
        class=[Width::px(100.0), Height::px(100.0), BackgroundColor(Color::rgb(0.65, 0.65, 0.65)), hovered(BackgroundColor(Color::rgb(0.75, 0.75, 0.75))), pressed(BackgroundColor::DEFAULT)]
    ) {
        (text class=[FontSize::px(12.0), hovered(FontSize::px(24.0))]) { {i}: {name} }
    }
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn_bsml(Menu);
    commands.spawn_bsml(bsml!((MenuItem i={0} name={"Continue".to_owned()})));
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BsmlPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_ui)
        .run();
}
