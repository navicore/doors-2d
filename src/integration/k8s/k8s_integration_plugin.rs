use crate::cli;

use bevy::prelude::*;
use clap::Parser;

use super::k8s_integration_systems::fire_k8s_file_floorplan_event;

pub struct K8sIntegrationPlugin;

impl Plugin for K8sIntegrationPlugin {
    fn build(&self, app: &mut App) {
        let generator_choise = cli::Cli::parse().room_generator;
        match generator_choise {
            Some(cli::RoomGeneratorType::Rooms2) => {
                //todo: don't double check this in each
                //integration plugin :(
                //noop
            }
            Some(cli::RoomGeneratorType::Rooms5) => {
                //noop
            }
            Some(cli::RoomGeneratorType::Rooms25) => {
                //noop
            }
            _ => {
                // default across all plugins
                app.add_systems(Startup, fire_k8s_file_floorplan_event);
            }
        }
    }
}
