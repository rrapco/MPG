use bevy::app::AppExit;
use bevy::prelude::*;

use crate::gamestate::GameState;

#[derive(Component)]
pub struct MainMenuUI;

#[derive(Component)]
pub enum MenuButtonAction {
    Play,
    Quit,
}

pub fn setup_menu(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/NotoSans-Regular.ttf");

    commands
        .spawn((
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.08)),
            MainMenuUI,
        ))
        .with_children(|parent| {
            parent
                .spawn(Node {
                    width: Val::Px(300.0),
                    height: Val::Px(220.0),
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
                    spawn_button(parent, "Quit", MenuButtonAction::Quit);
                });
        });
}

fn spawn_button(parent: &mut ChildSpawnerCommands, text: &str, action: MenuButtonAction) {
    parent
        .spawn((
            Button,
            Node {
                width: Val::Px(220.0),
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
    mut next_state: ResMut<NextState<GameState>>,
    mut exit: MessageWriter<AppExit>,
) {
    for (interaction, action, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Pressed => {
                *color = BackgroundColor(Color::srgb(0.35, 0.75, 0.35));

                match action {
                    MenuButtonAction::Play => {
                        next_state.set(GameState::LoadingLevel);
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

pub fn cleanup_menu(
    mut commands: Commands,
    query: Query<Entity, With<MainMenuUI>>,
) {
    for entity in &query {
        commands.entity(entity).despawn_children();
        commands.entity(entity).despawn();
    }
}