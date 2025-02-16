use std::fs;

use crate::floorplan::{DoorData, FloorPlan, FloorPlanEvent, FloorPlanResult, RoomData};
use bevy::prelude::*;

use super::k8s_utils::{get_deployment_names, get_namespaces};

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

fn handle_deployments(
    floorplan: &mut FloorPlan,
    yaml_content: &str,
    namespace_room: &RoomData,
    door_id: &mut usize,
) -> FloorPlanResult<()> {
    if let Ok(deployments) = get_deployment_names(yaml_content, &namespace_room.name) {
        for deployment in deployments {
            let deployment_room = RoomData {
                id: deployment.clone(),
                name: deployment.clone(),
            };
            floorplan.add_room(deployment_room.clone());
            connect_rooms_with_doors(floorplan, namespace_room, &deployment_room, door_id)?;
        }
    }
    Ok(())
}

pub fn fire_k8s_file_floorplan_event(mut events: EventWriter<FloorPlanEvent>) {
    if let Ok(floorplan) = generate_k8s_floorplan_from_file() {
        events.send(FloorPlanEvent { floorplan });
    } else {
        warn!("No K8S FloorPlanEvent");
    }
}

fn generate_k8s_floorplan_from_file() -> FloorPlanResult<FloorPlan> {
    let mut floorplan = FloorPlan::new();
    if let Ok(yaml_content) = fs::read_to_string("assets/k8s.yaml") {
        let cluster_room = RoomData {
            id: "cluster".to_string(),
            name: "Cluster".to_string(),
        };
        floorplan.add_room(cluster_room.clone());

        let mut door_id = 0;
        if let Ok(namespaces) = get_namespaces(&yaml_content) {
            for namespace in namespaces {
                let namespace_room = RoomData {
                    id: namespace.clone(),
                    name: namespace.clone(),
                };
                floorplan.add_room(namespace_room.clone());
                connect_rooms_with_doors(
                    &mut floorplan,
                    &cluster_room,
                    &namespace_room,
                    &mut door_id,
                )?;
                handle_deployments(&mut floorplan, &yaml_content, &namespace_room, &mut door_id)?;
                // Add similar functions for replicasets, pods, and containers here
            }
        }

        Ok(floorplan)
    } else {
        error!("No k8s yaml file found");
        Err(crate::floorplan::FloorPlanError::RoomDataNotFound(
            "no file".to_string(),
        ))
    }
}
