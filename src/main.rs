use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use camera::CameraPlugin;
use environ::EnvironPlugin;
use movement::MovementPlugin;
use platform::PlatformPlugin;
use player::PlayerPlugin;
use scheduler::SchedulePlugin;
use state::StatePlugin;

mod camera;
mod environ;
mod floorplan;
mod movement;
mod platform;
mod player;
mod scheduler;
mod state;

fn main() {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            EnvironPlugin,
            PhysicsPlugins::default(),
            MovementPlugin,
            PlayerPlugin,
            SchedulePlugin,
            StatePlugin,
            PlatformPlugin,
            CameraPlugin,
        ))
        .run();
}
