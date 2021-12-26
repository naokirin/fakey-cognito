use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::validator::includes;

fn includes_valid_name(value: &str) -> Result<(), ValidationError> {
    includes(
        value,
        vec!["verified_email", "verified_phone_number", "admin_only"],
    )
}

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct RecoveryOptionType {
    #[validate(required)]
    #[validate(custom(function = "includes_valid_name"))]
    name: Option<String>,
    #[validate(required)]
    #[validate(range(min = 1, max = 2))]
    priority: Option<i64>,
}
