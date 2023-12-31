/// Build and display the UI buttons
use super::board::assets::FixedFont;
use crate::input::buttons::{NewPuzzle, ResetPuzzle, SolvePuzzle};
use crate::input::{input_mode::InputMode, CellInput};
use bevy::render::render_resource::{AsBindGroup, ShaderRef};
use bevy::{ecs::component::Component, prelude::*};
use std::marker::PhantomData;

use self::assets::*;
use self::config::*;

pub struct BoardButtonsPlugin;

// QUALITY: use system sets for clarity
impl Plugin for BoardButtonsPlugin {
    fn build(&self, app: &mut App) {
        app
            // ASSETS
            .init_resource::<ButtonMaterials<NewPuzzle>>()
            .init_resource::<ButtonMaterials<ResetPuzzle>>()
            .init_resource::<ButtonMaterials<SolvePuzzle>>()
            .init_resource::<ButtonMaterials<InputMode>>()
            .init_resource::<ButtonMaterials<CellInput>>()
            .add_plugins(UiMaterialPlugin::<CustomUiMaterial>::default())
            .init_resource::<NoneColor>()
            // SETUP
            // Must be complete before we can spawn buttons
            .add_systems(PreStartup, setup::spawn_layout_boxes)
            .add_systems(Startup, setup::spawn_buttons)
            // ACTIONS
            .add_systems(Update, actions::responsive_buttons)
            .add_systems(
                Update,
                (
                    actions::responsive_buttons,
                    actions::show_selected_input_mode,
                )
                    .chain(),
            );
    }
}

mod config {
    // The horizontal percentage of the screen that the UI panel takes up
    pub const UI_FRACTION: f32 = 50.0;
    /// The side length of the UI buttons
    pub const BUTTON_LENGTH: f32 = 64.0;
    /// The side length of the numpad-like input buttons
    pub const NUM_BUTTON_LENGTH: f32 = 64.0;
}

// QUALITY: reduce asset loading code duplication dramatically
mod assets {
    use super::*;
    /// The null, transparent color
    #[derive(Resource)]
    pub struct NoneColor(pub Handle<CustomUiMaterial>);

    impl FromWorld for NoneColor {
        fn from_world(world: &mut World) -> Self {
            let mut materials = world
                .get_resource_mut::<Assets<CustomUiMaterial>>()
                .expect("ResMut<Assets<CustomUiMaterial>> not found.");
            NoneColor(materials.add(CustomUiMaterial {
                color: Color::NONE.into(),
            }))
        }
    }

    /// Resource that contains the raw materials for each button type
    /// corresponding to the Marker type marker component
    #[derive(Resource)]
    pub struct ButtonMaterials<Marker: Component> {
        pub normal: Handle<ColorMaterial>,
        pub hovered: Handle<ColorMaterial>,
        pub pressed: Handle<ColorMaterial>,
        pub _marker: PhantomData<Marker>,
    }

    /// Component for the material of a button at rest
    #[derive(Component)]
    pub struct NormalMaterial(pub Handle<ColorMaterial>);
    /// Component for the material of a button when hovered
    #[derive(Component)]
    pub struct HoveredMaterial(pub Handle<ColorMaterial>);
    /// Component for the material of a button when pressed
    #[derive(Component)]
    pub struct PressedMaterial(pub Handle<ColorMaterial>);

    impl FromWorld for ButtonMaterials<NewPuzzle> {
        fn from_world(world: &mut World) -> Self {
            let mut materials = world
                .get_resource_mut::<Assets<ColorMaterial>>()
                .expect("ResMut<Assets<ColorMaterial>> not found.");
            ButtonMaterials {
                normal: materials.add(Color::rgb(1.0, 0.15, 0.15).into()),
                hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
                pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
                _marker: PhantomData,
            }
        }
    }

    impl FromWorld for ButtonMaterials<ResetPuzzle> {
        fn from_world(world: &mut World) -> Self {
            let mut materials = world
                .get_resource_mut::<Assets<ColorMaterial>>()
                .expect("ResMut<Assets<ColorMaterial>> not found.");
            ButtonMaterials {
                normal: materials.add(Color::rgb(0.15, 1.0, 0.15).into()),
                hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
                pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
                _marker: PhantomData,
            }
        }
    }

    impl FromWorld for ButtonMaterials<SolvePuzzle> {
        fn from_world(world: &mut World) -> Self {
            let mut materials = world
                .get_resource_mut::<Assets<ColorMaterial>>()
                .expect("ResMut<Assets<ColorMaterial>> not found.");
            ButtonMaterials {
                normal: materials.add(Color::rgb(0.15, 0.15, 1.0).into()),
                hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
                pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
                _marker: PhantomData,
            }
        }
    }

