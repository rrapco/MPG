use bevy::prelude::*;
use avian2d::prelude::*;

use crate::constants::{JUMP_FORCE, PLAYER_HEIGHT, PLAYER_SPEED, PLAYER_WIDTH};
use crate::map::loader::PlayerSpawnPoint;
use crate::animation::{AnimationConfig, PlayerAnimation};
use crate::texture::GameTextures;
use crate::gamestate::InGameEntity;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct PlayerSprite;

pub fn spawn_player(
    mut commands: Commands,
    spawn_point: Res<PlayerSpawnPoint>,
    textures: Res<GameTextures>,
) {
    commands
        .spawn((
            Transform::from_xyz(spawn_point.0.x, spawn_point.0.y, 1.0),
            Visibility::Visible,
            Player,
            InGameEntity,
            RigidBody::Dynamic,
            Collider::rectangle(PLAYER_WIDTH, PLAYER_HEIGHT),
            LinearVelocity::ZERO,
            LockedAxes::ROTATION_LOCKED,
            Friction::ZERO,
            Restitution::ZERO,
        ))
        .with_children(|parent| {
            parent.spawn((
                Sprite {
                    image: textures.player_idle.clone(),
                    texture_atlas: Some(TextureAtlas {
                        layout: textures.player_idle_layout.clone(),
                        index: 0,
                    }),
                    ..default()
                },
                Transform::from_xyz(0.0, 15.0, 0.0),
                PlayerSprite,
                PlayerAnimation::Idle,
                AnimationConfig::new(0, 6, 8),
            ));
        });
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(Entity, &mut LinearVelocity, &mut Transform), With<Player>>,
    spatial_query: SpatialQuery,
    spawn_point: Res<PlayerSpawnPoint>,
) {
    let Ok((player_entity, mut velocity, mut transform)) = query.single_mut() else {
        return;
    };

    // movement
    velocity.x = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity.x -= PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.x += PLAYER_SPEED;
    }

    let filter = SpatialQueryFilter::default().with_excluded_entities([player_entity]);

    let ray_distance = PLAYER_HEIGHT / 2.0 + 4.0;

    let ground_check_width = PLAYER_WIDTH / 2.0 - 2.0;

    let center = transform.translation.truncate();
    let left = center + Vec2::new(-ground_check_width, 0.0);
    let right = center + Vec2::new(ground_check_width, 0.0);

    // ground check
    let on_ground =
        spatial_query
            .cast_ray(center, Dir2::NEG_Y, ray_distance, true, &filter)
            .is_some()
            || spatial_query
            .cast_ray(left, Dir2::NEG_Y, ray_distance, true, &filter)
            .is_some()
            || spatial_query
            .cast_ray(right, Dir2::NEG_Y, ray_distance, true, &filter)
            .is_some();

    // skok
    if keyboard_input.just_pressed(KeyCode::Space) && on_ground {
        velocity.y = JUMP_FORCE;
    }

    if keyboard_input.just_pressed(KeyCode::KeyW) && on_ground {
        velocity.y = JUMP_FORCE;
    }

    // reset, treba pridat ked spadne player pod -10 y napriklad
    // if keyboard_input.pressed(KeyCode::KeyR) {
    //     transform.translation.x = spawn_point.0.x;
    //     transform.translation.y = spawn_point.0.y;
    //     transform.translation.z = 1.0;
    //     velocity.x = 0.0;
    //     velocity.y = 0.0;
    // }
}

pub fn debug_player_position(
    query: Query<&Transform, With<Player>>,
) {
    if let Ok(transform) = query.single() {
        println!(
            "Player position: x = {:.2}, y = {:.2}",
            transform.translation.x,
            transform.translation.y
        );
    }
}