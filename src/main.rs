use avian2d::PhysicsPlugins;
use bevy::prelude::*;
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};
use bevy_tokio_tasks::TokioTasksPlugin;
use camera::CameraPlugin;
use clap::Parser;
use door::DoorPlugin;
use integration::integration_plugin::IntegrationPlugin;
use pause::PausePlugin;
#[cfg(feature = "perfmon")]
use perf::PerfPlugin;
use player::PlayerPlugin;
use room::{room_component::WINDOW_WIDTH, RoomPlugin, WINDOW_HEIGHT};
use schedule::SchedulePlugin;
use state::StatePlugin;

mod camera;
mod cli;
mod door;
mod floorplan;
mod integration;
mod pause;
mod perf;
mod player;
mod room;
mod schedule;
mod state;

fn main() {
    cli::Cli::parse();

    App::new()
        .add_plugins((
            TokioTasksPlugin::default(),
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
            SchedulePlugin,
            PhysicsPlugins::default(),
            CameraPlugin,
            RoomPlugin,
            PlayerPlugin,
            DoorPlugin,
            StatePlugin,
            PausePlugin,
            IntegrationPlugin,
            #[cfg(feature = "perfmon")]
            PerfPlugin,
        ))
        .run();
}
