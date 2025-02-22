use crate::cli;

use bevy::prelude::*;
use clap::Parser;

use super::{k8s_file, k8s_live, test_mode};

pub struct IntegrationPlugin;

impl Plugin for IntegrationPlugin {
    fn build(&self, app: &mut App) {
        let generator_choise = cli::Cli::parse().room_generator;
        match generator_choise {
            // k8s_file is the default
            Some(cli::RoomGeneratorType::K8sLive) => {
                app.add_plugins(k8s_live::K8sIntegrationPlugin)
            }
            None | Some(cli::RoomGeneratorType::K8sFile) => {
                app.add_plugins(k8s_file::K8sIntegrationPlugin)
            }
            _ => app.add_plugins(test_mode::TestModeIntegrationPlugin),
        };
    }
}
