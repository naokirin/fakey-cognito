use crate::common::ARN_REGEX;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct SmsConfigurationType {
    external_id: Option<String>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex(path = *ARN_REGEX))]
    sns_caller_arn: Option<String>,
}
