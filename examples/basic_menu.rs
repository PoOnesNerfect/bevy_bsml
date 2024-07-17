use bevy::prelude::*;
use bevy_bsml::prelude::*;

#[derive(Debug, Clone, Component)]
pub struct MenuScreen;

bsml! {MenuScreen;
    (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_TRANSPARENT]) {
        (for
            {name in ["Continue", "Change Color", "Setting", "Exit"]}
            class=[FLEX_COL, gap_y(20.0)]
        ) {
            (MenuItem { name, width: 200.0 })
        }
    }
}

#[derive(Debug, Clone, Component)]
pub struct MenuItem {
    pub name: &'static str,
    pub width: f32,
}

bsml! {MenuItem;
    (slot
        class=[
            w_px(self.width), h_px(75.0), BG_BLUE_400, hovered(BG_BLUE_600), pressed(BG_BLUE_800),
            JUSTIFY_CENTER, ITEMS_CENTER
        ]
    ) {
        (text class=[TEXT_BASE]) { "{}", self.name }
    }
}

fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

// handle interactions with menu items
fn menu_item_system(
    query: Query<(Entity, &Interaction, &MenuItem), Changed<Interaction>>,
    mut classes: Query<&mut BackgroundColorClassList>,
    mut exit: EventWriter<AppExit>,
) {
    for (entity, interaction, item) in query.iter() {
        if *interaction == Interaction::Pressed {
            println!("Pressed: {}", item.name);

            if item.name == "Exit" {
                println!("exiting game...");
                exit.send(AppExit::Success);
                return;
            } else if item.name == "Change Color" {
                println!("changing color...");
                let mut classes = classes.get_mut(entity).unwrap();
                classes.set(Interaction::Pressed, BG_RED_700);
                classes.set(Interaction::Hovered, BG_RED_600);
                classes.set(Interaction::None, BG_RED_400);
                return;
            }
        }
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn_bsml(MenuScreen);
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
        .add_systems(Update, close_on_esc)
        .run();
}
