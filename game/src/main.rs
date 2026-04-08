use bevy::prelude::*;
use avian2d::prelude::*;
use bevy::window::WindowResolution;

mod camera;
mod constants;
mod platform;
mod player;
mod background;

use camera::{camera_follow_player, setup_camera};
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use background::setup_background;
use platform::setup_platforms;
use player::{spawn_player, player_movement};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Temu Jump King".into(),
                resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                resizable: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugins(PhysicsPlugins::default())
        .insert_resource(Gravity(Vec2::NEG_Y * 900.0))
        .add_systems(Startup, (setup_background, setup_camera, setup_platforms, spawn_player))
        .add_systems(
            Update,
            (player_movement, camera_follow_player).chain(),
        )
        .run();
}