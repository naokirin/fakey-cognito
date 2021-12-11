use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct SmsConfigurationType {
    external_id: Option<String>,
    sns_caller_arn: Option<String>,
}
