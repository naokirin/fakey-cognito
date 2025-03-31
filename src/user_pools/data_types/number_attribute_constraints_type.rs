use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

static DEVICE_KEY_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\w-]+_[0-9a-f-]+").unwrap());

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct NumberAttributeConstraintsType {
    max_value: Option<String>,
    #[validate(regex(path = *DEVICE_KEY_REGEX))]
    min_value: Option<String>,
}
