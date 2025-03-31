use crate::validator::includes;
use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

fn includes_valid_time_unit(value: &str) -> Result<(), ValidationError> {
    includes(value, vec!["seconds", "minutes", "hours", "days"])
}

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct TokenValidityUnitsType {
    #[validate(custom(function = includes_valid_time_unit))]
    access_token: Option<String>,
    #[validate(custom(function = includes_valid_time_unit))]
    id_token: Option<String>,
    #[validate(custom(function = includes_valid_time_unit))]
    refresh_token: Option<String>,
}
