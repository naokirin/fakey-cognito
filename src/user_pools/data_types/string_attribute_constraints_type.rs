use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct StringAttributeConstraintsType {
    max_length: Option<String>,
    min_length: Option<String>,
}
