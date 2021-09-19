use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MFAOptionType {
    attribute_name: Option<String>,
    delivery_medium: Option<String>,
}
