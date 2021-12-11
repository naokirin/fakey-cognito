use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct UsernameConfigurationType {
    case_sensitive: Option<bool>,
}
