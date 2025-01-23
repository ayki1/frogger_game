use bevy::prelude::*;
use bevy::window::PrimaryWindow;

mod components;
mod systems;
mod ui;

use components::*;
use systems::*;
use ui::*;

const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;
const ROAD_HEIGHT: f32 = 250.0;
const RIVER_HEIGHT: f32 = 250.0;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    Playing,
    GameOver,
    Victory,
}

#[derive(Resource)]
pub struct GameData {
    pub high_score: i32,
}

impl Default for GameData {
    fn default() -> Self {
        Self { high_score: 0 }
    }
}

#[derive(Resource)]
pub struct HighScore(pub i32);

impl Default for HighScore {
    fn default() -> Self {
        Self(0)
    }
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Frogger".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                ..default()
            }),
            ..default()
        }))
        .init_resource::<GameData>()
        .init_resource::<HighScore>()
        .add_state::<GameState>()
        .add_systems(Startup, setup_game)
        .add_plugins(UIPlugin)
        .add_systems(
            Update,
            (
                movement_system,
                collision_system,
                vehicle_river_collision_system,
                spawn_vehicles,
                spawn_river_objects,
                move_river_objects,
                check_victory,
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        )
        .init_resource::<LaneObjects>()
        .run();
}

fn setup_game(
    mut commands: Commands,
) {
    // Camera
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        ..default()
    });

    // Spawn initial frog
    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.0, 0.8, 0.0),
                custom_size: Some(Vec2::new(30.0, 30.0)),
                ..default()
            },
            transform: Transform::from_xyz(
                0.0,
                -WINDOW_HEIGHT / 2.0 + 50.0,
                1.0,
            ),
            ..default()
        },
        Frog {
            score: 0,
            lives: 3,
        },
    ));

    // Spawn road lanes
    for y in [-180.0, -120.0, -60.0] {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.15, 0.15, 0.15),
                    custom_size: Some(Vec2::new(WINDOW_WIDTH, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, y, 0.0),
                ..default()
            },
            Road,
        ));
    }

    // Spawn river lanes
    for y in [60.0, 120.0, 180.0] {
        commands.spawn((
            SpriteBundle {
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.3, 0.8),
                    custom_size: Some(Vec2::new(WINDOW_WIDTH, 50.0)),
                    ..default()
                },
                transform: Transform::from_xyz(0.0, y, 0.0),
                ..default()
            },
            River,
        ));
    }

    // Spawn score text
    commands.spawn(
        TextBundle::from_section(
            "Score: 0",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(10.0),
            ..default()
        }),
    );

    // Spawn lives text
    commands.spawn(
        TextBundle::from_section(
            "Lives: 3",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            left: Val::Px(10.0),
            top: Val::Px(50.0),
            ..default()
        }),
    );

    // Start in Playing state
    commands.insert_resource(NextState(Some(GameState::Playing)));
}
