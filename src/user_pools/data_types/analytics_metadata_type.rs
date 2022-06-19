use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AnalyticsMetadataType {
    #[validate(required)]
    pub analytics_endpoint_id: Option<String>,
}
