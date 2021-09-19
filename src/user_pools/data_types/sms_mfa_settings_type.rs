use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SMSMfaSettingsType {
    pub enabled: Option<bool>,
    pub prefrred_mfa: Option<bool>,
}
