use bevy::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState {
    #[default]
    InGame,
    Paused,
    TransitioningOut,
    RoomChange,
    TransitioningIn,
}

#[derive(Component)]
pub struct FadeOverlay;

#[derive(Resource)]
pub struct FadeEffect {
    pub curtain: f32,     // Timer (0.0 = start, 1.0 = ready to open curtain)
    pub alpha: f32,       // Opacity (0.0 = visible, 1.0 = fully black)
    pub fading_out: bool, // Track whether fading in or out
}
