use bevy::prelude::*;
use avian2d::prelude::*;
use crate::constants::*;
use crate::gamestate::InGameEntity;
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
        //Mass(100000.0),
        //LockedAxes::ROTATION_LOCKED,
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
                max_distance: (TILE_SIZE_X * 4.0),
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

pub fn check_player_enemy_collision(
    player_query: Query<&Transform, With<Player>>,
    enemy_query: Query<&Transform, With<Enemy>>,
) {
    let Ok(player_transform) = player_query.single() else {
        return;
    };

    let player_pos = player_transform.translation.truncate();

    for enemy_transform in &enemy_query {
        let enemy_pos = enemy_transform.translation.truncate();

        let overlap_x = (player_pos.x - enemy_pos.x).abs() < (PLAYER_WIDTH / 2.0 + ENEMY_WIDTH / 2.0);
        let overlap_y = (player_pos.y - enemy_pos.y).abs() < (PLAYER_HEIGHT / 2.0 + ENEMY_HEIGHT / 2.0);

        if overlap_x && overlap_y {
            println!("You died");
            break;
        }
    }
}