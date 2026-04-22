use bevy::prelude::*;

use crate::constants::*;
use crate::enemy::{Enemy, ENEMY_HEIGHT, ENEMY_WIDTH};
use crate::gamestate::{GameState, InGameEntity};
use crate::player::Player;

#[derive(Component)]
pub struct DeathText;

#[derive(Resource)]
pub struct Dead(pub Timer);

fn spawn_death_ui(commands: &mut Commands) {
    commands.spawn((
        Node {
            position_type: PositionType::Absolute,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.6)),
        DeathText,
        InGameEntity,
    ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("YOU DIED"),
                TextFont {
                    font_size: 72.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 0.1, 0.1)),
            ));

            parent.spawn((
                Text::new("Press SPACE to restart"),
                TextFont {
                    font_size: 28.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));
        });
}

pub fn check_player_enemy_collision(
    mut commands: Commands,
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
    dead: Option<Res<Dead>>,
) {
    if dead.is_some() {
        return;
    }

    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();

    for enemy_transform in &enemy_query {
        let enemy_pos = enemy_transform.translation.truncate();

        let overlap_x =
            (player_pos.x - enemy_pos.x).abs() < (PLAYER_WIDTH / 2.0 + ENEMY_WIDTH / 2.0);
        let overlap_y =
            (player_pos.y - enemy_pos.y).abs() < (PLAYER_HEIGHT / 2.0 + ENEMY_HEIGHT / 2.0);

        if overlap_x && overlap_y {
            println!("You died");
            spawn_death_ui(&mut commands);
            commands.insert_resource(Dead(Timer::from_seconds(60.0, TimerMode::Once)));
            break;
        }
    }
}

pub fn death_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
    dead: Option<Res<Dead>>,
) {
    if dead.is_none() {
        return;
    }

    if keyboard_input.just_pressed(KeyCode::Space) {
        next_state.set(GameState::Menu);
    }
}

pub fn death_countdown(
    time: Res<Time>,
    dead: Option<ResMut<Dead>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    let Some(mut dead) = dead else {
        return;
    };

    dead.0.tick(time.delta());

    if dead.0.just_finished() {
        next_state.set(GameState::Menu);
    }
}