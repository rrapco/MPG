use bevy::prelude::*;
use avian2d::prelude::*;
use bevy::window::WindowResolution;

mod camera;
mod constants;
mod player;
mod map;
mod animation;
mod texture;

use texture::{load_textures, setup_background};
use map::load_map;
use camera::{camera_follow_player, setup_camera};
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use player::{spawn_player, player_movement};
use animation::{execute_animations, update_player_animation};

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
        .add_plugins(PhysicsDebugPlugin::default())
        .insert_resource(Gravity(Vec2::NEG_Y * 900.0))
        .add_systems(Startup, (load_textures, setup_background, load_map, setup_camera, spawn_player).chain())
        .add_systems(
            Update,
            (player_movement,
             update_player_animation,
             execute_animations,
             camera_follow_player).chain(),
        )
        .run();
}