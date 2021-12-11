use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct EmailConfigurationType {
    configuration_set: Option<String>,
    email_sending_account: Option<String>,
    from: Option<String>,
    reply_to_email_address: Option<String>,
    source_arn: Option<String>,
}
