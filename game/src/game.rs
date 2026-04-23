use bevy::prelude::*;

use crate::map::goal::VictoryTimer;
use crate::death::DeathTimer;

pub fn can_run_gameplay(
    victory_timer: Option<Res<VictoryTimer>>,
    dead: Option<Res<DeathTimer>>,
) -> bool {
    victory_timer.is_none() && dead.is_none()
}