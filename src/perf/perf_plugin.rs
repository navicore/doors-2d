use super::perf_component::{
    RoomDoorCount, TimeSinceLastFloorplanModified, TimeSinceLastFloorplanRefresh, WorldEdgeCount,
    WorldNodeCount,
};
use super::perf_system::{toggle_builtins, toggle_customs};
use bevy::diagnostic::{
    EntityCountDiagnosticsPlugin, FrameTimeDiagnosticsPlugin, SystemInformationDiagnosticsPlugin,
};
use bevy::prelude::*;
use bevy::render::diagnostic::RenderDiagnosticsPlugin;
use iyes_perf_ui::prelude::*;

#[allow(dead_code)]
pub struct PerfPlugin;
impl Plugin for PerfPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            FrameTimeDiagnosticsPlugin,
            EntityCountDiagnosticsPlugin,
            SystemInformationDiagnosticsPlugin, // does not work with dynamic linking
            RenderDiagnosticsPlugin,
        ))
        .add_perf_ui_simple_entry::<RoomDoorCount>()
        .add_perf_ui_simple_entry::<WorldNodeCount>()
        .add_perf_ui_simple_entry::<WorldEdgeCount>()
        .add_perf_ui_simple_entry::<TimeSinceLastFloorplanRefresh>()
        .add_perf_ui_simple_entry::<TimeSinceLastFloorplanModified>()
        .add_plugins(PerfUiPlugin)
        .add_systems(
            Update,
            toggle_builtins.before(iyes_perf_ui::PerfUiSet::Setup),
        )
        .add_systems(
            Update,
            toggle_customs.before(iyes_perf_ui::PerfUiSet::Setup),
        );
    }
}
