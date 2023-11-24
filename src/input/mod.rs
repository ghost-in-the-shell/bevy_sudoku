use bevy::prelude::*;

pub mod board;
pub mod buttons;
pub mod input_mode;

mod keyboard;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<buttons::NewPuzzle>()
            .add_event::<buttons::ResetPuzzle>()
            .add_event::<buttons::SolvePuzzle>()
            .add_event::<board::CellClick>()
            .add_event::<CellInput>()
            .init_resource::<keyboard::cell_input::CellInputMap>()
            .init_resource::<board::cell_index::CellIndex>()
            .init_resource::<input_mode::InputMode>();
    }
}

/// Marker component for selected cells
#[derive(Debug, Component)]
pub struct Selected;

/// Events that change the value stored in a cell
#[derive(Clone, Event)]
pub struct CellInput {
    pub num: u8,
}
