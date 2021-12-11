use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct LambdaConfigType {
    create_auth_challenge: Option<String>,
    custom_email_sender: Option<super::CustomEmailLambdaVersionConfigType>,
    custom_message: Option<String>,
    #[serde(rename = "CustomSMSSender")]
    custom_sms_sender: Option<super::CustomSMSLambdaVersionConfigType>,
    define_auth_challenge: Option<String>,
    #[serde(rename = "KMSKeyID")]
    kms_key_id: Option<String>,
    post_authentication: Option<String>,
    post_confirmation: Option<String>,
    pre_authentication: Option<String>,
    pre_sign_up: Option<String>,
    pre_token_generation: Option<String>,
    user_migration: Option<String>,
    verify_auth_challenge_response: Option<String>,
}
