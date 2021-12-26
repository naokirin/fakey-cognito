use crate::common::ARN_REGEX;
use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct LambdaConfigType {
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    create_auth_challenge: Option<String>,
    #[validate]
    custom_email_sender: Option<super::CustomEmailLambdaVersionConfigType>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    custom_message: Option<String>,
    #[serde(rename = "CustomSMSSender")]
    #[validate]
    custom_sms_sender: Option<super::CustomSMSLambdaVersionConfigType>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    define_auth_challenge: Option<String>,
    #[serde(rename = "KMSKeyID")]
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    kms_key_id: Option<String>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    post_authentication: Option<String>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    post_confirmation: Option<String>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    pre_authentication: Option<String>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    pre_sign_up: Option<String>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    pre_token_generation: Option<String>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    user_migration: Option<String>,
    #[validate(length(min = 20, max = 2048))]
    #[validate(regex = "ARN_REGEX")]
    verify_auth_challenge_response: Option<String>,
}
