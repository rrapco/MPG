use bevy::prelude::*;

mod camera;
mod constants;
mod physics;
mod platform;
mod player;
mod background;

use camera::{camera_follow_player, setup_camera};
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use background::setup_background;
use physics::{apply_gravity, player_platform_collision};
use platform::setup_platforms;
use player::{apply_velocity, spawn_player, player_movement};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Temu Jump King".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, (setup_background, setup_camera, setup_platforms, spawn_player))
        .add_systems(
            Update,
            (
                player_movement,
                apply_gravity,
                apply_velocity,
                player_platform_collision,
                camera_follow_player,
            )
                .chain(),
        )
        .run();
}