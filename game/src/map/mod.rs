use bevy::prelude::*;
use crate::gamestate::InGameEntity;
use crate::map::goal::VictoryTimer;
use crate::map::loader::PlayerSpawnPoint;

pub mod loader;
pub mod goal;
pub mod tiles;

pub use loader::*;
pub use goal::*;

pub fn cleanup_ingame(
    mut commands: Commands,
    query: Query<Entity, With<InGameEntity>>,
    has_timer: Option<Res<VictoryTimer>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }

    if has_timer.is_some() {
        commands.remove_resource::<VictoryTimer>();
    }

    commands.remove_resource::<PlayerSpawnPoint>();
}