    impl FromWorld for ButtonMaterials<InputMode> {
        fn from_world(world: &mut World) -> Self {
            let mut materials = world
                .get_resource_mut::<Assets<ColorMaterial>>()
                .expect("ResMut<Assets<ColorMaterial>> not found.");
            ButtonMaterials {
                normal: materials.add(Color::rgb(0.15, 0.15, 0.15).into()),
                hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
                pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
                _marker: PhantomData,
            }
        }
    }

    impl FromWorld for ButtonMaterials<CellInput> {
        fn from_world(world: &mut World) -> Self {
            let mut materials = world
                .get_resource_mut::<Assets<ColorMaterial>>()
                .expect("ResMut<Assets<ColorMaterial>> not found.");
            ButtonMaterials {
                normal: materials.add(Color::rgb(0.8, 0.8, 0.8).into()),
                hovered: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
                pressed: materials.add(Color::rgb(0.35, 0.75, 0.35).into()),
                _marker: PhantomData,
            }
        }
    }
}

mod setup {
    use super::*;
    #[derive(Bundle)]
    struct BoardButtonBundle<Marker: Component> {
        marker: Marker,
        button_bundle: ButtonBundle,
        normal_material: NormalMaterial,
        hovered_material: HoveredMaterial,
        pressed_material: PressedMaterial,
    }

    impl<Marker: Component + Default> BoardButtonBundle<Marker> {
        fn new(size: (Val, Val), materials: &ButtonMaterials<Marker>) -> Self {
            let data = Marker::default();
            Self::new_with_data(size, materials, data)
        }
    }

    impl<Marker: Component> BoardButtonBundle<Marker> {
        fn new_with_data(
            size: (Val, Val),
            materials: &ButtonMaterials<Marker>,
            data: Marker,
        ) -> Self {
            let normal_material = materials.normal.clone();
            let hovered_material = materials.hovered.clone();
            let pressed_material = materials.pressed.clone();

            BoardButtonBundle {
                marker: data,
                button_bundle: ButtonBundle {
                    style: Style {
                        width: size.0,
                        height: size.1,
                        // Padding between buttons
                        margin: UiRect::all(Val::Px(5.0)),
                        // Horizontally center child text
                        justify_content: JustifyContent::Center,
                        // Vertically center child text
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    // material: normal_material.clone(), FIXME
                    ..Default::default()
                },
                normal_material: NormalMaterial(normal_material),
                hovered_material: HoveredMaterial(hovered_material),
                pressed_material: PressedMaterial(pressed_material),
            }
        }
    }

    /// Marker component for layout box of Sudoku game elements
    #[derive(Component)]
    pub struct SudokuBox;
    /// Marker component for layout box of UI elements
    #[derive(Component)]
    pub struct UiBox;

