use bevy::prelude::*;

use crate::scheduler::InGameSet;

use super::{
    environ_component::{CurrentFloorPlan, EnvironState, WINDOW_HEIGHT, WINDOW_WIDTH},
    environ_systems::{handle_floor_plan_changes, setup_environment},
};
/// define the game window size and environment constants and create the left and right walls, the
/// ground, and the top boundary.
pub struct EnvironPlugin;

impl Plugin for EnvironPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentFloorPlan::default())
            .insert_resource(EnvironState::default())
            .add_plugins(DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Kubernetes Platformer".to_string(),
                    resolution: bevy::window::WindowResolution::from((WINDOW_WIDTH, WINDOW_HEIGHT)),
                    ..default()
                }),
                ..default()
            }))
            .add_systems(Startup, setup_environment)
            .add_systems(
                Update,
                handle_floor_plan_changes.in_set(InGameSet::EntityUpdates),
            );
    }
}
