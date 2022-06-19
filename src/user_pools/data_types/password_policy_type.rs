use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct PasswordPolicyType {
    #[validate(range(min = 6, max = 99))]
    minimum_length: Option<i64>,
    require_lowercase: Option<bool>,
    requires_symbol: Option<bool>,
    require_uppercase: Option<bool>,
    #[validate(range(min = 0, max = 365))]
    temporary_password_validity_days: Option<i64>,
}
