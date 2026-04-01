use bevy::prelude::*;

use crate::constants::{GRAVITY, PLAYER_HEIGHT, PLAYER_WIDTH};
use crate::platform::{Platform, PlatformSize};
use crate::player::{OnGround, Player, Velocity};

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
