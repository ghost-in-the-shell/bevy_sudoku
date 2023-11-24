use bevy::prelude::*;

mod graphics;
mod input;
mod logic;

fn main() {
    App::new()
        .insert_resource(ClearColor(graphics::BACKGROUND_COLOR))
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, graphics::spawn_cameras)
        .add_plugins(graphics::board::BoardPlugin)
        .add_plugins(graphics::buttons::BoardButtonsPlugin)
        .add_plugins(input::InteractionPlugin)
        .add_plugins(logic::board::LogicPlugin)
        .add_plugins(logic::sudoku_generator::GeneratorPlugin)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum CommonSet {
    Input,
    Action,
}
