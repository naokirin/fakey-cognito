use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AdminCreateUserConfigType {
    allow_admin_create_user_only: Option<bool>,
    #[validate(nested)]
    invite_message_template: Option<super::MessageTemplateType>,
    #[validate(range(min = 0, max = 365))]
    unsed_account_validity_days: Option<i64>,
}
