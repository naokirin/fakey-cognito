use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct UserPoolPolicyType {
    password_policy: Option<super::PasswordPolicyType>,
}
