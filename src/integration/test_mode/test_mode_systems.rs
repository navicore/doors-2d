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

fn create_rooms(lim: u8) -> Vec<Room> {
    (0..lim)
        .map(|i| Room {
            id: i.to_string(),
            name: format!("Room {}", i + 1),
        })
        .collect()
}

fn create_doors(lim: u8) -> Vec<Door> {
    (0..lim)
        .map(|i| Door {
            id: i.to_string(),
            name: format!("Door {}", i + 1),
        })
        .collect()
}

// create a 2 room floorplan
fn generate_room2_floorplan() -> FloorPlanResult<FloorPlan> {
    let mut floorplan = FloorPlan::new();
    let rooms = create_rooms(2);
    let room1 = &rooms[0];
    let room2 = &rooms[1];

    floorplan.add_room(room1.clone());
    floorplan.add_room(room2.clone());

    let doors = create_doors(2);

    door_adder(&mut floorplan, room1, room2, &doors[0].clone())?;
    door_adder(&mut floorplan, room2, room1, &doors[1].clone())?;

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

// create a 2 room floorplan
fn generate_room5_floorplan() -> FloorPlanResult<FloorPlan> {
    let mut floorplan = FloorPlan::new();

    fn create_rooms(lim: u8) -> Vec<Room> {
        (0..lim)
            .map(|i| Room {
                id: i.to_string(),
                name: format!("Room {}", i + 1),
            })
            .collect()
    }
    let rooms = create_rooms(5);
    let room1 = &rooms[0];
    let room2 = &rooms[1];
    let room3 = &rooms[2];
    let room4 = &rooms[3];
    let room5 = &rooms[4];

    floorplan.add_room(room1.clone());
    floorplan.add_room(room2.clone());
    floorplan.add_room(room3.clone());
    floorplan.add_room(room4.clone());
    floorplan.add_room(room5.clone());

    fn create_doors(lim: u8) -> Vec<Door> {
        (0..lim)
            .map(|i| Door {
                id: i.to_string(),
                name: format!("Door {}", i + 1),
            })
            .collect()
    }
    let doors = create_doors(10);

    door_adder(&mut floorplan, room1, room2, &doors[0].clone())?;
    door_adder(&mut floorplan, room2, room1, &doors[1].clone())?;

    door_adder(&mut floorplan, room2, room3, &doors[2].clone())?;
    door_adder(&mut floorplan, room3, room2, &doors[3].clone())?;

    door_adder(&mut floorplan, room1, room4, &doors[4].clone())?;
    door_adder(&mut floorplan, room4, room1, &doors[5].clone())?;

    door_adder(&mut floorplan, room3, room5, &doors[6].clone())?;
    door_adder(&mut floorplan, room5, room3, &doors[7].clone())?;

    door_adder(&mut floorplan, room3, room4, &doors[8].clone())?;
    door_adder(&mut floorplan, room4, room3, &doors[9].clone())?;

    Ok(floorplan)
}

fn door_adder(
    plan: &mut FloorPlan,
    room1: &Room,
    room2: &Room,
    door: &Door,
) -> FloorPlanResult<()> {
    plan.add_door(
        plan.get_room_by_id(&room1.id)?,
        plan.get_room_by_id(&room2.id)?,
        door.clone(),
    );
    Ok(())
}
