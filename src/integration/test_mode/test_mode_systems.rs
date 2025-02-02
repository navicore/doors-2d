use crate::floorplan::{Door, FloorPlan, FloorPlanEvent, Room};
use bevy::app::AppExit;
use bevy::prelude::*;

const NUMBER_OF_ROOMS: usize = 15;
const NUMBER_OF_DOORS: usize = 250;

pub fn fire_floorplan_event(mut events: EventWriter<FloorPlanEvent>, exit: EventWriter<AppExit>) {
    let floorplan = generate_test_floorplan(exit);
    events.send(FloorPlanEvent { floorplan });
}

fn generate_test_floorplan(mut exit: EventWriter<AppExit>) -> FloorPlan {
    let mut floorplan = FloorPlan::new();

    // Create rooms
    let rooms: Vec<Room> = (0..NUMBER_OF_ROOMS)
        .map(|i| Room {
            id: i.to_string(),
            name: format!("Room {}", i + 1),
        })
        .collect();

    // Add rooms to floorplan
    let room_indices: Vec<petgraph::prelude::NodeIndex> = rooms
        .iter()
        .map(|room| floorplan.add_room(room.clone()))
        .collect();

    // Create doors and connect rooms
    for i in 0..NUMBER_OF_DOORS {
        let door = Door {
            id: i.to_string(),
            name: format!("Door {}", i + 1),
        };

        if i == 0 {
            let first_room_id = &rooms[i].id.clone();
            if let Err(e) = floorplan.set_start_room(first_room_id) {
                error!("Failed to set start room: {:?}", e);
                exit.send(AppExit::error());
            }
        }

        // Connect rooms in a simple linear fashion for now
        let room1_index = room_indices[i % NUMBER_OF_ROOMS];
        let room2_index = room_indices[(i + 1) % NUMBER_OF_ROOMS];

        floorplan.add_door(room1_index, room2_index, door);
    }
    floorplan
}
