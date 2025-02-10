use crate::{cli, floorplan::FloorPlanEvent};

use bevy::prelude::*;
use clap::Parser;

use super::test_mode_systems::{
    fire_room25_floorplan_event, fire_room2_floorplan_event, fire_room5_floorplan_event,
};

pub struct TestModeIntegrationPlugin;

impl Plugin for TestModeIntegrationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<FloorPlanEvent>();

        if cli::Cli::parse().room_generator == Some(cli::RoomGeneratorType::Rooms2) {
            app.add_systems(Startup, fire_room2_floorplan_event);
        } else if cli::Cli::parse().room_generator == Some(cli::RoomGeneratorType::Rooms25) {
            app.add_systems(Startup, fire_room25_floorplan_event);
        } else {
            app.add_systems(Startup, fire_room5_floorplan_event);
        }
    }
}
