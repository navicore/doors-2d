use crate::cli;

use bevy::prelude::*;
use clap::Parser;

use super::{k8s::K8sIntegrationPlugin, test_mode::TestModeIntegrationPlugin};

pub struct IntegrationPlugin;

impl Plugin for IntegrationPlugin {
    fn build(&self, app: &mut App) {
        let generator_choise = cli::Cli::parse().room_generator;
        match generator_choise {
            // k8s_file is the default
            None | Some(cli::RoomGeneratorType::K8sFile) => app.add_plugins(K8sIntegrationPlugin),
            _ => app.add_plugins(TestModeIntegrationPlugin),
        };
    }
}
