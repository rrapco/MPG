use bevy::prelude::*;
use avian2d::prelude::*;
use std::fs;
use crate::enemy::{spawn_enemy, EnemyType};
use crate::constants::{TILE_SIZE_X, TILE_SIZE_Y};

pub const ROW_GAP: f32 = 50.0;

#[derive(Component)]
pub struct Slope;

#[derive(Resource)]
pub struct PlayerSpawnPoint(pub Vec2);

pub fn load_map(mut commands: Commands) {
    let content = match fs::read_to_string("assets/maps/level1.txt") {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Chyba pri nacitani mapy: {}", e);
            return;
        }
    };

    let lines: Vec<&str> = content.lines().collect();
    let map_height = lines.len();

    let mut spawn_point = Vec2::new(TILE_SIZE_X, TILE_SIZE_Y);

    for (row, line) in lines.iter().enumerate() {
        let mut col = 0;
        let chars: Vec<char> = line.chars().collect();

        while col < chars.len() {
            let ch = chars[col];

            let x = col as f32 * TILE_SIZE_X + TILE_SIZE_X / 2.0;
            let y = (map_height - 1 - row) as f32 * (TILE_SIZE_Y + ROW_GAP) + TILE_SIZE_Y / 2.0;

            match ch {
                '_' => {
                    let start_col = col;
                    while col < chars.len() && chars[col] == '_' {
                        col += 1;
                    }
                    let count = col - start_col;
                    let width = count as f32 * TILE_SIZE_X;
                    let cx = start_col as f32 * TILE_SIZE_X + width / 2.0;
                    spawn_tile(
                        &mut commands,
                        cx, y,
                        width, TILE_SIZE_Y, // výška platformy
                        Color::srgb(0.2, 0.8, 0.3),
                    );
                    continue;
                }
                '/' => {
                    spawn_slope(&mut commands, x, y, false);
                }
                '\\' => {
                    spawn_slope(&mut commands, x, y, true);
                }
                'P' => {
                    spawn_point = Vec2::new(x, y);
                }
                'e' => {
                    spawn_enemy(&mut commands, x, y, EnemyType::Standing);
                }
                'w' => {
                    spawn_enemy(&mut commands, x, y, EnemyType::Walking);
                }
                'j' => {
                    spawn_enemy(&mut commands, x, y, EnemyType::Jumping);
                }
                _ => {}
            }

            col += 1;
        }
    }

    commands.insert_resource(PlayerSpawnPoint(spawn_point));
}

fn spawn_tile(
    commands: &mut Commands,
    x: f32, y: f32,
    width: f32, height: f32,
    color: Color,
) {
    commands.spawn((
        Sprite::from_color(color, Vec2::new(width, height)),
        Transform::from_xyz(x, y, 0.0),
        RigidBody::Static,
        Collider::rectangle(width, height),
    ));
}

fn spawn_slope(
    commands: &mut Commands,
    x: f32, y: f32,
    flip: bool,
) {
    let (v1, v2, v3) = if !flip {
        (
            Vec2::new(-TILE_SIZE_X / 2.0, -TILE_SIZE_Y / 2.0),
            Vec2::new(TILE_SIZE_X / 2.0, TILE_SIZE_Y / 2.0),
            Vec2::new(TILE_SIZE_X / 2.0, -TILE_SIZE_Y / 2.0),
        )
    } else {
        (
            Vec2::new(-TILE_SIZE_X / 2.0, TILE_SIZE_Y / 2.0),
            Vec2::new(TILE_SIZE_X / 2.0, -TILE_SIZE_Y / 2.0),
            Vec2::new(-TILE_SIZE_X / 2.0, -TILE_SIZE_Y / 2.0),
        )
    };

    let rotation = if !flip {
        std::f32::consts::FRAC_PI_4
    } else {
        -std::f32::consts::FRAC_PI_4
    };

    commands.spawn((
        Sprite::from_color(Color::srgb(0.9, 0.6, 0.1), Vec2::new(TILE_SIZE_X, TILE_SIZE_Y)),
        Transform::from_xyz(x, y, 0.0)
            .with_rotation(Quat::from_rotation_z(rotation)),
        RigidBody::Static,
        Collider::triangle(v1, v2, v3),
        Slope,
    ));
}