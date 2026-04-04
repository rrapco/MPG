use bevy::prelude::*;

use crate::platform::{Platform, PlatformSize};
use crate::player::{OnGround, Player, Velocity};
use crate::constants::{JUMP_FORCE, PLAYER_HEIGHT, PLAYER_SPEED, PLAYER_WIDTH, GRAVITY};

pub fn apply_gravity(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &OnGround), With<Player>>,
) {
    for (mut velocity, on_ground) in &mut query {
        if !on_ground.0 {
            velocity.y -= GRAVITY * time.delta_secs();
        }
    }
}
pub fn player_platform_collision(
    time: Res<Time>,
    mut player_query: Query<
        (&mut Transform, &mut Velocity, &mut OnGround),
        (With<Player>, Without<Platform>),
    >,
    platform_query: Query<
        (&Transform, &PlatformSize),
        (With<Platform>, Without<Player>),
    >,
) {
    let Ok((mut player_transform, mut player_velocity, mut on_ground)) = player_query.single_mut() else {
        return;
    };

    on_ground.0 = false;

    let player_half_width = PLAYER_WIDTH / 2.0;
    let player_half_height = PLAYER_HEIGHT / 2.0;

    let player_left = player_transform.translation.x - player_half_width;
    let player_right = player_transform.translation.x + player_half_width;
    let player_bottom = player_transform.translation.y - player_half_height;
    let player_top = player_transform.translation.y + player_half_height;

    let previous_bottom = player_bottom - player_velocity.y * time.delta_secs();

    for (platform_transform, platform_size) in &platform_query {
        // funguje tento kod?
        if (platform_transform.translation.y - player_transform.translation.y).abs() > 200.0 {
            continue;
        }
        let platform_half_width = platform_size.width / 2.0;
        let platform_half_height = platform_size.height / 2.0;

        let platform_left = platform_transform.translation.x - platform_half_width;
        let platform_right = platform_transform.translation.x + platform_half_width;
        let platform_bottom = platform_transform.translation.y - platform_half_height;
        let platform_top = platform_transform.translation.y + platform_half_height;

        let overlaps_x = player_right > platform_left && player_left < platform_right;

        let crossed_platform_top =
            previous_bottom >= platform_top && player_bottom <= platform_top;

        if overlaps_x && crossed_platform_top && player_velocity.y <= 0.0 {
            player_transform.translation.y = platform_top + player_half_height;
            player_velocity.y = 0.0;
            on_ground.0 = true;
        }
    }
}