use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ResourceServerScopeType {
    pub scope_description: Option<String>,
    pub scope_name: Option<String>,
}
