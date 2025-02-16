use std::fs;

use crate::floorplan::{DoorData, FloorPlan, FloorPlanEvent, FloorPlanResult, RoomData};
use bevy::prelude::*;
use serde_json::json;
use serde_yaml::Value;

use super::k8s_utils::{get_names, get_namespaces};

fn connect_rooms_with_doors(
    plan: &mut FloorPlan,
    room1: &RoomData,
    room2: &RoomData,
    door_id: &mut usize,
) -> FloorPlanResult<()> {
    let door1 = DoorData {
        id: door_id.to_string(),
        name: format!("Door to {}", room2.name),
    };
    *door_id += 1;
    plan.add_door(
        plan.get_room_by_id(&room1.id)?,
        plan.get_room_by_id(&room2.id)?,
        door1,
    );

    let door2 = DoorData {
        id: door_id.to_string(),
        name: format!("Door to {}", room1.name),
    };
    *door_id += 1;
    plan.add_door(
        plan.get_room_by_id(&room2.id)?,
        plan.get_room_by_id(&room1.id)?,
        door2,
    );

    Ok(())
}

fn add_rooms(
    floorplan: &mut FloorPlan,
    json_value: &serde_json::Value,
    namespace: &str,
    outer_room: &RoomData,
    door_id_generator: &mut usize,
    kind: &str,
) -> FloorPlanResult<()> {
    if let Ok(names) = get_names(json_value, kind, namespace) {
        for name in names {
            let room = RoomData {
                id: name.clone(),
                name: name.clone(),
            };
            floorplan.add_room(room.clone());
            connect_rooms_with_doors(floorplan, &room, outer_room, door_id_generator)?;
        }
    }
    Ok(())
}

fn setup_hallway_and_rooms(
    plan: &mut FloorPlan,
    json_value: &serde_json::Value, // might want to pass this in a pre-parsed format: TODO
    namespace: &str,
    outer_room: &RoomData, // will often be the cluster lobby
    door_id_generator: &mut usize,
    kind: &str, // hallways collect similar resources
) -> FloorPlanResult<()> {
    let hallway = RoomData {
        id: format!("{namespace}-{kind}s"),
        name: format!("{namespace} {kind}s Hallway"),
    };
    plan.add_room(hallway.clone());
    connect_rooms_with_doors(plan, outer_room, &hallway, door_id_generator)?;
    add_rooms(
        plan,
        json_value,
        namespace,
        &hallway,
        door_id_generator,
        kind,
    )?;
    Ok(())
}

fn generate_k8s_floorplan_from_file() -> FloorPlanResult<FloorPlan> {
    let mut floorplan = FloorPlan::new();
    if let Ok(yaml_content) = fs::read_to_string("assets/k8s.yaml") {
        if let Ok(yaml_value) = serde_yaml::from_str::<Value>(&yaml_content) {
            let json_value = json!(yaml_value);

            let cluster_room = RoomData {
                id: "cluster".to_string(),
                name: "Cluster Lobby".to_string(),
            };
            floorplan.add_room(cluster_room.clone());

            let mut door_id = 0;
            if let Ok(namespaces) = get_namespaces(&json_value) {
                for namespace in namespaces {
                    let namespace_room = RoomData {
                        id: namespace.clone(),
                        name: format!("{namespace} NS Hallway"),
                    };
                    floorplan.add_room(namespace_room.clone());
                    connect_rooms_with_doors(
                        &mut floorplan,
                        &cluster_room,
                        &namespace_room,
                        &mut door_id,
                    )?;

                    for kind in &[
                        "Pod",
                        "Deployment",
                        "DaemonSet",
                        "Replicaset",
                        "Service",
                        "ConfigMap",
                    ] {
                        setup_hallway_and_rooms(
                            &mut floorplan,
                            &json_value,
                            &namespace,
                            &namespace_room,
                            &mut door_id,
                            kind,
                        )?;
                    }
                }
            }
        } else {
            panic!("Failed to parse k8s yaml content");
        }

        Ok(floorplan)
    } else {
        error!("No k8s yaml file found");
        Err(crate::floorplan::FloorPlanError::RoomDataNotFound(
            "no file".to_string(),
        ))
    }
}

pub fn fire_k8s_file_floorplan_event(mut events: EventWriter<FloorPlanEvent>) {
    if let Ok(floorplan) = generate_k8s_floorplan_from_file() {
        events.send(FloorPlanEvent { floorplan });
    } else {
        warn!("No K8S FloorPlanEvent");
    }
}
