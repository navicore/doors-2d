use bevy::prelude::*;

use super::platform_systems::spawn_platforms;

/// create platforms for the player to jump on.  platforms tend to have doors on top of them.
pub struct PlatformPlugin;

impl Plugin for PlatformPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_platforms);
    }
}
