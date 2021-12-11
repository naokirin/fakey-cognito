use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct UserPoolAddOnsType {
    advanced_security_mode: Option<String>,
}
