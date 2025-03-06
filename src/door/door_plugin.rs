use bevy::prelude::*;

use crate::{room::room_systems::update_room, schedule::InGameSet, state::GameState::RoomChange};

use super::door_systems::{despawn_existing_platforms, spawn_platforms};

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            OnEnter(RoomChange),
            (despawn_existing_platforms, spawn_platforms)
                .chain()
                .after(update_room)
                .in_set(InGameSet::EntityUpdates),
        );
    }
}
