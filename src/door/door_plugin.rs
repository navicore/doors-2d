use bevy::prelude::*;

use crate::{room::room_systems::update_room, state::GameState::RoomChange};

use super::door_systems::spawn_platforms;

pub struct DoorPlugin;

impl Plugin for DoorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(RoomChange), spawn_platforms.after(update_room));
    }
}
