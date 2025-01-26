use bevy::prelude::*;

use super::environ_systems::{setup_environment, WINDOW_HEIGHT, WINDOW_WIDTH};
/// define the game window size and environment constants and create the left and right walls, the
/// ground, and the top boundary.
pub struct EnvironPlugin;

impl Plugin for EnvironPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Kubernetes Platformer".to_string(),
                resolution: bevy::window::WindowResolution::from((WINDOW_WIDTH, WINDOW_HEIGHT)),
                ..default()
            }),
            ..default()
        }))
        .add_systems(Startup, setup_environment);
    }
}
