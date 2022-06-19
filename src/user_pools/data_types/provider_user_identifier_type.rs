use crate::common::NAME_REGEX;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct ProviderUserIdentifierType {
    pub provider_attribute_name: Option<String>,
    pub provider_attribute_value: Option<String>,
    #[validate(length(min = 1, max = 32))]
    #[validate(regex = "NAME_REGEX")]
    pub provider_name: Option<String>,
}
