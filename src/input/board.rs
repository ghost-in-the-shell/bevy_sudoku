use bevy::prelude::*;

use crate::{graphics::MainCamera, logic::board::Cell};

/// Handle player input from the mouse, converting it into actions
///
/// Input handling from the buttons are found in /graphics/button.rs
use self::cell_index::CellIndex;

/// Event to dispatch cell clicks
#[derive(Event)]
pub struct CellClick {
    /// Some(entity) if a cell was clicked, otherwise None
    pub selected_cell: Option<Entity>,
    /// Should we select multiple cells at once
    pub multi: bool,
    /// Was the mouse dragged
    pub drag: bool,
}

/// Turns raw clicks into `CellClick` events
pub fn cell_click(
    camera_query: Query<&Transform, With<MainCamera>>,
    mouse_button_input: Res<Input<MouseButton>>,
    keyboard_input: Res<Input<KeyCode>>,
    windows: Query<&Window>,
    cell_index: Res<CellIndex>,
    mut cell_click_events: EventWriter<CellClick>,
) {
    if mouse_button_input.pressed(MouseButton::Left) {
        // These coordinates are in terms of the window's coordinates
        // and must be converted to the world coordinates used by our cell
        let window = windows.single();
        let Some(mut cursor_position) = window.cursor_position() else {
            return;
        };
        // QUALITY: use https://github.com/bevyengine/bevy/pull/1799 once merged instead
        let camera_transform = camera_query.get_single().expect("Camera not found.");
        let window_size = Vec2::new(window.width(), window.height());

        // World coordinates are measured from the center
        // while screen coordinates are measures from the bottom left.
        cursor_position -= 0.5 * window_size;

        // Apply the camera's transform to correct for scale, angle etc.
        // Returning a quaternion
        let world_quat =
            camera_transform.compute_matrix() * cursor_position.extend(0.0).extend(1.0);

        let cursor_position_world = Vec2::new(world_quat.x, world_quat.y);

        // Use the CellIndex resource to map the mouse position to a particular cell
        let selected_cell = cell_index.get(cursor_position_world);

        // Send a multi select event when Shift or Control is held
        let multi = keyboard_input.pressed(KeyCode::ShiftLeft)
            || keyboard_input.pressed(KeyCode::ShiftRight)
            || keyboard_input.pressed(KeyCode::ControlLeft)
            || keyboard_input.pressed(KeyCode::ControlRight);

        // Send a drag event when the mouse was not just pressed
        let drag = !mouse_button_input.just_pressed(MouseButton::Left);

        cell_click_events.send(CellClick {
            selected_cell,
            multi,
            drag,
        })
    }
}

pub mod cell_index {
    use bevy::utils::HashMap;

    use super::*;

    /// An index that allows us to look up the entity at the correct position
    #[derive(Default, Resource)]
    pub struct CellIndex {
        pub cell_map: HashMap<Entity, BoundingBox>,
    }

    impl CellIndex {
        pub fn get(&self, position: Vec2) -> Option<Entity> {
            // This is a slow and naive linear-time approach to spatial indexing
            // But it works fine for 81 items!
            for (entity, bounding_box) in self.cell_map.iter() {
                // Checks if the position is in the bounding box on both x and y
                let in_bounds = position.cmpge(bounding_box.bottom_left)
                    & position.cmple(bounding_box.top_right);
                // Only returns true if it's inside the box on both x and y
                if in_bounds.all() {
                    // This early return of a single item only works correctly
                    // because we know our entitities never overlap
                    // We would need a way to break ties otherwise
                    return Some(*entity);
                }
            }
            // Return None if no matches found
            None
        }
    }

    /// Builds a `CellIndex` for cells whose `Transform` has been changed
    pub fn index_cells(
        query: Query<(Entity, &Sprite, &Transform), (With<Cell>, Changed<Transform>)>,
        mut cell_index: ResMut<CellIndex>,
    ) {
        // Our Changed<Transform> filter ensures that this system only does work
        // on entities whose Transforms were added or mutated since the last time
        // this system ran
        for (entity, sprite, transform) in query.iter() {
            let center = transform.translation.truncate();
            let bottom_left = center - sprite.custom_size.unwrap_or_default() / 2.0;
            let top_right = center + sprite.custom_size.unwrap_or_default() / 2.0;

            // .insert overwrites existing values
            cell_index.cell_map.insert(
                entity,
                BoundingBox {
                    bottom_left,
                    top_right,
                },
            );
        }
    }

    /// The axis-aligned rectangle that contains our cells
    pub struct BoundingBox {
        pub bottom_left: Vec2,
        pub top_right: Vec2,
    }
}
