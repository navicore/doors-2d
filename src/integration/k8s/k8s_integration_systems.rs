use std::fs;

use crate::floorplan::{DoorData, FloorPlan, FloorPlanEvent, FloorPlanResult, RoomData};
use bevy::prelude::*;

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

fn handle_pods(
    floorplan: &mut FloorPlan,
    yaml_content: &str,
    namespace_room: &RoomData,
    door_id: &mut usize,
) -> FloorPlanResult<()> {
    if let Ok(pods) = get_names(yaml_content, "Pod", &namespace_room.name) {
        for pod in pods {
            let pod_room = RoomData {
                id: pod.clone(),
                name: pod.clone(),
            };
            floorplan.add_room(pod_room.clone());
            connect_rooms_with_doors(floorplan, namespace_room, &pod_room, door_id)?;
        }
    }
    Ok(())
}

fn handle_replicasets(
    floorplan: &mut FloorPlan,
    yaml_content: &str,
    namespace_room: &RoomData,
    door_id: &mut usize,
) -> FloorPlanResult<()> {
    if let Ok(replicasets) = get_names(yaml_content, "ReplicaSet", &namespace_room.name) {
        for replicaset in replicasets {
            let replicaset_room = RoomData {
                id: replicaset.clone(),
                name: replicaset.clone(),
            };
            floorplan.add_room(replicaset_room.clone());
            connect_rooms_with_doors(floorplan, namespace_room, &replicaset_room, door_id)?;
        }
    }
    Ok(())
}

fn handle_deployments(
    floorplan: &mut FloorPlan,
    yaml_content: &str,
    namespace_room: &RoomData,
    door_id: &mut usize,
) -> FloorPlanResult<()> {
    if let Ok(deployments) = get_names(yaml_content, "Deployment", &namespace_room.name) {
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

                let deployment_room = RoomData {
                    id: format!("{}-deployments", namespace),
                    name: format!("{} Deployments", namespace),
                };
                floorplan.add_room(deployment_room.clone());
                connect_rooms_with_doors(
                    &mut floorplan,
                    &namespace_room,
                    &deployment_room,
                    &mut door_id,
                )?;
                handle_deployments(
                    &mut floorplan,
                    &yaml_content,
                    &deployment_room,
                    &mut door_id,
                )?;

                let replicaset_room = RoomData {
                    id: format!("{}-replicasets", namespace),
                    name: format!("{} ReplicaSets", namespace),
                };
                floorplan.add_room(replicaset_room.clone());
                connect_rooms_with_doors(
                    &mut floorplan,
                    &namespace_room,
                    &replicaset_room,
                    &mut door_id,
                )?;
                handle_replicasets(
                    &mut floorplan,
                    &yaml_content,
                    &replicaset_room,
                    &mut door_id,
                )?;

                let pod_room = RoomData {
                    id: format!("{}-pods", namespace),
                    name: format!("{} pods", namespace),
                };
                floorplan.add_room(pod_room.clone());
                connect_rooms_with_doors(&mut floorplan, &namespace_room, &pod_room, &mut door_id)?;

                //TODO: bug replicaset and pods don't create any rooms
                handle_pods(&mut floorplan, &yaml_content, &pod_room, &mut door_id)?;
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
