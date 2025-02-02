use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use camera::CameraPlugin;
use integration::TestModeIntegrationPlugin;
use platform::PlatformPlugin;
use player::PlayerPlugin;
use room::RoomPlugin;
use scheduler::SchedulePlugin;
use state::StatePlugin;

mod camera;
mod floorplan;
mod integration;
mod platform;
mod player;
mod room;
mod scheduler;
mod state;

fn main() {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            RoomPlugin,
            PhysicsPlugins::default(),
            PlayerPlugin,
            SchedulePlugin,
            StatePlugin,
            PlatformPlugin,
            CameraPlugin,
            TestModeIntegrationPlugin,
        ))
        .run();
}
