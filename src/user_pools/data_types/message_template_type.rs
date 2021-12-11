use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct MessageTemplateType {
    email_message: Option<String>,
    email_subject: Option<String>,
    sms_message: Option<String>,
}
