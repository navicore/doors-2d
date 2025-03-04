use bevy::prelude::*;
use iyes_perf_ui::prelude::PerfUiAllEntries;
use iyes_perf_ui::prelude::PerfUiRoot;

pub fn toggle(
    mut commands: Commands,
    q_root: Query<Entity, With<PerfUiRoot>>,
    kbd: Res<ButtonInput<KeyCode>>,
) {
    if kbd.just_pressed(KeyCode::F12) {
        if let Ok(e) = q_root.get_single() {
            // despawn the existing Perf UI
            commands.entity(e).despawn_recursive();
        } else {
            // create a simple Perf UI with default settings
            // and all entries provided by the crate:
            commands.spawn(PerfUiAllEntries::default());
        }
    }
}
