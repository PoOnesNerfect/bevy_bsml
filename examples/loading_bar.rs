use bevy::ecs::system::Commands;
use bevy::prelude::{App, Camera2dBundle, Component, Query, Res, Startup, Update};
use bevy::time::{Time, Timer, TimerMode};
use bevy::ui::Interaction;
use bevy::DefaultPlugins;
use bevy_bsml::class::StyleClass;
use bevy_bsml::prelude::*;
use std::time::Duration;

// spawns a menu with items in the middle of the screen
#[derive(Debug, Clone, Component)]
pub struct Screen;

bsml! {Screen;
    (node class=[W_FULL, H_FULL, JUSTIFY_CENTER, ITEMS_CENTER, BG_TRANSPARENT]) {
        (LoadingBar)
    }
}

#[derive(Debug, Clone, Default, Component)]
pub struct LoadingBar;
bsml! {LoadingBar;
    (node
        class=[w_px(300.0), h_px(30.0), BG_SLATE_400]
    ) {
        (Loaded timer={Timer::new(Duration::from_secs(5), TimerMode::Once)})
    }
}

#[derive(Debug, Clone, Default, Component)]
pub struct Loaded {
    pub timer: Timer,
}

bsml! {Loaded; (node class=[w_perc(0.0), H_FULL, BG_BLUE_400]) }

fn loading_bar_system(
    mut query: Query<(&mut Loaded, &mut ClassList<StyleClass>)>,
    time: Res<Time>,
) {
    let (mut loaded, mut classes) = query.single_mut();
    loaded.timer.tick(time.delta());

    if loaded.timer.just_finished() {
        classes.set(Interaction::None, w_perc(100.0));
    } else if !loaded.timer.finished() {
        classes.set(Interaction::None, w_perc(loaded.timer.percent() * 100.0));
    }
}

fn spawn_ui(mut commands: Commands) {
    commands.spawn_bsml(Screen);
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
        .add_systems(Update, loading_bar_system)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
