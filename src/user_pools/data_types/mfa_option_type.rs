use crate::common::NAME_REGEX;
use crate::validator::includes;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

fn includes_delivery_medium(value: &str) -> Result<(), ValidationError> {
    includes(value, vec!["SMS", "EMAIL"])
}

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct MFAOptionType {
    #[validate(length(min = 1, max = 32))]
    #[validate(regex(path = *NAME_REGEX))]
    attribute_name: Option<String>,
    #[validate(custom(function = includes_delivery_medium))]
    delivery_medium: Option<String>,
}
