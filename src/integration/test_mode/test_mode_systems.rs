use crate::floorplan::{Door, FloorPlan, FloorPlanEvent, Room};
use bevy::app::AppExit;
use bevy::prelude::*;

const NUMBER_OF_ROOMS: usize = 25;
const NUMBER_OF_DOORS: usize = 250;
const MAX_DOORS_PER_ROOM: usize = 19;

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
            name: format!("Room {i}"),
        })
        .collect();

    // Add rooms to floorplan
    let room_indices: Vec<petgraph::prelude::NodeIndex> = rooms
        .iter()
        .map(|room| floorplan.add_room(room.clone()))
        .collect();

    // Create doors and connect rooms
    let mut connected_rooms: Vec<Vec<usize>> = vec![vec![]; NUMBER_OF_ROOMS];

    (0..NUMBER_OF_DOORS).for_each(|i| {
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

        // Find two rooms that are not yet fully connected and have not exceeded the max doors limit
        let mut room1_index = i % NUMBER_OF_ROOMS;
        let mut room2_index = (i + 1) % NUMBER_OF_ROOMS;

        let mut attempts = 0;
        while (connected_rooms[room1_index].contains(&room2_index)
            || room1_index == room2_index
            || connected_rooms[room1_index].len() >= MAX_DOORS_PER_ROOM
            || connected_rooms[room2_index].len() >= MAX_DOORS_PER_ROOM)
            && attempts < NUMBER_OF_ROOMS * 2
        {
            room2_index = (room2_index + 1) % NUMBER_OF_ROOMS;
            if room2_index == room1_index {
                room1_index = (room1_index + 1) % NUMBER_OF_ROOMS;
            }
            attempts += 1;
        }

        if room1_index != room2_index
            && !connected_rooms[room1_index].contains(&room2_index)
            && connected_rooms[room1_index].len() < MAX_DOORS_PER_ROOM
            && connected_rooms[room2_index].len() < MAX_DOORS_PER_ROOM
        {
            connected_rooms[room1_index].push(room2_index);
            connected_rooms[room2_index].push(room1_index);

            floorplan.add_door(room_indices[room1_index], room_indices[room2_index], door);
        }
    });
    floorplan
}
