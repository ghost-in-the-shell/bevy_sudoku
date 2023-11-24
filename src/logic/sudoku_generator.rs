use crate::input::buttons::{NewPuzzle, ResetPuzzle, SolvePuzzle};
use bevy::prelude::*;
use bevy::utils::HashMap;
use sudoku::Sudoku;

use crate::logic::board::{Cell, Coordinates, FixedValue, Value};

pub struct GeneratorPlugin;

impl Plugin for GeneratorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PuzzleState>()
            .add_systems(Startup, first_sudoku)
            .add_systems(Update, (generate_sudoku, fill_puzzle).chain())
            .add_systems(Update, (reset_sudoku, solve_sudoku));
    }
}

#[derive(Default, Resource)]
struct PuzzleState {
    completed: HashMap<Coordinates, Value>,
    initial: HashMap<Coordinates, Value>,
}

/// Converts a sudoku generated by the `sudoku` crate into a usable format
fn parse_sudoku(sudoku: Sudoku) -> HashMap<Coordinates, Value> {
    let (mut row, mut column) = (1, 0);
    let mut map = HashMap::default();

    // Sudoku::iter() goes from left to right, top to bottom
    for value in sudoku.iter() {
        column += 1;
        if column == 10 {
            row += 1;
            column = 1;
        }
        let square = Coordinates::compute_square(row, column);

        let coordinates = Coordinates {
            row,
            column,
            square,
        };

        let value = match value {
            Some(v) => Value::Filled(v),
            None => Value::Empty,
        };
        map.insert(coordinates, value);
    }
    map
}

/// Sends an event to create a new sudoku on app startup
fn first_sudoku(mut event_writer: EventWriter<NewPuzzle>) {
    event_writer.send(NewPuzzle);
}

/// Create a new sudoku puzzle from sudoku crate
fn generate_sudoku(
    mut new_puzzle_events: EventReader<NewPuzzle>,
    mut puzzle_state: ResMut<PuzzleState>,
) {
    for _ in new_puzzle_events.read() {
        let completed = Sudoku::generate_filled();
        let initial = Sudoku::generate_unique_from(completed);
        puzzle_state.initial = parse_sudoku(initial);
        puzzle_state.completed = parse_sudoku(completed);
    }
}

/// Fills fixed values from the puzzle into the board
fn fill_puzzle(
    puzzle_state: Res<PuzzleState>,
    mut query: Query<(&Coordinates, &mut Value, &mut FixedValue), With<Cell>>,
) {
    // Only run when the puzzle is changed
    if !puzzle_state.is_changed() {
        return;
    }

    for (coordinates, mut value, mut is_fixed) in query.iter_mut() {
        let initial_value = puzzle_state
            .initial
            .get(coordinates)
            .expect("No values found in puzzle for these coordinates");

        // Fill in cells from initial puzzle and mark non-empty cells as fixed
        *value = initial_value.clone();
        is_fixed.0 = !(*initial_value == Value::Empty);
    }
}

/// Resets the puzzle to its original state
fn reset_sudoku(mut event_reader: EventReader<ResetPuzzle>, mut puzzle_state: ResMut<PuzzleState>) {
    for _ in event_reader.read() {
        // Flags the puzzle as having changed, causing the fill_puzzle system to reset all values
        // as if a new identical puzzle had been generated
        // QUALITY: use an explicit set_changed() method instead once added, see https://github.com/bevyengine/bevy/pull/2208
        puzzle_state.set_changed();
    }
}

/// "Solves" the given Sudoku by looking up the solution
fn solve_sudoku(
    mut event_reader: EventReader<SolvePuzzle>,
    mut puzzle_state: ResMut<PuzzleState>,
    mut query: Query<(&Coordinates, &mut Value), With<Cell>>,
) {
    for _ in event_reader.read() {
        for (coordinates, mut value) in query.iter_mut() {
            let correct_value = puzzle_state
                .completed
                .get(coordinates)
                .expect("No values found in puzzle for these coordinates");

            // Fill in cells from initial puzzle and mark those cells as fixed
            *value = correct_value.clone();
        }
    }
}
