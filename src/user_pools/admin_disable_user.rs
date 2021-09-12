use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_DISABLE_USER_NAME: &str = "AdminDisableUser";
pub const ADMIN_DISABLE_USER_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminDisableUser";

/// AdminDisableUser response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminDisableUser.html#API_AdminDisableUser_Errors
#[derive(Display, EnumString)]
pub enum AdminDisableUserError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminDisableUserError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminDisableUserError::InvalidParameterException
            | AdminDisableUserError::NotAuthorizedException
            | AdminDisableUserError::ResourceNotFoundException
            | AdminDisableUserError::TooManyRequestsException
            | AdminDisableUserError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminDisableUserRequest {
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminDisableUserRequest {
    fn to_action_name() -> &'static str {
        ADMIN_DISABLE_USER_NAME
    }
}

impl super::ToResponse for AdminDisableUserRequest {
    fn to_response(&self) -> super::Response {
        if let Some(response) =
            super::config_response::<AdminDisableUserRequest, AdminDisableUserError>()
        {
            return response;
        };
        if !valid_request(&self) {
            let error = super::ResponseError::<AdminDisableUserError>::CommonError(
                super::CommonError::InvalidParameterValue,
            );
            return super::error_response(error);
        }

        warp::http::Response::builder()
            .status(http::status_code(200))
            .body(super::responses::empty_body())
            .unwrap()
    }
}

/// Validates request.
fn valid_request(request: &AdminDisableUserRequest) -> bool {
    !common::is_blank(&request.username) && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminDisableUserRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminDisableUserRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminDisableUserError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminDisableUserError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
