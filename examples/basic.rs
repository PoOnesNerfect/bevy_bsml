use bevy::ecs::system::Commands;
use bevy::prelude::{
    default, App, BuildChildren, ButtonBundle, Camera2dBundle, Color, NodeBundle, Startup,
    TextBundle,
};
use bevy::text::TextStyle;
use bevy::ui::{AlignItems, BackgroundColor, BorderColor, JustifyContent, Style, UiRect, Val};
use bevy::DefaultPlugins;
use bevy_bsml::class::text::FontSize;
use bevy_bsml::class::{
    hovered, pressed,
    styles::{
        flexbox_grid::JUSTIFY_CENTER,
        sizing::{Height, Width, H_FULL, W_FULL},
    },
};
use bevy_bsml::{bsml, BsmlCommand, BsmlPlugin};

bsml! {
    (Menu class=[W_FULL, H_FULL, JUSTIFY_CENTER, BackgroundColor::DEFAULT, hovered(BackgroundColor::DEFAULT), pressed(BackgroundColor(Color::rgb(0.75, 0.75, 0.75)))]) {
        (MenuItem i={0} name={"Continue".to_owned()})
        (MenuItem i={1} name={"Setting".to_owned()})
        (MenuItem i={2} name={"Exit".to_owned()})
    }
}

bsml! {
    (MenuItem
        {i: u8}
        {name: String}
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
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(BsmlPlugin)
        .add_systems(Startup, (setup, spawn_ui))
        .run();
}

fn _spawn_box(mut commands: Commands) {
    let container_node = NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        ..default()
    };

    let button_node = ButtonBundle {
        style: Style {
            width: Val::Px(150.0),
            height: Val::Px(65.0),
            border: UiRect::all(Val::Px(5.0)),
            // horizontally center child text
            justify_content: JustifyContent::Center,
            // vertically center child text
            align_items: AlignItems::Center,
            ..default()
        },
        border_color: BorderColor(Color::BLACK),
        ..default()
    };

    let button_text_node = TextBundle::from_section(
        "Button",
        TextStyle {
            font_size: 40.0,
            color: Color::rgb(0.9, 0.9, 0.9),
            ..default()
        },
    );

    let container = commands.spawn(container_node).id();
    let button = commands.spawn(button_node).id();
    let button_text = commands.spawn(button_text_node).id();

    commands.entity(button).push_children(&[button_text]);
    commands.entity(container).push_children(&[button]);
}
