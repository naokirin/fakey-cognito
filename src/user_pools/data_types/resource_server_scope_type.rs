use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use validator::Validate;

static SCOPE_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\x21\x23-\x2E\x30-\x5B\x5D-\x7E]+").unwrap());

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct ResourceServerScopeType {
    #[validate(length(min = 1, max = 256))]
    pub scope_description: Option<String>,
    #[validate(length(min = 1, max = 256))]
    #[validate(regex = "SCOPE_NAME_REGEX")]
    pub scope_name: Option<String>,
}
