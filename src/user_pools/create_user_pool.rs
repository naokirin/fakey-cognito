use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const CREATE_USER_POOL_NAME: &str = "CreateUserPool";
pub const CREATE_USER_POOL_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.CreateUserPool";

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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserPoolRequest {
    account_recovery_setting: Option<super::data_types::AccountRecoverySettingType>,
    admin_create_user_config: Option<super::data_types::AdminCreateUserConfigType>,
    alias_attributes: Option<Vec<String>>,
    auto_verified_attributes: Option<Vec<String>>,
    device_configuration: Option<super::data_types::DeviceConfigurationType>,
    email_configuration: Option<super::data_types::EmailConfigurationType>,
    email_verification_message: Option<String>,
    lambda_config: Option<super::data_types::LambdaConfigType>,
    mfa_configuration: Option<String>,
    plicies: Option<super::data_types::UserPoolPolicyType>,
    pool_name: Option<String>,
    schema: Option<Vec<super::data_types::SchemaAttributeType>>,
    sms_authentication_message: Option<String>,
    sms_configuration: Option<super::data_types::SmsConfigurationType>,
    sms_verification_message: Option<String>,
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
        super::to_json_response(self, CREATE_USER_POOL_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &CreateUserPoolRequest) -> bool {
    !common::is_blank(&request.pool_name)
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
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = CreateUserPoolRequest {
            pool_name: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
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
