use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AttributeType {
    pub name: Option<String>,
    pub value: Option<String>,
}
