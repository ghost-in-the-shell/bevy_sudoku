use bevy::prelude::*;

use crate::graphics::game::game_setup;
use crate::logic::GameState;
use crate::{
    input::Selected,
    logic::board::{Cell, Coordinates, FixedValue, Value},
};

use self::assets::*;
use self::config::*;

pub struct BoardPlugin;

impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<FixedFont>()
            .init_resource::<FillableFont>()
            .init_resource::<BoardBackgroundColor>()
            .init_resource::<SelectionColor>()
            // .add_systems(OnEnter(GameState::Game), setup::spawn_cells.after(game_setup))
            // .add_systems(Startup, (setup::spawn_grid, setup::spawn_cell_numbers))
            // .add_systems(
            //     Update,
            //     (
            //         actions::update_cell_numbers,
            //         actions::color_selected,
            //         actions::style_numbers,
            //     )
            //         .in_set(CommonSet::Action),
            // );
        ;
    }
}

pub(crate) fn spawn_board_layout(parent: &mut ChildBuilder, row: u8, column: u8) {
    parent.spawn(NodeBundle {
        style: Style {
            height: Val::Percent(100.0),
            width: Val::Percent(100.0),
            display: Display::Grid,

            ..default()
        },
        ..default()
    });
}

mod config {
    use super::*;

    // Colors
    pub const SELECTION_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

    pub const GRID_COLOR: Color = Color::rgb(0.1, 0.1, 0.1);
    pub const NUMBER_COLOR: Color = Color::BLACK;

    // Fonts
    pub const FIXED_NUM_FONT: &str = "fonts/Ubuntu-Bold.ttf";
    pub const FILLABLE_NUM_FONT: &str = "fonts/Ubuntu-Light.ttf";

    // Sizes
    pub const CELL_SIZE: f32 = 50.0;
    pub const GRID_SIZE: f32 = 9.0 * CELL_SIZE;
    pub const MINOR_LINE_THICKNESS: f32 = 2.0;
    pub const MAJOR_LINE_THICKNESS: f32 = 4.0;

    // Positions
    // Defines the center lines of the grid in absolute coordinates
    // (0, 0) is in the center of the screen in Bevy
    pub const GRID_CENTER_X: f32 = -300.0;
    pub const GRID_LEFT_EDGE: f32 = GRID_CENTER_X - 0.5 * GRID_SIZE;
    pub const GRID_CENTER_Y: f32 = 0.0;
    pub const GRID_BOT_EDGE: f32 = GRID_CENTER_Y - 0.5 * GRID_SIZE;

    pub const NUM_OFFSET_X: f32 = 0.0 * CELL_SIZE;
    pub const NUM_OFFSET_Y: f32 = 0.03 * CELL_SIZE;
}

// QUALITY: reduce asset loading code duplication dramatically
pub mod assets {
    use crate::graphics::BACKGROUND_COLOR;

    use super::*;

    // Various colors for our cells
    /// The color of the game's background, and the default color of the cells
    #[derive(Resource)]
    pub struct BoardBackgroundColor(pub Handle<ColorMaterial>);
    /// The color of cells when selected
    #[derive(Resource)]
    pub struct SelectionColor(pub Handle<ColorMaterial>);

    impl FromWorld for BoardBackgroundColor {
        fn from_world(world: &mut World) -> Self {
            let mut materials = world
                .get_resource_mut::<Assets<ColorMaterial>>()
                .expect("ResMut<Assets<ColorMaterial>> not found.");
            BoardBackgroundColor(materials.add(BACKGROUND_COLOR.into()))
        }
    }

    impl FromWorld for SelectionColor {
        fn from_world(world: &mut World) -> Self {
            let mut materials = world
                .get_resource_mut::<Assets<ColorMaterial>>()
                .expect("ResMut<Assets<ColorMaterial>> not found.");
            SelectionColor(materials.add(SELECTION_COLOR.into()))
        }
    }

    // Fonts used in our game
    #[derive(Resource)]
    pub struct FixedFont(pub Handle<Font>);

    impl FromWorld for FixedFont {
        fn from_world(world: &mut World) -> Self {
            let asset_server = world
                .get_resource_mut::<AssetServer>()
                .expect("ResMut<AssetServer> not found.");
            FixedFont(asset_server.load(FIXED_NUM_FONT))
        }
    }

