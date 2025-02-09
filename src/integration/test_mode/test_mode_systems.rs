use crate::floorplan::{Door, FloorPlan, FloorPlanEvent, FloorPlanResult, Room};
use bevy::prelude::*;

const NUMBER_OF_ROOMS: usize = 25;
const NUMBER_OF_DOORS: usize = 250;
const MAX_DOORS_PER_ROOM: usize = 19;

pub fn fire_room25_floorplan_event(mut events: EventWriter<FloorPlanEvent>) {
    let floorplan = generate_room25_floorplan();
    events.send(FloorPlanEvent { floorplan });
    info!("Fired 25Room FloorPlanEvent");
}

fn generate_room25_floorplan() -> FloorPlan {
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

            floorplan.add_door(
                room_indices[room1_index],
                room_indices[room2_index],
                door.clone(),
            );
            floorplan.add_door(room_indices[room2_index], room_indices[room1_index], door);
        }
    });
    floorplan
}

pub fn fire_room2_floorplan_event(mut events: EventWriter<FloorPlanEvent>) {
    if let Ok(floorplan) = generate_room2_floorplan() {
        events.send(FloorPlanEvent { floorplan });
        info!("Fired 2Room FloorPlanEvent");
    } else {
        warn!("No 2Room FlooplanEvent");
    }
}

fn generate_room2_floorplan() -> FloorPlanResult<FloorPlan> {
    let mut floorplan = FloorPlan::new();
    let room1 = Room {
        id: "0".to_string(),
        name: "Room 1".to_string(),
    };
    let room2 = Room {
        id: "1".to_string(),
        name: "Room 2".to_string(),
    };
    floorplan.add_room(room1.clone());
    floorplan.add_room(room2.clone());
    let door1 = Door {
        id: "0".to_string(),
        name: "Door 1".to_string(),
    };
    floorplan.add_door(
        floorplan.get_room_by_id(&room1.id)?,
        floorplan.get_room_by_id(&room2.id)?,
        door1.clone(),
    );
    floorplan.add_door(
        floorplan.get_room_by_id(&room2.id)?,
        floorplan.get_room_by_id(&room1.id)?,
        door1,
    );
    if let Err(e) = floorplan.set_start_room(&room1.id) {
        error!("Failed to set start room: {:?}", e);
    }

    Ok(floorplan)
}

pub fn fire_room5_floorplan_event(mut events: EventWriter<FloorPlanEvent>) {
    if let Ok(floorplan) = generate_room5_floorplan() {
        events.send(FloorPlanEvent { floorplan });
        info!("Fired 5Room FloorPlanEvent");
    } else {
        error!("No 5Room FloorPlanEvent");
    }
}

fn generate_room5_floorplan() -> FloorPlanResult<FloorPlan> {
    let mut floorplan = FloorPlan::new();
    let room1 = Room {
        id: "0".to_string(),
        name: "Room 1".to_string(),
    };
    let room2 = Room {
        id: "1".to_string(),
        name: "Room 2".to_string(),
    };
    let room3 = Room {
        id: "2".to_string(),
        name: "Room 3".to_string(),
    };
    let room4 = Room {
        id: "3".to_string(),
        name: "Room 4".to_string(),
    };
    let room5 = Room {
        id: "4".to_string(),
        name: "Room 5".to_string(),
    };
    floorplan.add_room(room1.clone());
    floorplan.add_room(room2.clone());
    floorplan.add_room(room3.clone());
    floorplan.add_room(room4.clone());
    floorplan.add_room(room5.clone());

    let door1 = Door {
        id: "0".to_string(),
        name: "Door 1".to_string(),
    };
    let door2 = Door {
        id: "1".to_string(),
        name: "Door 2".to_string(),
    };
    let door3 = Door {
        id: "2".to_string(),
        name: "Door 3".to_string(),
    };
    let door4 = Door {
        id: "3".to_string(),
        name: "Door 4".to_string(),
    };
    let door5 = Door {
        id: "4".to_string(),
        name: "Door 5".to_string(),
    };
    let door6 = Door {
        id: "5".to_string(),
        name: "Door 6".to_string(),
    };
    let door7 = Door {
        id: "6".to_string(),
        name: "Door 7".to_string(),
    };
    let door8 = Door {
        id: "7".to_string(),
        name: "Door 8".to_string(),
    };

    floorplan.add_door(
        floorplan.get_room_by_id(&room1.id)?,
        floorplan.get_room_by_id(&room2.id)?,
        door1,
    );
    floorplan.add_door(
        floorplan.get_room_by_id(&room2.id)?,
        floorplan.get_room_by_id(&room1.id)?,
        door2,
    );
    floorplan.add_door(
        floorplan.get_room_by_id(&room2.id)?,
        floorplan.get_room_by_id(&room3.id)?,
        door3,
    );
    floorplan.add_door(
        floorplan.get_room_by_id(&room3.id)?,
        floorplan.get_room_by_id(&room2.id)?,
        door4,
    );
    floorplan.add_door(
        floorplan.get_room_by_id(&room1.id)?,
        floorplan.get_room_by_id(&room4.id)?,
        door5,
    );
    floorplan.add_door(
        floorplan.get_room_by_id(&room4.id)?,
        floorplan.get_room_by_id(&room1.id)?,
        door6,
    );
    floorplan.add_door(
        floorplan.get_room_by_id(&room3.id)?,
        floorplan.get_room_by_id(&room5.id)?,
        door7,
    );
    floorplan.add_door(
        floorplan.get_room_by_id(&room5.id)?,
        floorplan.get_room_by_id(&room3.id)?,
        door8,
    );

    Ok(floorplan)
}
