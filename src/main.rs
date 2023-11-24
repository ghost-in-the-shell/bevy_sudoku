use bevy::prelude::*;

mod graphics;
mod input;
mod logic;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(input::InteractionPlugin)
        .add_plugins(logic::board::LogicPlugin)
        .add_plugins(logic::sudoku_generator::GeneratorPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup() {}