    #[derive(Resource)]
    pub struct FillableFont(pub Handle<Font>);

    impl FromWorld for FillableFont {
        fn from_world(world: &mut World) -> Self {
            let asset_server = world
                .get_resource_mut::<AssetServer>()
                .expect("ResMut<AssetServer> not found.");
            FillableFont(asset_server.load(FILLABLE_NUM_FONT))
        }
    }
}

pub(crate) mod setup {
    use bevy::sprite::MaterialMesh2dBundle;

    use super::*;

    pub fn spawn_grid(
        mut commands: Commands,
        mut materials: ResMut<Assets<ColorMaterial>>,
        mut meshes: ResMut<Assets<Mesh>>,
    ) {
        let grid_handle = materials.add(GRID_COLOR.into());

        for row in 0..=9 {
            commands.spawn(new_gridline(
                Orientation::Horizontal,
                row,
                grid_handle.clone(),
                &mut meshes,
            ));
        }

        for column in 0..=9 {
            commands.spawn(new_gridline(
                Orientation::Vertical,
                column,
                grid_handle.clone(),
                &mut meshes,
            ));
        }
    }

    enum Orientation {
        Horizontal,
        Vertical,
    }

    fn new_gridline(
        orientation: Orientation,
        i: u8,
        grid_handle: Handle<ColorMaterial>,
        meshes: &mut ResMut<Assets<Mesh>>,
    ) -> MaterialMesh2dBundle<ColorMaterial> {
        // The grid lines that define the boxes need to be thicker
        let thickness = if (i % 3) == 0 {
            MAJOR_LINE_THICKNESS
        } else {
            MINOR_LINE_THICKNESS
        };

        let length = GRID_SIZE + thickness;

        let size = match orientation {
            Orientation::Horizontal => Vec2::new(length, thickness),
            Orientation::Vertical => Vec2::new(thickness, length),
        };

        // Each objects' position is defined by its center
        let offset = i as f32 * CELL_SIZE;

        let (x, y) = match orientation {
            Orientation::Horizontal => (GRID_LEFT_EDGE + 0.5 * GRID_SIZE, GRID_BOT_EDGE + offset),
            Orientation::Vertical => (GRID_LEFT_EDGE + offset, GRID_BOT_EDGE + 0.5 * GRID_SIZE),
        };

        MaterialMesh2dBundle {
            mesh: meshes.add(Mesh::from(shape::Quad::new(size))).into(),
            // We want these grid lines to cover any cell that it might overlap with
            transform: Transform::from_xyz(x, y, 1.0),
            material: grid_handle,
            ..Default::default()
        }
    }

    pub fn spawn_cells(mut commands: Commands) {
        for row in 1..=9 {
            for column in 1..=9 {
                commands.spawn(CellBundle::new(row, column));
            }
        }
    }

    pub fn spawn_cell(builder: &mut ChildBuilder, row: u8, column: u8) {
        builder
            .spawn(NodeBundle {
                style: Style {
                    display: Display::Grid,
                    padding: UiRect::all(Val::Px(1.0)),
                    ..default()
                },
                // background_color: BackgroundColor(Color::BLACK),
                ..default()
            })
            .with_children(|builder| {
                builder.spawn(NodeBundle {
                    background_color: BackgroundColor(Color::BEIGE),
                    ..default()
                });
            });
    }

    #[derive(Bundle)]
    pub(crate) struct CellBundle {
        cell: Cell,
        coordinates: Coordinates,
        value: Value,
        fixed: FixedValue,
        cell_fill: SpriteBundle,
    }

    impl CellBundle {
        fn new(row: u8, column: u8) -> Self {
            let x = GRID_LEFT_EDGE + CELL_SIZE * row as f32 - 0.5 * CELL_SIZE;
            let y = GRID_BOT_EDGE + CELL_SIZE * column as f32 - 0.5 * CELL_SIZE;

            CellBundle {
                cell: Cell,
                coordinates: Coordinates {
                    row,
                    column,
                    square: Coordinates::compute_square(row, column),
                },
                // No digits are filled in to begin with
                value: Value::Empty,
                fixed: FixedValue(false),
                cell_fill: SpriteBundle {
                    // The material for this sprite begins with the same material as our background
                    sprite: Sprite {
                        custom_size: Some(Vec2::new(CELL_SIZE, CELL_SIZE)),
                        ..Default::default()
                    },
                    // We want this cell to be covered by any grid lines that it might overlap with
                    transform: Transform::from_xyz(x, y, 0.0),
                    ..Default::default()
                },
            }
        }
    }

