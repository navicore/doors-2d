use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    TransitioningOut,
    TransitioningIn,
}

#[derive(Debug, Resource)]
pub struct Transition {
    pub progress: f32,

    #[allow(dead_code)]
    pub captured_image: Option<Handle<Image>>,
}

impl Default for Transition {
    fn default() -> Self {
        Self {
            progress: 0.0,
            captured_image: None,
        }
    }
}
