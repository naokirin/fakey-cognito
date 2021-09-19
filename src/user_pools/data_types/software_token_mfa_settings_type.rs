use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SoftwareTokenMfaSettingsType {
    enabled: Option<bool>,
    prefrred_mfa: Option<bool>,
}
