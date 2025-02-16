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
}

impl K8sResource {
    pub fn new(name: String, kind: String, owner: Option<Self>) -> Self {
        Self {
            name,
            kind,
            owner: owner.map(Box::new),
        }
    }
}

pub fn get_names(
    json_value: &serde_json::Value,
    kind: &str,
    namespace: &str,
    //) -> Result<Vec<(String, Option<(String, String)>)>, Box<dyn Error>> {
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
            //name.map(|n| (n, owner_reference))
            let owner = owner_reference.map(|(kind, name)| K8sResource::new(name, kind, None));
            name.map(|n| K8sResource::new(n, kind.to_string(), owner))
        })
        .collect();

    Ok(deployments)
}
