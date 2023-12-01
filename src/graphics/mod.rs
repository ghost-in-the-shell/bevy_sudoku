use bevy::prelude::*;

pub mod game;
pub mod menu;

pub const BACKGROUND_COLOR: Color = Color::rgb(1.0, 1.0, 1.0);

/// Marker component for game camera
#[derive(Component)]
pub struct MainCamera;

/// Marker component for UI camera
#[derive(Component)]
pub struct UiCamera;

// Generic system that takes a component as a parameter, and will despawn all entities with that component
fn despawn_screen<T: Component>(to_despawn: Query<Entity, With<T>>, mut commands: Commands) {
    for entity in &to_despawn {
        commands.entity(entity).despawn_recursive();
    }
}