    /// Marker component for the visual representation of a cell's values
    #[derive(Component)]
    pub struct CellNumber;

    // Marker relation to designate that the Value on the source entity (the Cell entity)
    // is displayed by the target entity (the Text2d entity in the same location)
    #[derive(Component)]
    pub struct DisplayedBy {
        pub cell: Entity,
        pub text: Entity,
    }

    /// Adds a text number associated with each cell to display its value
    pub fn spawn_cell_numbers(
        query: Query<(Entity, &Transform), With<Cell>>,
        mut commands: Commands,
        font_res: Res<FixedFont>,
    ) {
        const TEXT_ALIGNMENT: TextAlignment = TextAlignment::Center;

        for (cell_entity, cell_transform) in query.iter() {
            let mut number_transform = cell_transform.clone();

            // Tweaks for aesthetic perfection
            number_transform.translation.x += NUM_OFFSET_X;
            number_transform.translation.y += NUM_OFFSET_Y;

            // These numbers must be displayed on top of the cells they are in
            number_transform.translation.z += 1.0;

            let text_style = TextStyle {
                font: font_res.0.clone(),
                font_size: 0.8 * CELL_SIZE,
                color: NUMBER_COLOR,
            };

            let text_entity = commands
                .spawn(Text2dBundle {
                    // This value begins empty, but then is later set in update_cell_numbers system
                    // to match the cell's `value` field
                    text: Text::from_section("", text_style.clone()).with_alignment(TEXT_ALIGNMENT),
                    transform: number_transform,
                    ..Default::default()
                })
                .insert(CellNumber)
                .id();

            commands.entity(cell_entity).insert(DisplayedBy {
                cell: cell_entity,
                text: text_entity,
            });
            // .insert_relation(DisplayedBy, text_entity);
        }
    }
}

mod actions {
    use super::setup::DisplayedBy;
    use super::*;

    /// Changes the cell displays to match their values
    pub fn update_cell_numbers(
        cell_query: Query<(&Value, &DisplayedBy), (With<Cell>, Changed<Value>)>,
        mut num_query: Query<&mut Text>,
    ) {
        use Value::*;
        for (cell_value, displayed_by) in cell_query.iter() {
            let mut text = num_query
                .get_mut(displayed_by.text)
                .expect("No corresponding entity found!");

            // There is only one section in our text
            text.sections[0].value = match cell_value.clone() {
                Filled(n) => n.to_string(),
                // TODO: properly display markings
                Marked(center, corner) => {
                    format!("Center: {}", center.to_string())
                        + "|"
                        + &format!("Corner: {}", corner.to_string())
                }
                Empty => "".to_string(),
            }
        }
    }

    /// Set the background color of selected cells
    pub fn color_selected(
        mut query: Query<(Option<&Selected>, &mut Handle<ColorMaterial>), With<Cell>>,
        background_color: Res<BoardBackgroundColor>,
        selection_color: Res<SelectionColor>,
    ) {
        // QUALITY: use Added and Removed queries to avoid excessive spinning
        // once https://github.com/bevyengine/bevy/issues/2148 is fixed
        for (maybe_selected, mut material_handle) in query.iter_mut() {
            match maybe_selected {
                Some(_) => *material_handle = selection_color.0.clone(),
                None => *material_handle = background_color.0.clone(),
            }
        }
    }

    /// Sets the style of the numbers based on whether or not they're fixed
    pub fn style_numbers(
        cell_query: Query<(&FixedValue, &DisplayedBy), Changed<FixedValue>>,
        mut text_query: Query<&mut Text>,
        fixed_font_res: Res<FixedFont>,
        fillable_font_res: Res<FillableFont>,
    ) {
        for (is_fixed, displayed_by) in cell_query.iter() {
            let mut text = text_query
                .get_mut(displayed_by.text)
                .expect("Corresponding text entity not found.");
            text.sections[0].style.font = match is_fixed.0 {
                true => fixed_font_res.0.clone(),
                false => fillable_font_res.0.clone(),
            }
        }
    }
}
