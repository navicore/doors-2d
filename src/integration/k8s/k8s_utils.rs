use std::{collections::HashSet, error::Error};

use jsonpath_lib::select;
use serde_json::json;
use serde_yaml::Value;

pub fn get_namespaces(yaml_str: &str) -> Result<Vec<String>, Box<dyn Error>> {
    let yaml_value: Value = serde_yaml::from_str(yaml_str)?;
    let json_value = json!(yaml_value);

    let namespaces: HashSet<String> = select(&json_value, "$..metadata.namespace")?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    Ok(namespaces.into_iter().collect())
}

pub fn get_names(
    yaml_str: &str,
    kind: &str,
    namespace: &str,
) -> Result<Vec<String>, Box<dyn Error>> {
    let yaml_value: Value = serde_yaml::from_str(yaml_str)?;
    let json_value = json!(yaml_value);

    let query = format!(
        "$..[?(@.kind == '{kind}' && @.metadata.namespace == '{namespace}')].metadata.name"
    );

    let deployments: Vec<String> = select(&json_value, &query)?
        .iter()
        .filter_map(|v| v.as_str().map(String::from))
        .collect();

    Ok(deployments)
}