    /// Spawns layout-only nodes for storing the game's user interface
    pub fn spawn_layout_boxes(mut commands: Commands, none_color: Res<NoneColor>) {
        // Global root node
        commands
            .spawn(MaterialNodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    align_items: AlignItems::Center,
                    justify_content: JustifyContent::Center,
                    ..Default::default()
                },
                material: none_color.0.clone(),
                ..Default::default()
            })
            .with_children(|parent| {
                // Sudoku on left
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(100.0 - UI_FRACTION),
                            height: Val::Percent(100.0),
                            ..Default::default()
                        },
                        // material: none_color.0.clone(), FIXME
                        ..Default::default()
                    })
                    .insert(SudokuBox);

                // Interface on right
                parent
                    .spawn(NodeBundle {
                        style: Style {
                            width: Val::Percent(UI_FRACTION),
                            height: Val::Percent(100.0),
                            // UI elements are arranged in stacked rows, growing from the bottom
                            flex_direction: FlexDirection::ColumnReverse,
                            // Don't wrap these elements
                            flex_wrap: FlexWrap::NoWrap,
                            // These buttons should be grouped tightly together within each row
                            align_items: AlignItems::Center,
                            // Center the UI vertically
                            justify_content: JustifyContent::Center,
                            ..Default::default()
                        },
                        // material: none_color.0.clone(), FIXME
                        ..Default::default()
                    })
                    .insert(UiBox);
            });
    }

    /// Creates the side panel buttons
    pub fn spawn_buttons(
        mut commands: Commands,
        ui_root_query: Query<Entity, With<UiBox>>,
        new_button_materials: Res<ButtonMaterials<NewPuzzle>>,
        reset_button_materials: Res<ButtonMaterials<ResetPuzzle>>,
        solve_button_materials: Res<ButtonMaterials<SolvePuzzle>>,
        number_materials: Res<ButtonMaterials<CellInput>>,
        // TODO: split into three? Or maybe group into two resources total?
        input_mode_button_materials: Res<ButtonMaterials<InputMode>>,
        font: Res<FixedFont>,
    ) {
        let button_size = (Val::Px(BUTTON_LENGTH), Val::Px(BUTTON_LENGTH));
        let num_button_size = (Val::Px(NUM_BUTTON_LENGTH), Val::Px(NUM_BUTTON_LENGTH));

        // Layout nodes
        const N_ROWS: usize = 5;
        let mut layout_nodes = [Entity::from_raw(0); N_ROWS];
        for i in 0..N_ROWS {
            layout_nodes[i] = commands
                .spawn(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        align_content: AlignContent::Center,
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .id();
        }

        // Number input buttons
        let mut number_buttons = [Entity::from_raw(0); 9];
        for i in 0..9 {
            let num = i + 1;

            const TEXT_ALIGNMENT: TextAlignment = TextAlignment::Center;

            let text_style = TextStyle {
                font: font.0.clone(),
                font_size: 0.8 * NUM_BUTTON_LENGTH,
                color: Color::BLACK,
            };

            number_buttons[i] = commands
                .spawn(BoardButtonBundle::<CellInput>::new_with_data(
                    num_button_size,
                    &*number_materials,
                    CellInput { num: num as u8 },
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle {
                        text: Text::from_section(num.to_string(), text_style.clone())
                            .with_alignment(TEXT_ALIGNMENT),
                        ..Default::default()
                    });
                })
                .id();
        }

        // Input mode buttons
        let fill_button = commands
            .spawn(BoardButtonBundle::<InputMode>::new_with_data(
                button_size,
                &*input_mode_button_materials,
                InputMode::Fill,
            ))
            .id();

        let center_mark_button = commands
            .spawn(BoardButtonBundle::<InputMode>::new_with_data(
                button_size,
                &*input_mode_button_materials,
                InputMode::CenterMark,
            ))
            .id();

        let corner_mark_button = commands
            .spawn(BoardButtonBundle::<InputMode>::new_with_data(
                button_size,
                &*input_mode_button_materials,
                InputMode::CornerMark,
            ))
            .id();

        // Game control buttons
        let new_game_button = commands
            .spawn(BoardButtonBundle::<NewPuzzle>::new(
                button_size,
                &*new_button_materials,
            ))
            .id();

        let reset_game_button = commands
            .spawn(BoardButtonBundle::<ResetPuzzle>::new(
                button_size,
                &*reset_button_materials,
            ))
            .id();

        let solve_game_button = commands
            .spawn(BoardButtonBundle::<SolvePuzzle>::new(
                button_size,
                &*solve_button_materials,
            ))
            .id();

        // Building our hierarchy, from bottom to top
        let ui_root_entity = ui_root_query
            .get_single()
            .expect("No UI root entity found.");
        commands.entity(ui_root_entity).push_children(&layout_nodes);

        // Number buttons
        commands
            .entity(layout_nodes[0])
            .push_children(&number_buttons[0..3]);

        commands
            .entity(layout_nodes[1])
            .push_children(&number_buttons[3..6]);

        commands
            .entity(layout_nodes[2])
            .push_children(&number_buttons[6..9]);

        // Row 1 buttons
        commands.entity(layout_nodes[3]).push_children(&[
            fill_button,
            center_mark_button,
            corner_mark_button,
        ]);

        // Row 2 buttons
        commands.entity(layout_nodes[4]).push_children(&[
            new_game_button,
            reset_game_button,
            solve_game_button,
        ]);
    }
}

mod actions {
    use super::*;

    /// Marker component for entities whose materials should not respond
    #[derive(Component)]
    pub struct FixedMaterial;

    /// Changes the button materials when interacted with
    pub fn responsive_buttons(
        mut button_query: Query<
            (
                &Interaction,
                &mut Handle<ColorMaterial>,
                &NormalMaterial,
                &HoveredMaterial,
                &PressedMaterial,
            ),
            (Without<FixedMaterial>, Changed<Interaction>),
        >,
    ) {
        for (interaction, mut material, normal_material, hovered_material, pressed_material) in
            button_query.iter_mut()
        {
            *material = match *interaction {
                Interaction::None => normal_material.0.clone(),
                Interaction::Hovered => hovered_material.0.clone(),
                Interaction::Pressed => pressed_material.0.clone(),
            }
        }
    }

    /// Permanently displays selected input mode as pressed
    pub fn show_selected_input_mode(
        mut button_query: Query<(
            Entity,
            &InputMode,
            &mut Handle<ColorMaterial>,
            &PressedMaterial,
            &NormalMaterial,
        )>,
        input_mode: Res<InputMode>,
        mut commands: Commands,
    ) {
        if input_mode.is_changed() {
            for (entity, button_input_mode, mut material, pressed_material, normal_material) in
                button_query.iter_mut()
            {
                if *button_input_mode == *input_mode {
                    *material = pressed_material.0.clone();
                    commands.entity(entity).insert(FixedMaterial);
                } else {
                    *material = normal_material.0.clone();
                    commands.entity(entity).remove::<FixedMaterial>();
                }
            }
        }
    }
}

#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
pub struct CustomUiMaterial {
    #[uniform(0)]
    pub color: Vec4,
}

impl UiMaterial for CustomUiMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/custom_material.wgsl".into()
    }
}
