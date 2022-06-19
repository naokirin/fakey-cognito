use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceSecretVerifierConfigType {
    password_verifier: Option<String>,
    salt: Option<String>,
}
