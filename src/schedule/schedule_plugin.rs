use bevy::prelude::*;

use super::schedule_component::InGameSet;

pub struct SchedulePlugin;

impl Plugin for SchedulePlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (InGameSet::Update, InGameSet::Render).chain());
        // .add_systems(
        //     Update,
        //     apply_deferred
        //         .after(InGameSet::DespawnEntities)
        //         .before(InGameSet::UserInput),
        // );
    }
}
