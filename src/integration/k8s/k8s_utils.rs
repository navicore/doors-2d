use std::{collections::HashSet, error::Error};

use jsonpath_lib::select;

pub fn get_namespaces(json_value: &serde_json::Value) -> Result<Vec<String>, Box<dyn Error>> {
    let namespaces: HashSet<String> = select(json_value, "$..metadata.namespace")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    Ok(namespaces.into_iter().collect())
}

pub struct K8sResource {
    pub name: String,
    pub kind: String,
    pub owner: Option<Box<K8sResource>>,
    pub containers: Vec<String>,
}

impl K8sResource {
    pub fn new(name: String, kind: String, owner: Option<Self>, containers: Vec<String>) -> Self {
        Self {
            name,
            kind,
            owner: owner.map(Box::new),
            containers,
        }
    }
}

pub fn get_names(
    json_value: &serde_json::Value,
    kind: &str,
    namespace: &str,
) -> Result<Vec<K8sResource>, Box<dyn Error>> {
    let query = format!("$..[?(@.kind == '{kind}' && @.metadata.namespace == '{namespace}')]");

    let deployments: Vec<K8sResource> = select(json_value, &query)?
        .iter()
        .filter_map(|v| {
            let name = v["metadata"]["name"].as_str().map(String::from);
            let owner_reference = v["metadata"]["ownerReferences"]
                .as_array()
                .and_then(|refs| refs.first())
                .and_then(|owner_ref| {
                    let owner_kind = owner_ref["kind"].as_str().map(String::from);
                    let owner_name = owner_ref["name"].as_str().map(String::from);
                    match (owner_kind, owner_name) {
                        (Some(kind), Some(name)) => Some((kind, name)),
                        _ => None,
                    }
                });
            let containers = v["spec"]["containers"]
                .as_array()
                .map(|containers| {
                    containers
                        .iter()
                        .filter_map(|container| container["name"].as_str().map(String::from))
                        .collect()
                })
                .unwrap_or_default();
            let owner =
                owner_reference.map(|(kind, name)| K8sResource::new(name, kind, None, Vec::new()));
            name.map(|n| K8sResource::new(n, kind.to_string(), owner, containers))
        })
        .collect();

    Ok(deployments)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_get_names_with_owner_and_containers() {
        let json_value = json!({
            "items": [
                {
                    "kind": "Pod",
                    "metadata": {
                        "name": "pod1",
                        "namespace": "default",
                        "ownerReferences": [
                            {
                                "kind": "ReplicaSet",
                                "name": "rs1"
                            }
                        ]
                    },
                    "spec": {
                        "containers": [
                            {
                                "name": "container1"
                            },
                            {
                                "name": "container2"
                            }
                        ]
                    }
                },
                {
                    "kind": "Pod",
                    "metadata": {
                        "name": "pod2",
                        "namespace": "default"
                    },
                    "spec": {
                        "containers": [
                            {
                                "name": "container3"
                            }
                        ]
                    }
                }
            ]
        });

        let result = get_names(&json_value, "Pod", "default").unwrap();
        assert_eq!(result.len(), 2);

        let pod1 = &result[0];
        assert_eq!(pod1.name, "pod1");
        assert_eq!(pod1.kind, "Pod");
        assert!(pod1.owner.is_some());
        let owner = pod1.owner.as_ref().unwrap();
        assert_eq!(owner.name, "rs1");
        assert_eq!(owner.kind, "ReplicaSet");
        assert_eq!(pod1.containers, vec!["container1", "container2"]);

        let pod2 = &result[1];
        assert_eq!(pod2.name, "pod2");
        assert_eq!(pod2.kind, "Pod");
        assert!(pod2.owner.is_none());
        assert_eq!(pod2.containers, vec!["container3"]);
    }

    #[test]
    fn test_get_names_without_owner_and_containers() {
        let json_value = json!({
            "items": [
                {
                    "kind": "Service",
                    "metadata": {
                        "name": "service1",
                        "namespace": "default"
                    }
                }
            ]
        });

        let result = get_names(&json_value, "Service", "default").unwrap();
        assert_eq!(result.len(), 1);

        let service = &result[0];
        assert_eq!(service.name, "service1");
        assert_eq!(service.kind, "Service");
        assert!(service.owner.is_none());
        assert!(service.containers.is_empty());
    }
}
