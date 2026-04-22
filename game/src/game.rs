use bevy::prelude::*;

use crate::map::goal::VictoryTimer;
use crate::death::Dead;

pub fn can_run_gameplay(
    victory_timer: Option<Res<VictoryTimer>>,
    dead: Option<Res<Dead>>,
) -> bool {
    victory_timer.is_none() && dead.is_none()
}