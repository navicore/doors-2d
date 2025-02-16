use bevy::prelude::*;

use crate::{floorplan::FloorPlanEvent, state::GameState::RoomChange};

use super::{
    room_component::{CurrentFloorPlan, RoomState},
    room_systems::{handle_floor_plan_changes, setup_room, update_doors, update_room},
};
/// define the game window size and environment constants and create the left and right walls, the
/// ground, and the top boundary.
pub struct RoomPlugin;

impl Plugin for RoomPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(CurrentFloorPlan::default())
            .add_event::<FloorPlanEvent>()
            .insert_resource(RoomState::default())
            .add_systems(Startup, setup_room)
            .add_systems(Update, handle_floor_plan_changes)
            .add_systems(OnEnter(RoomChange), (update_doors, update_room).chain());
    }
}
