use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct RecoveryOptionType {
    name: Option<String>,
    priority: Option<i64>,
}
