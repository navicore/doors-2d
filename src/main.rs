use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use camera::CameraPlugin;
use door::DoorPlugin;
use integration::TestModeIntegrationPlugin;
use pause::PausePlugin;
use player::PlayerPlugin;
use room::RoomPlugin;
use state::StatePlugin;

mod camera;
mod cli;
mod door;
mod floorplan;
mod integration;
mod pause;
mod player;
mod room;
mod state;

fn main() {
    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            PhysicsPlugins::default(),
            CameraPlugin,
            RoomPlugin,
            PlayerPlugin,
            DoorPlugin,
            StatePlugin,
            PausePlugin,
            TestModeIntegrationPlugin,
        ))
        .run();
}
