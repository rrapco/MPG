use bevy::prelude::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    #[default]
    Menu,
    LoadingLevel,
    InGame,
}

#[derive(Component)]
pub struct InGameEntity;