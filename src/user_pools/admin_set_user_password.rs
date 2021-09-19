use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_SET_USER_PASSWORD_NAME: &str = "AdminSetUserPassword";
pub const ADMIN_SET_USER_PASSWORD_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminSetUserPassword";

/// AdminSetUserPassword response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminSetUserPassword.html#API_AdminSetUserPassword_Errors
#[derive(Display, EnumString)]
pub enum AdminSetUserPasswordError {
    InternalErrorException,
    InvalidParameterException,
    InvalidPasswordException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminSetUserPasswordError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminSetUserPasswordError::InvalidParameterException
            | AdminSetUserPasswordError::InvalidPasswordException
            | AdminSetUserPasswordError::NotAuthorizedException
            | AdminSetUserPasswordError::ResourceNotFoundException
            | AdminSetUserPasswordError::TooManyRequestsException
            | AdminSetUserPasswordError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminSetUserPasswordRequest {
    pub password: Option<String>,
    pub permanent: Option<String>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminSetUserPasswordRequest {
    fn to_action_name() -> &'static str {
        ADMIN_SET_USER_PASSWORD_NAME
    }
}

impl super::ToResponse for AdminSetUserPasswordRequest {
    fn to_response(&self) -> super::Response {
        if let Some(response) =
            super::config_response::<AdminSetUserPasswordRequest, AdminSetUserPasswordError>()
        {
            return response;
        };
        if !valid_request(&self) {
            let error = super::ResponseError::<AdminSetUserPasswordError>::CommonError(
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
fn valid_request(request: &AdminSetUserPasswordRequest) -> bool {
    !common::is_blank(&request.password)
        && !common::is_blank(&request.username)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminSetUserPasswordRequest {
            password: Some("password".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminSetUserPasswordRequest {
            password: Some("password".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminSetUserPasswordError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminSetUserPasswordError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
