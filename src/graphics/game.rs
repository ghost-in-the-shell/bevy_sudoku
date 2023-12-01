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
            builder
                .spawn((
                    NodeBundle {
                        style: Style {
                            height: Val::Percent(100.0),
                            aspect_ratio: Some(1.0),
                            display: Display::Grid,
                            // grid_column: GridPlacement::span(1),
                            padding: UiRect::all(Val::Px(12.0)),
                            grid_template_columns: RepeatedGridTrack::flex(9, 1.0),
                            grid_template_rows: RepeatedGridTrack::flex(9, 1.0),
                            // row_gap: Val::Px(1.0),
                            // column_gap: Val::Px(1.0),
                            ..default()
                        },
                        background_color: BackgroundColor(Color::DARK_GRAY),
                        ..default()
                    },
                    GameGrid::Board,
                ))
                .with_children(|builder| {
                    for row in 1..=9 {
                        for column in 1..=9 {
                            board::setup::spawn_cell(builder, row, column)
                        }
                    }
                });

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
    Board,
    Right,
}

// Tag component used to tag entities added on the game screen
#[derive(Component)]
struct OnGameScreen;
