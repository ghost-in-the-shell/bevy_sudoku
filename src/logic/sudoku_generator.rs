use bevy::prelude::*;
use sudoku::Sudoku;

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generate_sudoku);
    }
}

fn generate_sudoku() {
    let completed = Sudoku::generate_filled();
    println!("Completed: {:?}", completed);
    let inital = Sudoku::generate_unique_from(completed);
    println!("Inital: {:?}", inital);
}
