use bevy::prelude::*;
use std::fs;

use crate::constants::*;
use crate::enemy::{spawn_enemy, EnemyType};
use crate::map::goal::spawn_goal;
use crate::map::tiles::{spawn_slope, spawn_tile, ROW_GAP};
use crate::texture::GameTextures;

#[derive(Resource)]
pub struct PlayerSpawnPoint(pub Vec2);

pub fn load_map(
    mut commands: Commands,
    textures: Res<GameTextures>,
) {
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
                        &textures,
                        cx,
                        y,
                        width,
                        TILE_SIZE_Y,
                    );
                    continue;
                }
                '/' => {
                    spawn_slope(&mut commands, x, y, false);
                }
                '\\' => {
                    spawn_slope(&mut commands, x, y, true);
                }
                'p' => {
                    spawn_point = Vec2::new(x, y);
                }
                'c' => {
                    spawn_goal(&mut commands, x, y);
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