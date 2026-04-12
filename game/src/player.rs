use bevy::prelude::*;
use avian2d::prelude::*;

use crate::constants::{JUMP_FORCE, PLAYER_HEIGHT, PLAYER_SPEED, PLAYER_WIDTH};
use crate::map::PlayerSpawnPoint;
use crate::animation::{AnimationConfig, PlayerAnimation};
use crate::texture::GameTextures;

#[derive(Component)]
pub struct Player;

pub fn spawn_player(
    mut commands: Commands,
    spawn_point: Res<PlayerSpawnPoint>,
    textures: Res<GameTextures>,
) {
    commands.spawn((
        Sprite {
            image: textures.player_idle.clone(),
            texture_atlas: Some(TextureAtlas {
                layout: textures.player_idle_layout.clone(),
                index: 0,
            }),
            ..default()
        },
        Transform::from_xyz(spawn_point.0.x, spawn_point.0.y, 1.0),
        Player,
        RigidBody::Dynamic,
        Collider::rectangle(PLAYER_WIDTH, PLAYER_HEIGHT),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        Friction::ZERO,
        Restitution::ZERO,
        PlayerAnimation::Idle,
        AnimationConfig::new(0, 6, 8),
    ));
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

    velocity.x = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity.x -= PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.x += PLAYER_SPEED;
    }

    let filter = SpatialQueryFilter::default().with_excluded_entities([player_entity]);

    let on_ground = spatial_query
        .cast_ray(
            transform.translation.truncate(),
            Dir2::NEG_Y,
            PLAYER_HEIGHT / 2.0 + 4.0,
            true,
            &filter,
        )
        .is_some();

    if keyboard_input.just_pressed(KeyCode::Space) && on_ground {
        velocity.y = JUMP_FORCE;
    }

    if keyboard_input.pressed(KeyCode::KeyR) {
        transform.translation.x = spawn_point.0.x;
        transform.translation.y = spawn_point.0.y;
        transform.translation.z = 1.0;
        velocity.x = 0.0;
        velocity.y = 0.0;
    }
}