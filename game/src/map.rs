use bevy::prelude::*;
use avian2d::prelude::*;
use std::fs;

pub const TILE_SIZE: f32 = 20.0;
pub const ROW_GAP: f32 = 50.0;

#[derive(Component)]
pub struct Wall;

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

    let mut spawn_point = Vec2::new(TILE_SIZE, TILE_SIZE); // fallback

    for (row, line) in lines.iter().enumerate() {
        let mut col = 0;
        let chars: Vec<char> = line.chars().collect();

        while col < chars.len() {
            let ch = chars[col];

            let x = col as f32 * TILE_SIZE + TILE_SIZE / 2.0;
            let y = (map_height - 1 - row) as f32 * (TILE_SIZE + ROW_GAP) + TILE_SIZE / 2.0;

            match ch {
                '‾' => {
                    let start_col = col;
                    while col < chars.len() && chars[col] == '‾' {
                        col += 1;
                    }
                    let count = col - start_col;
                    let width = count as f32 * TILE_SIZE;
                    let cx = start_col as f32 * TILE_SIZE + width / 2.0;
                    spawn_tile(
                        &mut commands,
                        cx, y,
                        width, TILE_SIZE,
                        Color::srgb(0.2, 0.8, 0.3),
                        false,
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
    is_wall: bool,
) {
    let mut entity = commands.spawn((
        Sprite::from_color(color, Vec2::new(width, height)),
        Transform::from_xyz(x, y, 0.0),
        RigidBody::Static,
        Collider::rectangle(width, height),
    ));

    if is_wall {
        entity.insert(Wall);
    }
}

fn spawn_slope(
    commands: &mut Commands,
    x: f32, y: f32,
    flip: bool,
) {
    let (v1, v2, v3) = if !flip {
        (
            Vec2::new(-TILE_SIZE / 2.0, -TILE_SIZE / 2.0),
            Vec2::new(TILE_SIZE / 2.0, TILE_SIZE / 2.0),
            Vec2::new(TILE_SIZE / 2.0, -TILE_SIZE / 2.0),
        )
    } else {
        (
            Vec2::new(-TILE_SIZE / 2.0, TILE_SIZE / 2.0),
            Vec2::new(TILE_SIZE / 2.0, -TILE_SIZE / 2.0),
            Vec2::new(-TILE_SIZE / 2.0, -TILE_SIZE / 2.0),
        )
    };

    commands.spawn((
        Sprite::from_color(Color::srgb(0.9, 0.6, 0.1), Vec2::new(TILE_SIZE, TILE_SIZE)),
        Transform::from_xyz(x, y, 0.0),
        RigidBody::Static,
        Collider::triangle(v1, v2, v3),
        Slope,
    ));
}