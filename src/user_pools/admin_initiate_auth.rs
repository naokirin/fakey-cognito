use crate::common::{CLIENT_ID_REGEX, USER_POOL_ID_REGEX};
use crate::{http, validator::includes};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::{Validate, ValidationError};

pub const ADMIN_INITIATE_AUTH_NAME: &str = "AdminInitiateAuth";
pub const ADMIN_INITIATE_AUTH_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminInitiateAuth";

fn validate_auth_flow(value: &str) -> Result<(), ValidationError> {
    includes(
        value,
        vec![
            "USER_SRP_AUTH",
            "REFRESH_TOKEN_AUTH",
            "REFRESH_TOKEN",
            "CUSTOM_AUTH",
            "ADMIN_NO_SRP_AUTH",
            "USER_PASSWORD_AUTH",
            "ADMIN_USER_PASSWORD_AUTH",
        ],
    )
}

super::gen_response_err!(
    AdminInitiateAuthError,
    InvalidParameterException
    | InvalidLambdaResponseException
    | InvalidSmsRoleAccessPolicyException
    | InvalidSmsRoleTrustRelationshipException
    | InvalidUserPoolConfigurationException
    | MFAMethodNotFoundException
    | NotAuthorizedException
    | PasswordResetRequiredException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UnexpectedLambdaException
    | UserLambdaValidationException
    | UserNotConfirmedException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AdminInitiateAuthRequest {
    #[validate(nested)]
    pub analytics_metadata: Option<super::data_types::AnalyticsMetadataType>,
    #[validate(required)]
    #[validate(length(min = 1))]
    #[validate(custom(function = validate_auth_flow))]
    pub auth_flow: Option<String>,
    pub auth_parameters: Option<std::collections::HashMap<String, String>>,
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    #[validate(regex(path = *CLIENT_ID_REGEX))]
    pub client_id: Option<String>,
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    pub context_data: Option<super::data_types::ContextDataType>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex(path = *USER_POOL_ID_REGEX))]
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminInitiateAuthRequest {
    fn to_action_name() -> &'static str {
        ADMIN_INITIATE_AUTH_NAME
    }
}

impl super::ToResponse for AdminInitiateAuthRequest {
    type E = AdminInitiateAuthError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, ADMIN_INITIATE_AUTH_NAME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminInitiateAuthRequest {
            auth_flow: Some("USER_SRP_AUTH".to_string()),
            client_id: Some("client_id".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminInitiateAuthRequest {
            auth_flow: Some("USER_SRP_AUTH".to_string()),
            client_id: Some("client_id".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminInitiateAuthError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminInitiateAuthError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
