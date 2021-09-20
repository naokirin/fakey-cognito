use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_DELETE_USER_NAME: &str = "AdminDeleteUser";
pub const ADMIN_DELETE_USER_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.AdminDeleteUser";

/// AdminDeleteUser response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminDeleteUser.html#API_AdminDeleteUser_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum AdminDeleteUserError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminDeleteUserError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminDeleteUserError::InvalidParameterException
            | AdminDeleteUserError::NotAuthorizedException
            | AdminDeleteUserError::ResourceNotFoundException
            | AdminDeleteUserError::TooManyRequestsException
            | AdminDeleteUserError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminDeleteUserRequest {
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminDeleteUserRequest {
    fn to_action_name() -> &'static str {
        ADMIN_DELETE_USER_NAME
    }
}

impl super::ToResponse for AdminDeleteUserRequest {
    type E = AdminDeleteUserError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminDeleteUserRequest) -> bool {
    !common::is_blank(&request.username) && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminDeleteUserRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminDeleteUserRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminDeleteUserError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminDeleteUserError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
