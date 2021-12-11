use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct PasswordPolicyType {
    minimum_length: Option<i64>,
    require_lowercase: Option<bool>,
    requires_symbol: Option<bool>,
    require_uppercase: Option<bool>,
    temporary_password_validity_days: Option<i64>,
}
