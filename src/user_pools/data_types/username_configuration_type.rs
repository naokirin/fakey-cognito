use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct UsernameConfigurationType {
    #[validate(required)]
    case_sensitive: Option<bool>,
}
