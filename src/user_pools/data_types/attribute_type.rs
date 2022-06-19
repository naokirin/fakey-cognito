use crate::common::NAME_REGEX;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AttributeType {
    #[validate(required)]
    #[validate(length(min = 1, max = 32))]
    #[validate(regex = "NAME_REGEX")]
    pub name: Option<String>,
    #[validate(length(max = 2048))]
    pub value: Option<String>,
}
