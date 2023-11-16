use crate::logic::sudoku_generator::GeneratorPlugin;
use bevy::prelude::*;

mod logic;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(GeneratorPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup() {}
