use bevy::prelude::*;
use iyes_perf_ui::prelude::PerfUiAllEntries;
use iyes_perf_ui::prelude::PerfUiRoot;

use super::perf_component::RoomDoorCount;
use super::perf_component::TimeSinceLastFloorplanModified;
use super::perf_component::TimeSinceLastFloorplanRefresh;
use super::{WorldEdgeCount, WorldNodeCount};

pub fn toggle_builtins(
    mut commands: Commands,
    q_root: Query<Entity, With<PerfUiRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        if let Ok(e) = q_root.get_single() {
            commands.entity(e).despawn_recursive();
        } else {
            commands.spawn((PerfUiAllEntries::default(),));
        }
    }
}

pub fn toggle_customs(
    mut commands: Commands,
    q_root: Query<Entity, With<PerfUiRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F10) {
        if let Ok(e) = q_root.get_single() {
            commands.entity(e).despawn_recursive();
        } else {
            commands.spawn((
                RoomDoorCount::default(),
                WorldNodeCount::default(),
                WorldEdgeCount::default(),
                TimeSinceLastFloorplanRefresh::default(),
                TimeSinceLastFloorplanModified::default(),
            ));
        }
    }
}
