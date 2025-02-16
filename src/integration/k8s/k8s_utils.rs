use std::{collections::HashSet, error::Error};

use jsonpath_lib::select;

pub fn get_namespaces(json_value: &serde_json::Value) -> Result<Vec<String>, Box<dyn Error>> {
    let namespaces: HashSet<String> = select(json_value, "$..metadata.namespace")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    Ok(namespaces.into_iter().collect())
}

pub fn get_names(
    json_value: &serde_json::Value,
    kind: &str,
    namespace: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let query = format!(
        "$..[?(@.kind == '{kind}' && @.metadata.namespace == '{namespace}')].metadata.name"
    );

    let deployments: Vec<String> = select(json_value, &query)?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    Ok(deployments)
}
