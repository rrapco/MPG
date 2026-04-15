use bevy::prelude::*;
use avian2d::prelude::*;
use crate::constants::*;
use crate::gamestate::InGameEntity;
use crate::texture::GameTextures;

pub const ROW_GAP: f32 = 50.0;

#[derive(Component)]
pub struct Slope;

pub fn spawn_tile(
    commands: &mut Commands,
    textures: &Res<GameTextures>,
    x: f32, y: f32,
    width: f32, height: f32,
) {
    commands.spawn((
        Sprite {
            image: textures.platform.clone(),
            custom_size: Some(Vec2::new(width, height)),
            ..default()
        },
        Transform::from_xyz(x, y, 0.0),
        InGameEntity,
        RigidBody::Static,
        Collider::rectangle(width, height),
    ));
}

pub fn spawn_slope(
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
        InGameEntity,
        RigidBody::Static,
        Collider::triangle(v1, v2, v3),
        Slope,
    ));
}