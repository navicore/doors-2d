use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    TransitioningOut,
    TransitioningIn,
}

#[derive(Debug, Component)]
pub struct PausedText;

#[derive(Debug, Resource)]
pub struct Transition {
    pub progress: f32,
}

impl Default for Transition {
    fn default() -> Self {
        Self { progress: 0.0 }
    }
}
