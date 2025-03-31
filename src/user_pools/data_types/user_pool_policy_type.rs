use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct UserPoolPolicyType {
    #[validate(nested)]
    password_policy: Option<super::PasswordPolicyType>,
}
