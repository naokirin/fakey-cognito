use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AnalyticsConfigurationType {
    application_arn: Option<String>,
    application_id: Option<String>,
    external_id: Option<String>,
    role_arn: Option<String>,
    user_data_shared: Option<bool>,
}
