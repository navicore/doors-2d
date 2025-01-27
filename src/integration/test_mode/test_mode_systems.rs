use crate::floorplan::{Door, FloorPlan, FloorPlanEvent, Room};
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn fire_floor_plan_event(
    mut events: EventWriter<FloorPlanEvent>,
    mut exit: EventWriter<AppExit>,
) {
    // Create a new FloorPlan instance
    let mut floor_plan = FloorPlan::new();

    // Use the API to add rooms and doors
    let room1 = Room {
        id: "1".to_string(),
        name: "Room 1".to_string(),
    };
    let room2 = Room {
        id: "2".to_string(),
        name: "Room 2".to_string(),
    };

    let first_room_id = &room1.id.clone();
    let room1_index = floor_plan.add_room(room1);
    let room2_index = floor_plan.add_room(room2);
    if let Err(e) = floor_plan.set_start_room(first_room_id) {
        error!("Failed to set start room: {:?}", e);
        exit.send(AppExit::error());
    }

    let door = Door {
        id: "1".to_string(),
        name: "Door 1".to_string(),
    };
    floor_plan.add_door(room1_index, room2_index, door);

    // Fire the event
    events.send(FloorPlanEvent { floor_plan });
}
