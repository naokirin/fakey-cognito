use crate::common::ARN_REGEX;
use crate::validator::includes_lambda_version;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct CustomSMSLambdaVersionConfigType {
    #[validate(required)]
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex(path = *ARN_REGEX))]
    lambda_arn: Option<String>,
    #[validate(required)]
    #[validate(custom(function = includes_lambda_version))]
    lambda_version: Option<String>,
}
