use bevy::prelude::*;

pub fn setup_background(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let background = asset_server.load("background/obrazok.png");

    commands.spawn((
        Sprite::from_image(background),
        Transform {
            translation: Vec3::new(0.0, 0.0, -10.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..default()
        },
    ));
}