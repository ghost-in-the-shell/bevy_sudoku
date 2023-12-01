use bevy::prelude::*;

mod graphics;
mod input;
mod logic;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // resolution: [800., 600.].into(),
                title: "Bevy Sudoku Game".to_string(),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup)
        .add_plugins(logic::board::LogicPlugin)
        .add_plugins(graphics::menu::MenuPlugin)
        .add_plugins(graphics::game::GamePlugin)
        // .add_systems(Startup, graphics::spawn_cameras)
        // .add_plugins(graphics::board::BoardPlugin)
        // .add_plugins(graphics::buttons::BoardButtonsPlugin)
        // .add_plugins(input::InteractionPlugin)
        // .add_plugins(logic::sudoku_generator::GeneratorPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CommonSet {
    Input,
    Action,
}
