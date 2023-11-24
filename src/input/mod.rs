use crate::CommonSet;
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
            .init_resource::<input_mode::InputMode>()
            // BOARD
            .add_systems(PreUpdate, board::cell_index::index_cells)
            // BUTTONS
            .add_systems(
                Update,
                (
                    board::cell_click,
                    buttons::puzzle_button::<buttons::NewPuzzle>,
                    buttons::puzzle_button::<buttons::ResetPuzzle>,
                    buttons::puzzle_button::<buttons::SolvePuzzle>,
                    buttons::puzzle_button::<CellInput>,
                    buttons::input_mode_buttons,
                )
                    .in_set(CommonSet::Input),
            )
            // KEYBOARD
            .add_systems(
                Update,
                (
                    keyboard::select_all,
                    keyboard::cell_input::cell_keyboard_input,
                    keyboard::erase_selected_cells,
                    keyboard::swap_input_mode,
                )
                    .in_set(CommonSet::Input),
            );
    }
}

/// Marker component for selected cells
#[derive(Debug, Component)]
pub struct Selected;

/// Events that change the value stored in a cell
#[derive(Clone, Event, Component)]
pub struct CellInput {
    pub num: u8,
}
