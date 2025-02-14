use crate::camera::camera_systems::{follow_player, spawn_camera};
use bevy::{color::palettes::tailwind::GRAY_200, prelude::*};
use bevy_lit::prelude::Lighting2dPlugin;

/// a 2D camera that follows the player perpendicularly
pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Lighting2dPlugin)
            .insert_resource(ClearColor(Color::from(GRAY_200)))
            .add_systems(Startup, spawn_camera)
            .add_systems(Update, follow_player);
    }
}
