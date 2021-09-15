use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AnalyticsMetadataType {
    pub analytics_endpoint_id: Option<String>,
}
