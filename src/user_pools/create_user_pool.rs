use crate::{
    http,
    validator::{includes, includes_in_array},
};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::{Validate, ValidationError};

pub const CREATE_USER_POOL_NAME: &str = "CreateUserPool";
pub const CREATE_USER_POOL_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.CreateUserPool";

static EMAIL_VERIFICATION_MESSAGE_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*\{####\}[\p{L}\p{M}\p{S}\p{N}\p{P}\s*]*").unwrap()
});

static EMAIL_VERIFICATION_SUBJECT_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\p{L}\p{M}\p{S}\p{N}\p{P}\s]+").unwrap());

static POOL_NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\w\s+=,.@-]+").unwrap());

static SMS_AUTHENTICATION_MESSAGE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r".*\{####\}.*").unwrap());

static SMS_VERIFICATION_MESSAGE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r".*\{####\}.*").unwrap());

fn validate_alias_attributes(value: &[String]) -> Result<(), ValidationError> {
    includes_in_array(value, vec!["phone_number", "email", "preferred_username"])
}

fn validate_auto_verified_attributes(value: &[String]) -> Result<(), ValidationError> {
    includes_in_array(value, vec!["phone_number", "email"])
}

fn validate_mfa_configuration(value: &str) -> Result<(), ValidationError> {
    includes(value, vec!["OFF", "ON", "OPTIONAL"])
}

fn validate_username_attributes(value: &[String]) -> Result<(), ValidationError> {
    includes_in_array(value, vec!["phone_number", "email"])
}

super::gen_response_err!(
    CreateUserPoolError,
    InvalidEmailRoleAccessPolicyException
    | InvalidParameterException
    | InvalidSmsRoleAccessPolicyException
    | InvalidSmsRoleTrustRelationshipException
    | LimitExceededException
    | NotAuthorizedException
    | TooManyRequestsException
    | UserPoolTaggingException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserPoolRequest {
    account_recovery_setting: Option<super::data_types::AccountRecoverySettingType>,
    admin_create_user_config: Option<super::data_types::AdminCreateUserConfigType>,
    #[validate(custom(function = "validate_alias_attributes"))]
    alias_attributes: Option<Vec<String>>,
    #[validate(custom(function = "validate_auto_verified_attributes"))]
    auto_verified_attributes: Option<Vec<String>>,
    device_configuration: Option<super::data_types::DeviceConfigurationType>,
    email_configuration: Option<super::data_types::EmailConfigurationType>,
    #[validate(length(min = 6, max = 20000))]
    #[validate(regex = "EMAIL_VERIFICATION_MESSAGE_REGEX")]
    email_verification_message: Option<String>,
    #[validate(length(min = 1, max = 140))]
    #[validate(regex = "EMAIL_VERIFICATION_SUBJECT_REGEX")]
    email_verification_subject: Option<String>,
    lambda_config: Option<super::data_types::LambdaConfigType>,
    #[validate(custom(function = "validate_mfa_configuration"))]
    mfa_configuration: Option<String>,
    plicies: Option<super::data_types::UserPoolPolicyType>,
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    #[validate(regex = "POOL_NAME_REGEX")]
    pool_name: Option<String>,
    schema: Option<Vec<super::data_types::SchemaAttributeType>>,
    #[validate(length(min = 6, max = 140))]
    #[validate(regex = "SMS_AUTHENTICATION_MESSAGE_REGEX")]
    sms_authentication_message: Option<String>,
    sms_configuration: Option<super::data_types::SmsConfigurationType>,
    #[validate(length(min = 6, max = 140))]
    #[validate(regex = "SMS_VERIFICATION_MESSAGE_REGEX")]
    sms_verification_message: Option<String>,
    #[validate(custom(function = "validate_username_attributes"))]
    username_attributes: Option<Vec<String>>,
    username_configuration: Option<super::data_types::UsernameConfigurationType>,
    user_pool_add_ons: Option<super::data_types::UserPoolAddOnsType>,
    user_pool_tags: Option<std::collections::HashMap<String, String>>,
    verification_message_template: Option<super::data_types::VerificationMessageTemplateType>,
}

impl super::ToActionName for CreateUserPoolRequest {
    fn to_action_name() -> &'static str {
        CREATE_USER_POOL_NAME
    }
}

impl super::ToResponse for CreateUserPoolRequest {
    type E = CreateUserPoolError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, CREATE_USER_POOL_NAME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = CreateUserPoolRequest {
            pool_name: Some("pool_name".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = CreateUserPoolRequest {
            pool_name: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = CreateUserPoolError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = CreateUserPoolError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
