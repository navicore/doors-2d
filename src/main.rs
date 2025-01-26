use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use camera::CameraPlugin;
use environ::EnvironPlugin;
use movement::MovementPlugin;
use platform::PlatformPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;

mod camera;
mod environ;
mod movement;
mod platform;
mod player;
mod schedule;
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
