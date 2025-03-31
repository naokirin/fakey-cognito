use serde::{Deserialize, Serialize};
use validator::{Validate, ValidationError};

use crate::common::NAME_REGEX;
use crate::validator::includes;

fn includes_valid_attribute_data_type(value: &str) -> Result<(), ValidationError> {
    includes(value, vec!["String", "Number", "DateTime", "Boolean"])
}

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct SchemaAttributeType {
    #[validate(custom(function = includes_valid_attribute_data_type))]
    pub attribute_data_type: Option<String>,
    pub developer_only_attribute: Option<bool>,
    pub mutable: Option<bool>,
    #[validate(length(min = 1, max = 20))]
    #[validate(regex(path = *NAME_REGEX))]
    pub name: Option<String>,
    #[validate(nested)]
    pub number_attribute_constraints: Option<super::NumberAttributeConstraintsType>,
    pub required: Option<bool>,
    #[validate(nested)]
    pub string_attribute_constraints: Option<super::StringAttributeConstraintsType>,
}
