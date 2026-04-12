use bevy::prelude::*;
use std::time::Duration;
use crate::player::Player;
use avian2d::prelude::LinearVelocity;

// https://bevy.org/examples/2d-rendering/sprite-animation/

#[derive(Component, Clone, PartialEq)]
pub enum PlayerAnimation {
    Idle,
    Run,
}

#[derive(Resource)]
pub struct PlayerTextures {
    pub idle: Handle<Image>,
    pub run: Handle<Image>,
    pub idle_layout: Handle<TextureAtlasLayout>,
    pub run_layout: Handle<TextureAtlasLayout>,
}

#[derive(Component)]
pub struct AnimationConfig {
    pub first_sprite_index: usize,
    pub last_sprite_index: usize,
    pub fps: u8,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            first_sprite_index: first,
            last_sprite_index: last,
            fps,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(
            Duration::from_secs_f32(1.0 / fps as f32),
            TimerMode::Repeating,
        )
    }
}

pub fn load_player_textures(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlas_layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let idle_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(UVec2::new(80, 64), 7, 1, None, None)
    );
    let run_layout = texture_atlas_layouts.add(
        TextureAtlasLayout::from_grid(UVec2::new(80, 64), 8, 1, None, None)
    );

    commands.insert_resource(PlayerTextures {
        idle: asset_server.load("sprites/player/Mushroom-Idle.png"),
        run: asset_server.load("sprites/player/Mushroom-Run.png"),
        idle_layout,
        run_layout,
    });
}

pub fn execute_animations(
    time: Res<Time>,
    mut query: Query<(&mut AnimationConfig, &mut Sprite)>,
) {
    for (mut config, mut sprite) in &mut query {
        config.frame_timer.tick(time.delta());

        if config.frame_timer.just_finished() {
            if let Some(atlas) = &mut sprite.texture_atlas {
                if atlas.index == config.last_sprite_index {
                    atlas.index = config.first_sprite_index;
                } else {
                    atlas.index += 1;
                    config.frame_timer = AnimationConfig::timer_from_fps(config.fps);
                }
            }
        }
    }
}

pub fn update_player_animation(
    mut query: Query<
    (&LinearVelocity, &mut PlayerAnimation, &mut Sprite, &mut AnimationConfig),
    With<Player>,
    >,
    textures: Res<PlayerTextures>,
) {
    let Ok((velocity, mut current_anim, mut sprite, mut config)) = query.single_mut() else {
        return;
    };

    if velocity.x < -0.1 {
        sprite.flip_x = false;
    } else if velocity.x > 0.1 {
        sprite.flip_x = true;
    }

    let new_anim = if velocity.x.abs() > 0.1 {
        PlayerAnimation::Run
    } else {
        PlayerAnimation::Idle
    };

    if *current_anim == new_anim {
        return;
    }

    *current_anim = new_anim.clone();

    match new_anim {
        PlayerAnimation::Idle => {
            sprite.image = textures.idle.clone();
            sprite.texture_atlas = Some(TextureAtlas {
                layout: textures.idle_layout.clone(),
                index: 0,
            });
            *config = AnimationConfig::new(0, 6, 8);
        }
        PlayerAnimation::Run => {
            sprite.image = textures.run.clone();
            sprite.texture_atlas = Some(TextureAtlas {
                layout: textures.run_layout.clone(),
                index: 0,
            });
            *config = AnimationConfig::new(0, 7, 10);
        }
    }
}