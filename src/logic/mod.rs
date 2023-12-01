use bevy::prelude::States;

pub mod board;
pub mod sudoku_generator;

/// Enum for GameState
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
pub enum GameState {
    #[default]
    Menu,
    Game,
}
