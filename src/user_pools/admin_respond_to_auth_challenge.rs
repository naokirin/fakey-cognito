use crate::common::{CLIENT_ID_REGEX, USER_POOL_ID_REGEX};
use crate::{http, validator::includes};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::{Validate, ValidationError};

pub const ADMIN_RESPOND_TO_AUTH_CHALLENGE_NAME: &str = "AdminRespondToAuthChallenge";
pub const ADMIN_RESPOND_TO_AUTH_CHALLENGE_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminRespondToAuthChallenge";

fn validate_challenge_name(value: &str) -> Result<(), ValidationError> {
    includes(
        value,
        vec![
            "SMS_MFA",
            "SOFTWARE_TOKEN_MFA",
            "SELECT_MFA_TYPE",
            "MFA_SETUP",
            "PASSWORD_VERIFIER",
            "CUSTOM_CHALLENGE",
            "DEVICE_SRP_AUTH",
            "DEVICE_PASSWORD_VERIFIER",
            "ADMIN_NO_SRP_AUTH",
            "NEW_PASSWORD_REQUIRED",
        ],
    )
}

super::gen_response_err!(
    AdminRespondToAuthChallengeError,
    AliasExistsException
    | CodeMismatchException
    | ExpiredCodeException
    | InvalidParameterException
    | InvalidPasswordException
    | InvalidLambdaResponseException
    | InvalidSmsRoleAccessPolicyException
    | InvalidSmsRoleTrustRelationshipException
    | InvalidUserPoolConfigurationException
    | MFAMethodNotFoundException
    | NotAuthorizedException
    | PasswordResetRequiredException
    | ResourceNotFoundException
    | TooManyRequestsException
    | SoftwareTokenMFANotFoundException
    | UnexpectedLambdaException
    | UserLambdaValidationException
    | UserNotConfirmedException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AdminRespondToAuthChallengeRequest {
    pub analytics_metadata: Option<super::data_types::AnalyticsMetadataType>,
    #[validate(required)]
    #[validate(length(min = 1))]
    #[validate(custom(function = "validate_challenge_name"))]
    pub challenge_name: Option<String>,
    pub challenge_response: Option<std::collections::HashMap<String, String>>,
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    #[validate(regex = "CLIENT_ID_REGEX")]
    pub client_id: Option<String>,
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    pub context_data: Option<super::data_types::ContextDataType>,
    #[validate(length(min = 20, max = 2048))]
    pub session: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex = "USER_POOL_ID_REGEX")]
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminRespondToAuthChallengeRequest {
    fn to_action_name() -> &'static str {
        ADMIN_RESPOND_TO_AUTH_CHALLENGE_NAME
    }
}

impl super::ToResponse for AdminRespondToAuthChallengeRequest {
    type E = AdminRespondToAuthChallengeError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, ADMIN_RESPOND_TO_AUTH_CHALLENGE_NAME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminRespondToAuthChallengeRequest {
            challenge_name: Some("SMS_MFA".to_string()),
            client_id: Some("client_id".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminRespondToAuthChallengeRequest {
            challenge_name: Some("SMS_MFA".to_string()),
            client_id: Some("client_id".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminRespondToAuthChallengeError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminRespondToAuthChallengeError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
