use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_RESPOND_TO_AUTH_CHALLENGE_NAME: &str = "AdminRespondToAuthChallenge";
pub const ADMIN_RESPOND_TO_AUTH_CHALLENGE_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminRespondToAuthChallenge";

/// AdminRespondToAuthChallenge response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminRespondToAuthChallenge.html#API_AdminRespondToAuthChallenge_Errors
#[derive(Display, EnumString)]
pub enum AdminRespondToAuthChallengeError {
    AliasExistsException,
    CodeMismatchException,
    ExpiredCodeException,
    InternalErrorException,
    InvalidLambdaResponseException,
    InvalidParameterException,
    InvalidPasswordException,
    InvalidSmsRoleAccessPolicyException,
    InvalidSmsRoleTrustRelationshipException,
    InvalidUserPoolConfigurationException,
    MFAMethodNotFoundException,
    NotAuthorizedException,
    PasswordResetRequiredException,
    ResourceNotFoundException,
    SoftwareTokenMFANotFoundException,
    TooManyRequestsException,
    UnexpectedLambdaException,
    UserLambdaValidationException,
    UserNotConfirmedException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminRespondToAuthChallengeError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminRespondToAuthChallengeError::AliasExistsException
            | AdminRespondToAuthChallengeError::CodeMismatchException
            | AdminRespondToAuthChallengeError::ExpiredCodeException
            | AdminRespondToAuthChallengeError::InvalidParameterException
            | AdminRespondToAuthChallengeError::InvalidPasswordException
            | AdminRespondToAuthChallengeError::InvalidLambdaResponseException
            | AdminRespondToAuthChallengeError::InvalidSmsRoleAccessPolicyException
            | AdminRespondToAuthChallengeError::InvalidSmsRoleTrustRelationshipException
            | AdminRespondToAuthChallengeError::InvalidUserPoolConfigurationException
            | AdminRespondToAuthChallengeError::MFAMethodNotFoundException
            | AdminRespondToAuthChallengeError::NotAuthorizedException
            | AdminRespondToAuthChallengeError::PasswordResetRequiredException
            | AdminRespondToAuthChallengeError::ResourceNotFoundException
            | AdminRespondToAuthChallengeError::TooManyRequestsException
            | AdminRespondToAuthChallengeError::SoftwareTokenMFANotFoundException
            | AdminRespondToAuthChallengeError::UnexpectedLambdaException
            | AdminRespondToAuthChallengeError::UserLambdaValidationException
            | AdminRespondToAuthChallengeError::UserNotConfirmedException
            | AdminRespondToAuthChallengeError::UserNotFoundException => http::status_code(400),
            AdminRespondToAuthChallengeError::InternalErrorException => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminRespondToAuthChallengeRequest {
    pub analytics_metadata: Option<super::data_types::AnalyticsMetadataType>,
    pub challenge_name: Option<String>,
    pub challenge_response: Option<std::collections::HashMap<String, String>>,
    pub client_id: Option<String>,
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    pub context_data: Option<super::data_types::ContextDataType>,
    pub session: Option<String>,
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
        super::to_json_response(self, ADMIN_RESPOND_TO_AUTH_CHALLENGE_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminRespondToAuthChallengeRequest) -> bool {
    !common::is_blank(&request.challenge_name)
        && !common::is_blank(&request.client_id)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminRespondToAuthChallengeRequest {
            challenge_name: Some("challenge_name".to_string()),
            client_id: Some("client_id".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminRespondToAuthChallengeRequest {
            challenge_name: Some("challenge_name".to_string()),
            client_id: Some("client_id".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
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
