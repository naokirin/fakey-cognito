use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct VerificationMessageTemplateType {
    default_email_option: Option<String>,
    email_message: Option<String>,
    email_message_by_link: Option<String>,
    email_subject: Option<String>,
    email_subject_by_link: Option<String>,
    sms_message: Option<String>,
}
