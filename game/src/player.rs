
use bevy::prelude::*;

use crate::constants::{JUMP_FORCE, PLAYER_HEIGHT, PLAYER_SPEED, PLAYER_WIDTH};

#[derive(Component)]
pub struct Player;

#[derive(Component, Default)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component)]
pub struct OnGround(pub bool);

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb(1.0, 0.2, 0.2),
            Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT),
        ),
        Transform::from_xyz(0.0, -200.0, 1.0),
        Player,
        Velocity::default(),
        OnGround(false),
    ));
}

pub fn player_movement(
    time: Res<Time>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    player: Single<(&mut Velocity, &mut OnGround, &mut Transform), With<Player>>,
) {
    let (mut velocity, mut on_ground, mut transform) = player.into_inner();

    velocity.x = 0.0;

    if keyboard_input.pressed(KeyCode::KeyA) {
        velocity.x -= PLAYER_SPEED;
    }

    if keyboard_input.pressed(KeyCode::KeyD) {
        velocity.x += PLAYER_SPEED;
    }

    if keyboard_input.just_pressed(KeyCode::Space) && on_ground.0 {
        println!("Time: {:?} space kliknuty a elapsed {:?}",
                 time.delta(),
                 time.elapsed());
        velocity.y = JUMP_FORCE;
        on_ground.0 = false;
    }

    if keyboard_input.pressed(KeyCode::KeyR) {
        transform.translation.x = 0.0;
        transform.translation.y = -200.0;
        transform.translation.y = 1.0;
        on_ground.0 = true;
    }
}


pub fn apply_velocity(
    time: Res<Time>,
    mut player: Single<(&mut Transform, &Velocity), With<Player>>,
) {
    let (mut transform, velocity) = player.into_inner();

    transform.translation.x += velocity.x * time.delta_secs();
    transform.translation.y += velocity.y * time.delta_secs();
}