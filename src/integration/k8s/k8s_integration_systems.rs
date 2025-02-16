use std::{error::Error, fs};

use crate::floorplan::{DoorData, FloorPlan, FloorPlanEvent, FloorPlanResult, RoomData};
use bevy::{prelude::*, utils::HashSet};
use jsonpath_lib::select;
use serde_json::json;
use serde_yaml::Value;

fn get_namespaces(yaml_str: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let yaml_value: Value = serde_yaml::from_str(yaml_str)?;
    let json_value = json!(yaml_value);

    let namespaces: HashSet<String> = select(&json_value, "$..metadata.namespace")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    Ok(namespaces.into_iter().collect())
}

fn get_deployment_names(yaml_str: &str, namespace: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let yaml_value: Value = serde_yaml::from_str(yaml_str)?;
    let json_value = json!(yaml_value);

    let query = format!(
        "$..[?(@.kind == 'Deployment' && @.metadata.namespace == '{}')].metadata.name",
        namespace
    );

    let deployments: Vec<String> = select(&json_value, &query)?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    Ok(deployments)
}

fn door_adder(
    plan: &mut FloorPlan,
    room1: &RoomData,
    room2: &RoomData,
    door: &DoorData,
) -> FloorPlanResult<()> {
    plan.add_door(
        plan.get_room_by_id(&room1.id)?,
        plan.get_room_by_id(&room2.id)?,
        door.clone(),
    );
    Ok(())
}

pub fn fire_k8s_file_floorplan_event(mut events: EventWriter<FloorPlanEvent>) {
    if let Ok(floorplan) = generate_k8s_floorplan_from_file() {
        events.send(FloorPlanEvent { floorplan });
    } else {
        warn!("No K8S FlooplanEvent");
    }
}

fn generate_k8s_floorplan_from_file() -> FloorPlanResult<FloorPlan> {
    let mut floorplan = FloorPlan::new();
    if let Ok(yaml_content) = fs::read_to_string("assets/k8s.yaml") {
        // yq query to get a list of namespaces goes here
        let cluster_room = RoomData {
            id: "cluster".to_string(),
            name: "Cluster".to_string(),
        };
        floorplan.add_room(cluster_room.clone());

        let mut door_id = 0;
        if let Ok(namespaces) = get_namespaces(&yaml_content) {
            let namespace_rooms = namespaces.into_iter().map(|namespace| RoomData {
                id: namespace.clone(),
                name: namespace.clone(),
            });
            for namespace_room in namespace_rooms {
                floorplan.add_room(namespace_room.clone());
                let door1 = DoorData {
                    id: door_id.to_string(),
                    name: format!("Door to {}", namespace_room.name),
                };
                door_id += 1;
                door_adder(&mut floorplan, &cluster_room, &namespace_room, &door1)?;
                let door2 = DoorData {
                    id: door_id.to_string(),
                    name: format!("Door to {}", cluster_room.name),
                };
                door_id += 1;
                door_adder(&mut floorplan, &namespace_room, &cluster_room, &door2)?;

                if let Ok(deployments) =
                    get_deployment_names(&yaml_content, namespace_room.name.as_str())
                {
                    info!("Deployments: {:?}", deployments);
                    let deployment_rooms = deployments.into_iter().map(|deployment| RoomData {
                        id: deployment.clone(),
                        name: deployment.clone(),
                    });
                    for deployment_room in deployment_rooms {
                        floorplan.add_room(deployment_room.clone());
                        let door1 = DoorData {
                            id: door_id.to_string(),
                            name: format!("Door to {}", deployment_room.name),
                        };
                        door_id += 1;
                        door_adder(&mut floorplan, &namespace_room, &deployment_room, &door1)?;
                        let door2 = DoorData {
                            id: door_id.to_string(),
                            name: format!("Door to {}", namespace_room.name),
                        };
                        door_id += 1;
                        door_adder(&mut floorplan, &deployment_room, &namespace_room, &door2)?;
                    }
                }
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
