use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct SMSMfaSettingsType {
    pub enabled: Option<bool>,
    pub prefrred_mfa: Option<bool>,
}
