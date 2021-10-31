use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct StringAttributeConstraintsType {
    max_length: Option<String>,
    min_length: Option<String>,
}
