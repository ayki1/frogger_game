use bevy::prelude::*;
use crate::components::Frog;
use crate::GameState;
use crate::HighScore;

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.35, 0.75, 0.35);

#[derive(Component)]
pub struct MenuUI;

#[derive(Component)]
pub struct GameUI;

#[derive(Component)]
pub struct GameOverUI;

#[derive(Component)]
pub struct VictoryUI;

#[derive(Component)]
pub struct PlayButton;

#[derive(Component)]
pub struct ScoreText;

#[derive(Component)]
pub struct LivesText;

pub struct UIPlugin;

impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Menu), spawn_menu_ui)
            .add_systems(OnExit(GameState::Menu), cleanup_menu)
            .add_systems(OnEnter(GameState::Playing), (setup_game, spawn_game_ui))
            .add_systems(OnExit(GameState::Playing), cleanup_game_ui)
            .add_systems(OnEnter(GameState::GameOver), spawn_game_over_ui)
            .add_systems(OnExit(GameState::GameOver), cleanup_game_over)
            .add_systems(OnEnter(GameState::Victory), spawn_victory_ui)
            .add_systems(OnExit(GameState::Victory), cleanup_victory)
            .add_systems(Update, (button_system, update_game_ui).chain());
    }
}

pub fn spawn_menu_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..default()
                },
                ..default()
            },
            MenuUI,
        ))
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

pub fn spawn_game_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::FlexStart,
                    padding: UiRect::all(Val::Px(10.0)),
                    ..default()
                },
                ..default()
            },
            GameUI,
        ))
        .with_children(|parent| {
            // Score
            parent.spawn((
                TextBundle::from_section(
                    "Score: 0",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                ScoreText,
            ));

            // Lives
            parent.spawn((
                TextBundle::from_section(
                    "Lives: 3",
                    TextStyle {
                        font_size: 30.0,
                        color: Color::WHITE,
                        ..default()
                    },
                ),
                LivesText,
            ));
        });
}

pub fn spawn_game_over_ui(mut commands: Commands, high_score: Res<HighScore>) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            GameOverUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Game Over!",
                TextStyle {
                    font_size: 60.0,
                    color: Color::RED,
                    ..default()
                },
            ));

            parent.spawn(TextBundle::from_section(
                format!("High Score: {}", high_score.0),
                TextStyle {
                    font_size: 40.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play Again",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

pub fn spawn_victory_ui(mut commands: Commands) {
    commands
        .spawn((
            NodeBundle {
                style: Style {
                    width: Val::Percent(100.0),
                    height: Val::Percent(100.0),
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(20.0),
                    ..default()
                },
                background_color: Color::rgba(0.0, 0.0, 0.0, 0.7).into(),
                ..default()
            },
            VictoryUI,
        ))
        .with_children(|parent| {
            parent.spawn(TextBundle::from_section(
                "Victory!",
                TextStyle {
                    font_size: 60.0,
                    color: Color::GREEN,
                    ..default()
                },
            ));

            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.0),
                            height: Val::Px(65.0),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    PlayButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Play Again",
                        TextStyle {
                            font_size: 40.0,
                            color: Color::rgb(0.9, 0.9, 0.9),
                            ..default()
                        },
                    ));
                });
        });
}

pub fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = PRESSED_BUTTON.into();
                next_state.set(GameState::Playing);
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}

pub fn update_game_ui(
    mut query: ParamSet<(
        Query<&mut Text, With<ScoreText>>,
        Query<&mut Text, With<LivesText>>,
    )>,
    frog_query: Query<&Frog>,
) {
    if let Ok(frog) = frog_query.get_single() {
        if let Ok(mut text) = query.p0().get_single_mut() {
            text.sections[0].value = format!("Score: {}", frog.score);
        }
        if let Ok(mut text) = query.p1().get_single_mut() {
            text.sections[0].value = format!("Lives: {}", frog.lives);
        }
    }
}

pub fn setup_game(mut commands: Commands) {
    commands.spawn(Frog { score: 0, lives: 3 });
}

pub fn cleanup_menu(mut commands: Commands, menu_query: Query<Entity, With<MenuUI>>) {
    for entity in menu_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_game_ui(mut commands: Commands, ui_query: Query<Entity, With<GameUI>>) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_game_over(mut commands: Commands, ui_query: Query<Entity, With<GameOverUI>>) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

pub fn cleanup_victory(mut commands: Commands, ui_query: Query<Entity, With<VictoryUI>>) {
    for entity in ui_query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
