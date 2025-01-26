use crate::camera::camera_systems::{follow_player, spawn_camera};
use crate::schedule::InGameSet;
use bevy::prelude::*;

/// a 2D camera that follows the player perpendicularly
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_systems(Update, follow_player.in_set(InGameSet::EntityUpdates));
    }
}
