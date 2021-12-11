use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminCreateUserConfigType {
    allow_admin_create_user_only: Option<bool>,
    invite_message_template: Option<super::MessageTemplateType>,
    unsed_account_validity_days: Option<i64>,
}
