use bevy::prelude::*;
use avian2d::prelude::*;
use crate::enemy::Enemy;
use crate::player::Player;
use crate::map::goal::VictoryTimer;
use crate::enemy::Dead;

pub fn freeze_entities(
    mut query: Query<&mut LinearVelocity, Or<(With<Player>, With<Enemy>)>>,
    dead: Option<Res<Dead>>,
    victory: Option<Res<VictoryTimer>>,
) {
    if dead.is_none() && victory.is_none() {
        return;
    }

    for mut velocity in &mut query {
        velocity.x = 0.0;
        velocity.y = 0.0;
    }
}