use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use iyes_perf_ui::PerfUiPlugin;

use super::perf_system::toggle;

pub struct PerfPlugin;
impl Plugin for PerfPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin, // does not work with dynamic linking
            RenderDiagnosticsPlugin,
        ))
        .add_plugins(PerfUiPlugin)
        .add_systems(Update, toggle.before(iyes_perf_ui::PerfUiSet::Setup));
    }
}
