use crate::integration::integration_utils::IntegrationResource;
use kube::core::{ApiResource, DynamicObject, GroupVersionKind};
use kube::{
    api::{Api, ListParams},
    Client,
};
use std::error::Error;

pub async fn get_names(
    client: &Client,
    kind: &str,
    namespace: &str,
) -> Result<Vec<IntegrationResource>, Box<dyn Error>> {
    let gvk = GroupVersionKind::gvk("v1", "", kind);
    let resource = ApiResource::from_gvk(&gvk);
    let api: Api<DynamicObject> = Api::namespaced_with(client.clone(), namespace, &resource);
    let lp = ListParams::default();
    let resource_list = api
        .list(&lp)
        .await
        .map_err(|e| format!("Failed to list resources: {e}"))?;

    let mut resources = Vec::new();
    for resource in resource_list {
        let name = resource.metadata.name.clone();
        let owner_reference = get_owner_reference(&resource);
        let containers = get_containers(&resource);
        let owner = owner_reference
            .map(|(kind, name)| IntegrationResource::new(name, kind, None, Vec::new()));
        if let Some(name) = name {
            resources.push(IntegrationResource::new(
                name,
                kind.to_string(),
                owner,
                containers,
            ));
        }
    }

    Ok(resources)
}

fn get_owner_reference(v: &DynamicObject) -> Option<(String, String)> {
    v.metadata
        .owner_references
        .as_ref()
        .and_then(|refs| refs.first())
        .map(|owner_ref| (owner_ref.kind.clone(), owner_ref.name.clone()))
}

fn get_containers(v: &DynamicObject) -> Vec<IntegrationResource> {
    v.data["spec"]["containers"]
        .as_array()
        .map(|containers| {
            containers
                .iter()
                .filter_map(|container| {
                    let container_name = container["name"].as_str().map(String::from);
                    let volume_mounts = get_volume_mounts(container);
                    container_name.map(|n| IntegrationResource {
                        name: n,
                        kind: "Container".to_string(),
                        parent: None,
                        children: volume_mounts,
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}

fn get_volume_mounts(container: &serde_json::Value) -> Vec<IntegrationResource> {
    container["volumeMounts"]
        .as_array()
        .map(|volume_mounts| {
            volume_mounts
                .iter()
                .filter_map(|volume_mount| {
                    volume_mount["name"].as_str().map(|n| IntegrationResource {
                        name: n.to_string(),
                        kind: "VolumeMount".to_string(),
                        parent: None,
                        children: Vec::new(),
                    })
                })
                .collect()
        })
        .unwrap_or_default()
}
