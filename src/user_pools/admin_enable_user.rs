use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_ENABLE_USER_NAME: &str = "AdminEnableUser";
pub const ADMIN_ENABLE_USER_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.AdminEnableUser";

/// AdminEnableUser response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminEnableUser.html#API_AdminEnableUser_Errors
#[derive(Display, EnumString)]
pub enum AdminEnableUserError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminEnableUserError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminEnableUserError::InvalidParameterException
            | AdminEnableUserError::NotAuthorizedException
            | AdminEnableUserError::ResourceNotFoundException
            | AdminEnableUserError::TooManyRequestsException
            | AdminEnableUserError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminEnableUserRequest {
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminEnableUserRequest {
    fn to_action_name() -> &'static str {
        ADMIN_ENABLE_USER_NAME
    }
}

impl super::ToResponse for AdminEnableUserRequest {
    type E = AdminEnableUserError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminEnableUserRequest) -> bool {
    !common::is_blank(&request.username) && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminEnableUserRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminEnableUserRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminEnableUserError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminEnableUserError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
