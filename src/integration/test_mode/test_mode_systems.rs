use crate::floorplan::{Door, FloorPlan, FloorPlanEvent, Room};
use bevy::app::AppExit;
use bevy::prelude::*;

pub fn fire_floorplan_event(
    mut events: EventWriter<FloorPlanEvent>,
    mut exit: EventWriter<AppExit>,
) {
    // Create a new FloorPlan instance
    let mut floorplan = FloorPlan::new();

    // Use the API to add rooms and doors
    let room1 = Room {
        id: "1".to_string(),
        name: "Room 1".to_string(),
    };
    let room2 = Room {
        id: "2".to_string(),
        name: "Room 2".to_string(),
    };
    let room3 = Room {
        id: "3".to_string(),
        name: "Room 3".to_string(),
    };
    let room4 = Room {
        id: "4".to_string(),
        name: "Room 4".to_string(),
    };

    let first_room_id = &room1.id.clone();
    let room1_index = floorplan.add_room(room1);
    let room2_index = floorplan.add_room(room2);
    let room3_index = floorplan.add_room(room3);
    let room4_index = floorplan.add_room(room4);
    if let Err(e) = floorplan.set_start_room(first_room_id) {
        error!("Failed to set start room: {:?}", e);
        exit.send(AppExit::error());
    }

    let door = Door {
        id: "1".to_string(),
        name: "Door 1".to_string(),
    };
    floorplan.add_door(room1_index, room2_index, door);
    let door2 = Door {
        id: "2".to_string(),
        name: "Door 2".to_string(),
    };
    floorplan.add_door(room1_index, room3_index, door2);

    let door3 = Door {
        id: "3".to_string(),
        name: "Door 3".to_string(),
    };
    floorplan.add_door(room1_index, room4_index, door3);

    // Fire the event
    events.send(FloorPlanEvent { floorplan });
}
