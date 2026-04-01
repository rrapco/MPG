use bevy::prelude::*;

#[derive(Component)]
pub struct Platform;

#[derive(Component)]
pub struct PlatformSize {
    pub width: f32,
    pub height: f32,
}

pub fn spawn_platform(
    commands: &mut Commands,
    x: f32,
    y: f32,
    width: f32,
    height: f32,
) {
    commands.spawn((
        Sprite::from_color(
            Color::srgb(0.2, 0.8, 0.3),
            Vec2::new(width, height),
        ),
        Transform::from_xyz(x, y, 0.0),
        Platform,
        PlatformSize { width, height },
    ));
}

pub fn setup_platforms(mut commands: Commands) {
    spawn_platform(&mut commands, 0.0, -300.0, 500.0, 20.0);
    spawn_platform(&mut commands, 200.0, -150.0, 160.0, 20.0);
    spawn_platform(&mut commands, -100.0, 0.0, 160.0, 20.0);
    spawn_platform(&mut commands, 100.0, 150.0, 140.0, 20.0);
}