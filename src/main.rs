use avian2d::prelude::*;
use bevy::prelude::*;
use camera::CameraPlugin;
use environ::{EnvironPlugin, WINDOW_HEIGHT, WINDOW_WIDTH};
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
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Kubernetes Platformer".to_string(),
                    resolution: bevy::window::WindowResolution::from((WINDOW_WIDTH, WINDOW_HEIGHT)),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            MovementPlugin,
            PlayerPlugin,
            SchedulePlugin,
            StatePlugin,
            EnvironPlugin,
            PlatformPlugin,
            CameraPlugin,
        ))
        .run();
}
