use avian2d::prelude::*;
use bevy::prelude::*;
use environ::EnvironPlugin;
use movement::MovementPlugin;
use player::PlayerPlugin;
use schedule::SchedulePlugin;
use state::StatePlugin;

mod environ;
mod movement;
mod player;
mod schedule;
mod state;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Kubernetes Platformer".to_string(),
                    resolution: (1200.0, 800.0).into(),
                    ..default()
                }),
                ..default()
            }),
            PhysicsPlugins::default(),
            MovementPlugin,
            PlayerPlugin,
            SchedulePlugin,
            StatePlugin,
            EnvironPlugin, // Add the new environment plugin
        ))
        .run();
}
