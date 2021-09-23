use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ProviderUserIdentifierType {
    pub provider_attribute_name: Option<String>,
    pub provider_attribute_value: Option<String>,
    pub provider_name: Option<String>,
}
