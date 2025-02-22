use crate::floorplan::{FloorPlan, FloorPlanEvent, FloorPlanResult, RoomData};
use crate::integration::k8s_file::k8s_integration_systems::connect_rooms_with_doors;
use bevy::prelude::*;
use kube::{
    api::{Api, ListParams},
    Client,
};
use tokio::runtime::Builder;

use super::k8s_api::get_names;

pub fn fire_k8s_live_floorplan_event(mut events: EventWriter<FloorPlanEvent>) {
    if let Ok(rt) = Builder::new_current_thread().enable_all().build() {
        rt.block_on(async {
            match generate_k8s_floorplan_from_live().await {
                Ok(floorplan) => {
                    events.send(FloorPlanEvent { floorplan });
                }
                Err(e) => {
                    panic!("No K8S FloorPlanEvent: {e:?}");
                }
            }
        });
    } else {
        error!("No K8S runtime created");
    }
}

async fn generate_k8s_floorplan_from_live() -> FloorPlanResult<FloorPlan> {
    let client = Client::try_default()
        .await
        .map_err(|e| crate::floorplan::FloorPlanError::ServiceError(e.to_string()))?;

    let namespaces: Api<k8s_openapi::api::core::v1::Namespace> = Api::all(client);
    let lp = ListParams::default();
    let ns_list = namespaces
        .list(&lp)
        .await
        .map_err(|e| crate::floorplan::FloorPlanError::ServiceError(e.to_string()))?;

    let mut floorplan = FloorPlan::new();

    let cluster_room = RoomData {
        id: "cluster".to_string(),
        name: "Cluster Lobby".to_string(),
    };
    floorplan.add_room(cluster_room.clone());

    let mut door_id = 0;
    for ns in ns_list {
        if let Some(namespace) = ns.metadata.name {
            info!("processing namespace {namespace}");
            let namespace_room = RoomData {
                id: namespace.clone(),
                name: format!("{namespace} NS Hallway"),
            };
            floorplan.add_room(namespace_room.clone());
            connect_rooms_with_doors(&mut floorplan, &cluster_room, &namespace_room, &mut door_id)?;

            for kind in &[
                "Deployment",
                "DaemonSet",
                "ReplicaSet",
                "Service",
                "ConfigMap",
                "Ingress",
                "Pod",
            ] {
                setup_hallway_and_rooms(
                    &mut floorplan,
                    &namespace,
                    &namespace_room,
                    &mut door_id,
                    kind,
                )
                .await?;
            }
        }
    }

    Ok(floorplan)
}

async fn setup_hallway_and_rooms(
    plan: &mut FloorPlan,
    namespace: &str,
    outer_room: &RoomData,
    door_id_generator: &mut usize,
    kind: &str,
) -> FloorPlanResult<()> {
    info!("Setting up {kind} hallway and rooms");
    let client = Client::try_default()
        .await
        .map_err(|e| crate::floorplan::FloorPlanError::ServiceError(e.to_string()))?;

    let hallway = RoomData {
        id: format!("{namespace}-{kind}s"),
        name: format!("{namespace} {kind}s Hallway"),
    };
    plan.add_room(hallway.clone());
    connect_rooms_with_doors(plan, outer_room, &hallway, door_id_generator)?;

    let _ = add_rooms(plan, &client, namespace, &hallway, door_id_generator, kind).await;
    info!("Finished setting up {kind} hallway and rooms");
    Ok(())
}

async fn add_rooms(
    plan: &mut FloorPlan,
    client: &Client,
    namespace: &str,
    outer_room: &RoomData,
    door_id_generator: &mut usize,
    kind: &str,
) -> FloorPlanResult<()> {
    info!("Adding {kind} rooms");
    if let Ok(resources) = get_names(client, kind, namespace).await {
        for r in resources {
            let room = RoomData {
                id: format!("{namespace}-{}-{}", r.kind, r.name),
                name: format!("{} {}", r.kind, r.name),
            };
            plan.add_room(room.clone());
            connect_rooms_with_doors(plan, &room, outer_room, door_id_generator)?;

            if let Some(parent) = r.parent {
                let parent_room_id = format!("{namespace}-{}-{}", parent.kind, parent.name);
                let cplan = plan.clone();
                let parent_room = cplan.get_room_by_id(&parent_room_id);
                if let Ok(parent_room) = parent_room {
                    connect_rooms_with_doors(plan, &room, parent_room, door_id_generator)?;
                } else {
                    warn!("Owner room not found: {parent_room_id}");
                }
            }

            for container in r.children {
                let container_room = RoomData {
                    id: format!("{namespace}-{}-{}-{}", r.kind, "container", container.name),
                    name: format!("{} {}", "container", container.name),
                };
                plan.add_room(container_room.clone());
                connect_rooms_with_doors(plan, &container_room, &room, door_id_generator)?;
                for volume_mount in container.children {
                    let volume_mount_room = RoomData {
                        id: format!(
                            "{namespace}-{}-{}-{}-{}",
                            r.kind, "container", container.name, volume_mount.name
                        ),
                        name: format!("{} {}", "volume mount", volume_mount.name),
                    };
                    plan.add_room(volume_mount_room.clone());
                    connect_rooms_with_doors(
                        plan,
                        &volume_mount_room,
                        &container_room,
                        door_id_generator,
                    )?;
                }
            }
        }
        info!("Finished adding {kind} rooms");
        Ok(())
    } else {
        warn!("No {kind} found in {namespace}");
        Err(crate::floorplan::FloorPlanError::RoomDataNotFound(format!(
            "No {kind} found in {namespace}"
        )))
    }
}
