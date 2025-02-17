use avian2d::PhysicsPlugins;
use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use camera::CameraPlugin;
use clap::Parser;
use door::DoorPlugin;
use integration::integration_plugin::IntegrationPlugin;
use pause::PausePlugin;
use player::PlayerPlugin;
use room::{room_component::WINDOW_WIDTH, RoomPlugin, WINDOW_HEIGHT};
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
    cli::Cli::parse();

    App::new()
        .add_plugins((
            EmbeddedAssetPlugin {
                mode: PluginMode::ReplaceDefault,
            },
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Doors".to_string(),
                    resolution: bevy::window::WindowResolution::from((WINDOW_WIDTH, WINDOW_HEIGHT)),
                    ..default()
                }),
                ..default()
            }),
            //FrameTimeDiagnosticsPlugin,
            LogDiagnosticsPlugin::default(),
            PhysicsPlugins::default(),
            CameraPlugin,
            RoomPlugin,
            PlayerPlugin,
            DoorPlugin,
            StatePlugin,
            PausePlugin,
            IntegrationPlugin,
        ))
        .run();
}
