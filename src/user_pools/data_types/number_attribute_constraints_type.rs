use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct NumberAttributeConstraintsType {
    max_value: Option<String>,
    min_value: Option<String>,
}
