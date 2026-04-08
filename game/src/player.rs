use bevy::prelude::*;
use avian2d::prelude::*;

use crate::constants::{JUMP_FORCE, PLAYER_HEIGHT, PLAYER_SPEED, PLAYER_WIDTH};

#[derive(Component)]
pub struct Player;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb(1.0, 0.2, 0.2),
            Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
        ),
        Transform::from_xyz(0.0, -200.0, 1.0),
        Player,
        RigidBody::Dynamic,
        Collider::rectangle(PLAYER_WIDTH, PLAYER_HEIGHT),
        LinearVelocity::ZERO,
        LockedAxes::ROTATION_LOCKED,
        Friction::ZERO,
        Restitution::ZERO,
    ));
}

pub fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut LinearVelocity, &Transform), With<Player>>,
    spatial_query: SpatialQuery,
) {
    let Ok((mut velocity, transform)) = query.single_mut() else {
        return;
    };

    velocity.x = 0.0;
    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity.x -= PLAYER_SPEED;
    }
    if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.x += PLAYER_SPEED;
    }

    let on_ground = spatial_query
        .cast_ray(
            transform.translation.truncate(),
            Dir2::NEG_Y,
            PLAYER_HEIGHT / 2.0 + 4.0,
            true,
            &SpatialQueryFilter::default(),
        )
        .is_some();
    
    if keyboard_input.just_pressed(KeyCode::Space) && on_ground {
        velocity.y = JUMP_FORCE;
    }

    if keyboard_input.just_pressed(KeyCode::KeyR) {
        velocity.x = 0.0;
        velocity.y = 0.0;
    }
}