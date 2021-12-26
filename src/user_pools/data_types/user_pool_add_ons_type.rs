use crate::validator::includes;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

fn includes_valid_advanced_secure_mode(value: &str) -> Result<(), ValidationError> {
    includes(value, vec!["OFF", "AUDIT", "ENFORCED"])
}

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct UserPoolAddOnsType {
    #[validate(custom(function = "includes_valid_advanced_secure_mode"))]
    advanced_security_mode: Option<String>,
}
