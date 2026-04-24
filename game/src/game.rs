use bevy::prelude::*;

use crate::map::goal::VictoryTimer;
use crate::death::DeathTimer;
use crate::gamestate::GameState;

pub fn can_run_gameplay(
    victory_timer: Option<Res<VictoryTimer>>,
    dead: Option<Res<DeathTimer>>,
) -> bool {
    victory_timer.is_none() && dead.is_none()
}

pub fn return_to_menu(
    keys: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    if keys.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::Menu);
    }
}