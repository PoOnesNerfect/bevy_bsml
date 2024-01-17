use bevy::app::AppExit;
use bevy::ecs::system::Commands;
use bevy::prelude::{App, Camera2dBundle, Changed, Component, EventWriter, Query, Startup, Update};
use bevy::ui::Interaction;
use bevy::DefaultPlugins;
use bevy_bsml::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct Menu {
    pub items: &'static [&'static str],
}

bsml! {Menu;
    (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_TRANSPARENT]) {
        (for {i, name in self.items} class=[FLEX_COL, gap_y(20.0)]) {
            (MenuItem i name ..default)
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct MenuItem {
    pub i: usize,
    pub name: &'static str,
    pub width: f32,
}

impl Default for MenuItem {
    fn default() -> Self {
        Self {
            i: 0,
            name: "Menu Item",
            width: 200.0,
        }
    }
}

bsml! {MenuItem;
    (slot
        class=[w_px(self.width), h_px(75.0), BG_BLUE_400, hovered(BG_BLUE_600), pressed(BG_BLUE_800), JUSTIFY_CENTER, ITEMS_CENTER]
    ) {
        (text class=[TEXT_BASE]) { "{}", self.name }
    }
}

fn menu_item_system(
    query: Query<(&Interaction, &MenuItem), Changed<Interaction>>,
    mut exit: EventWriter<AppExit>,
) {
    for (interaction, item) in query.iter() {
        if *interaction == Interaction::Pressed {
            println!("Pressed: {}", item.name);

            if item.name == "Exit" {
                println!("exiting game...");
                exit.send(AppExit);
            }
        }
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn_bsml(Menu {
        items: &["Continue", "Setting", "Exit"],
    });
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BsmlPlugin)
        .add_systems(Startup, setup)
        .add_systems(Startup, spawn_ui)
        .add_systems(Update, menu_item_system)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
