use bevy::prelude::*;
use std::time::Duration;
use avian2d::prelude::LinearVelocity;
use crate::player::Player;
use crate::texture::GameTextures;

// https://bevy.org/examples/2d-rendering/sprite-animation/

#[derive(Component, Clone, PartialEq)]
pub enum PlayerAnimation {
    Idle,
    Run,
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
    textures: Res<GameTextures>,
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
            sprite.image = textures.player_idle.clone();
            sprite.texture_atlas = Some(TextureAtlas {
                layout: textures.player_idle_layout.clone(),
                index: 0,
            });
            *config = AnimationConfig::new(0, 6, 8);
        }
        PlayerAnimation::Run => {
            sprite.image = textures.player_run.clone();
            sprite.texture_atlas = Some(TextureAtlas {
                layout: textures.player_run_layout.clone(),
                index: 0,
            });
            *config = AnimationConfig::new(0, 7, 10);
        }
    }
}