use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_CONFIRM_SIGN_UP_NAME: &str = "AdminConfirmSignUp";
pub const ADMIN_CONFIRM_SIGN_UP_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminConfirmSignUp";

/// AdminConfirmSignUp response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminConfirmSignUp.html#API_AdminConfirmSignUp_Errors
#[derive(Display, EnumString)]
pub enum AdminConfirmSignUpError {
    InternalErrorException,
    InvalidLambdaResponseException,
    InvalidParameterException,
    LimitExceededException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyFailedAttemptsException,
    TooManyRequestsException,
    UseLambdaValidationException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminConfirmSignUpError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminConfirmSignUpError::InvalidParameterException
            | AdminConfirmSignUpError::InvalidLambdaResponseException
            | AdminConfirmSignUpError::LimitExceededException
            | AdminConfirmSignUpError::NotAuthorizedException
            | AdminConfirmSignUpError::ResourceNotFoundException
            | AdminConfirmSignUpError::TooManyFailedAttemptsException
            | AdminConfirmSignUpError::TooManyRequestsException
            | AdminConfirmSignUpError::UseLambdaValidationException
            | AdminConfirmSignUpError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AdminConfirmSignUpRequest {
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminConfirmSignUpRequest {
    fn to_action_name() -> &'static str {
        ADMIN_CONFIRM_SIGN_UP_NAME
    }
}

impl super::ToResponse for AdminConfirmSignUpRequest {
    type E = AdminConfirmSignUpError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminConfirmSignUpRequest) -> bool {
    !common::is_blank(&request.username) && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminConfirmSignUpRequest {
            client_metadata: Some(std::collections::HashMap::new()),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminConfirmSignUpRequest {
            client_metadata: Some(std::collections::HashMap::new()),
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminConfirmSignUpError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminConfirmSignUpError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
