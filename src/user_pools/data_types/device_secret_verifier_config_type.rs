use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct DeviceSecretVerifierConfigType {
    password_verifier: Option<String>,
    salt: Option<String>,
}
