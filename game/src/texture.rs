use bevy::prelude::*;
use crate::gamestate::InGameEntity;

#[derive(Resource)]
pub struct GameTextures {
    pub player_idle: Handle<Image>,
    pub player_run: Handle<Image>,
    pub player_idle_layout: Handle<TextureAtlasLayout>,
    pub player_run_layout: Handle<TextureAtlasLayout>,
    pub platform: Handle<Image>,
}

pub fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let background = asset_server.load("background/roman_hra.png");

    commands.spawn((
        Sprite::from_image(background),
        Transform {
            translation: Vec3::new(0.0, 1000.0, -10.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..default()
        },
        InGameEntity,
    ));
}

pub fn load_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let player_idle_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(UVec2::new(80, 64), 7, 1, None, None)
    );
    let player_run_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(UVec2::new(80, 64), 8, 1, None, None)
    );

    commands.insert_resource(GameTextures {
        player_idle: asset_server.load("sprites/player/Mushroom-Idle.png"),
        player_run: asset_server.load("sprites/player/Mushroom-Run.png"),
        player_idle_layout,
        player_run_layout,
        platform: asset_server.load("sprites/tiles/platform.png"),
    });
}