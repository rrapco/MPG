use bevy::prelude::*;

use crate::player::Player;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

pub fn camera_follow_player(
    player: Single<&Transform, (With<Player>, Without<Camera2d>)>,
    mut camera: Single<&mut Transform, (With<Camera2d>, Without<Player>)>,
) {
    camera.translation.x = player.translation.x;
    camera.translation.y = player.translation.y;
}