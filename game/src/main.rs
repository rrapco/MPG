use bevy::prelude::*;
use avian2d::prelude::*;
use bevy::window::WindowResolution;

mod animation;
mod camera;
mod constants;
mod enemy;
mod gamestate;
mod map;
mod menu;
mod player;
mod texture;
mod ui;

use animation::{execute_animations, update_player_animation};
use camera::{camera_follow_player, setup_camera};
use constants::{WINDOW_HEIGHT, WINDOW_WIDTH};
use enemy::update_enemies;
use gamestate::GameState;
use map::{load_map, check_goal_collision, victory_countdown, victory_input, cleanup_ingame};
use menu::{cleanup_menu, menu_action, setup_menu};
use player::{player_movement, spawn_player, debug_player_position};
use texture::{load_textures, setup_background};
use ui::{setup_ui, detect_height_change, update_height_ui, HeightChanged};

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
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins(PhysicsPlugins::default())
        .add_plugins(PhysicsDebugPlugin::default())
        .insert_resource(Gravity(Vec2::NEG_Y * 900.0))
        .init_state::<GameState>()
        .add_message::<HeightChanged>()
        .add_systems(Startup, setup_camera)
        .add_systems(OnEnter(GameState::Menu), setup_menu)
        .add_systems(Update, menu_action.run_if(in_state(GameState::Menu)))
        .add_systems(OnExit(GameState::Menu), cleanup_menu)
        .add_systems(
            OnEnter(GameState::InGame),
            (
                load_textures,
                setup_background,
                load_map,
                spawn_player,
                setup_ui,
            ).chain(),
        )
        .add_systems(
            Update,
            (
                player_movement,
                //debug_player_position,
                update_player_animation,
                execute_animations,
                detect_height_change,
                update_height_ui,
                update_enemies,
                check_goal_collision,
                victory_input,
                victory_countdown,
                camera_follow_player,
            )
                .chain()
                .run_if(in_state(GameState::InGame)),
        )
        .add_systems(OnExit(GameState::InGame), cleanup_ingame)
        .run();
}