use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct TokenValidityUnitsType {
    access_token: Option<String>,
    id_token: Option<String>,
    refresh_token: Option<String>,
}
