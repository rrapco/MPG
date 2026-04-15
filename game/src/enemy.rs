use bevy::prelude::*;
use avian2d::prelude::*;
use crate::constants::*;
use crate::gamestate::{GameState, InGameEntity};
use crate::player::Player;

pub const ENEMY_WIDTH: f32 = 20.0;
pub const ENEMY_HEIGHT: f32 = 30.0;

#[derive(Component, Clone, PartialEq)]
pub enum EnemyType {
    Standing,
    Walking,
    Jumping,
}

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct WalkingState {
    pub direction: f32,
    pub distance_moved: f32,
    pub max_distance: f32,
}

#[derive(Component)]
pub struct JumpingState {
    pub jump_timer: Timer,
}

#[derive(Component)]
pub struct DeathText;

#[derive(Resource)]
pub struct Dead(pub Timer);

pub fn spawn_enemy(commands: &mut Commands, x: f32, y: f32, enemy_type: EnemyType) {
    let color = match enemy_type {
        EnemyType::Standing => Color::srgb(1.0, 0.0, 0.0),
        EnemyType::Walking  => Color::srgb(1.0, 0.5, 0.0),
        EnemyType::Jumping  => Color::srgb(0.8, 0.0, 0.8),
    };

    let mut entity = commands.spawn((
        Sprite::from_color(color, Vec2::new(ENEMY_WIDTH, ENEMY_HEIGHT)),
        Transform::from_xyz(x, y + ENEMY_HEIGHT / 2.0, 1.0),
        Enemy,
        InGameEntity,
        RigidBody::Dynamic,
        Collider::rectangle(ENEMY_WIDTH, ENEMY_HEIGHT),
        LinearVelocity::ZERO,
        LockedAxes::new()
            .lock_rotation()
            .lock_translation_x(),
        Friction::ZERO,
        Restitution::ZERO,
        enemy_type.clone(),
    ));

    match enemy_type {
        EnemyType::Walking => {
            entity.insert(WalkingState {
                direction: 1.0,
                distance_moved: 0.0,
                max_distance: TILE_SIZE_X * 4.0,
            });
        }
        EnemyType::Jumping => {
            entity.insert(JumpingState {
                jump_timer: Timer::from_seconds(2.0, TimerMode::Repeating),
            });
        }
        _ => {}
    }
}

pub fn update_enemies(
    time: Res<Time>,
    mut query: Query<(
        &EnemyType,
        &mut LinearVelocity,
        Option<&mut WalkingState>,
        Option<&mut JumpingState>,
    ), With<Enemy>>,
) {
    for (enemy_type, mut velocity, walking, jumping) in &mut query {
        match enemy_type {
            EnemyType::Standing => {
                velocity.x = 0.0;
            }
            EnemyType::Walking => {
                if let Some(mut state) = walking {
                    let speed = 60.0;
                    velocity.x = state.direction * speed;
                    state.distance_moved += speed * time.delta_secs();

                    if state.distance_moved >= state.max_distance {
                        state.direction *= -1.0;
                        state.distance_moved = 0.0;
                    }
                }
            }
            EnemyType::Jumping => {
                velocity.x = 0.0;
                if let Some(mut state) = jumping {
                    state.jump_timer.tick(time.delta());
                    if state.jump_timer.just_finished() {
                        velocity.y = 400.0;
                    }
                }
            }
        }
    }
}

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
                Text::new("Press SPACE to return to menu"),
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