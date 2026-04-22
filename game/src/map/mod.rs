use bevy::prelude::*;
use crate::death::Dead;
use crate::gamestate::InGameEntity;
use crate::map::goal::VictoryTimer;
use crate::map::loader::PlayerSpawnPoint;

pub mod loader;
pub mod goal;
pub mod tiles;

pub use loader::*;
pub use goal::*;
pub use tiles::*;

pub fn cleanup_ingame(
    mut commands: Commands,
    query: Query<Entity, With<InGameEntity>>,
    has_victory_timer: Option<Res<VictoryTimer>>,
    has_dead: Option<Res<Dead>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    if has_victory_timer.is_some() {
        commands.remove_resource::<VictoryTimer>();
    }

    if has_dead.is_some() {
        commands.remove_resource::<Dead>();
    }

    commands.remove_resource::<PlayerSpawnPoint>();
}