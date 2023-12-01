use crate::graphics::despawn_screen;
use crate::logic::GameState;
use bevy::prelude::*;

mod board;
mod buttons;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(board::BoardPlugin)
            .add_systems(OnEnter(GameState::Game), game_setup)
            .add_systems(OnExit(GameState::Game), despawn_screen::<OnGameScreen>);
    }
}

fn game_setup(mut commands: Commands) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                display: Display::Grid,
                grid_template_columns: vec![GridTrack::min_content(), GridTrack::flex(1.0)],
                grid_template_rows: vec![GridTrack::flex(1.0)],
                ..default()
            },
            background_color: BackgroundColor(Color::WHITE),
            ..default()
        })
        .with_children(|builder| {
            builder.spawn((
                NodeBundle {
                    style: Style {
                        height: Val::Percent(100.0),
                        aspect_ratio: Some(1.0),
                        display: Display::Grid,
                        grid_column: GridPlacement::span(1),
                        ..default()
                    },

                    ..default()
                },
                GameGrid::Left,
            ));

            builder.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Grid,
                        grid_column: GridPlacement::span(1),
                        ..default()
                    },

                    ..default()
                },
                GameGrid::Right,
            ));
        });
}

#[derive(Component)]
pub enum GameGrid {
    Left,
    Right,
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;
