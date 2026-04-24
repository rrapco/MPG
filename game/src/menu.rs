use bevy::app::AppExit;
use bevy::prelude::*;

use crate::gamestate::GameState;
use crate::map::loader::CurrentLevel;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    LevelSelect,
    Leaderboard,
    SelectLevel(usize),
    Back,
    Quit,
}

pub fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/NotoSans-Regular.ttf");
    let background = asset_server.load("background/main_menu.png");

    spawn_background(&mut commands, background);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Px(340.0),
                    height: Val::Px(320.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(20.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Kráľ skoku"),
                        TextFont {
                            font: font.clone(),
                            font_size: 42.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    spawn_button(parent, "Play", MenuButtonAction::Play);
                    spawn_button(parent, "Level Change", MenuButtonAction::LevelSelect);
                    spawn_button(parent, "Leaderboard", MenuButtonAction::Leaderboard);
                    spawn_button(parent, "Quit", MenuButtonAction::Quit);
                });
        });
}

pub fn setup_level_select(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    current_level: Res<CurrentLevel>,
) {
    let font = asset_server.load("fonts/NotoSans-Regular.ttf");
    let background = asset_server.load("background/main_menu.png");

    spawn_background(&mut commands, background);

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Px(380.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(18.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Choose level"),
                        TextFont {
                            font: font.clone(),
                            font_size: 42.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    for level in 1..=current_level.max {
                        let label = format!("Level {}", level);
                        spawn_button(parent, &label, MenuButtonAction::SelectLevel(level));
                    }

                    spawn_button(parent, "Back", MenuButtonAction::Back);
                });
        });
}

fn spawn_background(commands: &mut Commands, background: Handle<Image>) {
    commands.spawn((
        ImageNode::new(background),
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        },
        MainMenuUI,
    ));
}

fn spawn_button(parent: &mut ChildSpawnerCommands, text: &str, action: MenuButtonAction) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(240.0),
                height: Val::Px(65.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.2, 0.2, 0.2)),
            action,
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new(text),
                TextFont {
                    font_size: 30.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn menu_action(
    mut interaction_query: Query<
        (&Interaction, &MenuButtonAction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
    mut current_level: ResMut<CurrentLevel>,
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.35, 0.75, 0.35));

                match action {
                    MenuButtonAction::Play => {
                        current_level.current = 1;
                        next_state.set(GameState::LoadingLevel);
                    }
                    MenuButtonAction::LevelSelect => {
                        next_state.set(GameState::LevelSelect);
                    }
                    MenuButtonAction::Leaderboard => {
                        next_state.set(GameState::Leaderboard);
                    }
                    MenuButtonAction::SelectLevel(level) => {
                        current_level.current = *level;
                        next_state.set(GameState::LoadingLevel);
                    }
                    MenuButtonAction::Back => {
                        next_state.set(GameState::Menu);
                    }
                    MenuButtonAction::Quit => {
                        exit.write(AppExit::Success);
                    }
                }
            }
            Interaction::Hovered => {
                *color = BackgroundColor(Color::srgb(0.35, 0.35, 0.35));
            }
            Interaction::None => {
                *color = BackgroundColor(Color::srgb(0.2, 0.2, 0.2));
            }
        }
    }
}

pub fn setup_leaderboard(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/NotoSans-Regular.ttf");
    let background = asset_server.load("background/main_menu.png");

    spawn_background(&mut commands, background);

    let times_text = std::fs::read_to_string("times.txt")
        .unwrap_or("No times yet".to_string());

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Px(420.0),
                    flex_direction: FlexDirection::Column,
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    row_gap: Val::Px(18.0),
                    ..default()
                })
                .with_children(|parent| {
                    parent.spawn((
                        Text::new("Leaderboard"),
                        TextFont {
                            font: font.clone(),
                            font_size: 42.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    parent.spawn((
                        Text::new(times_text),
                        TextFont {
                            font: font.clone(),
                            font_size: 28.0,
                            ..default()
                        },
                        TextColor(Color::WHITE),
                    ));

                    spawn_button(parent, "Back", MenuButtonAction::Back);
                });
        });
}

pub fn cleanup_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_children();
        commands.entity(entity).despawn();
    }
}