use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_USER_GLOBAL_SIGN_OUT_NAME: &str = "AdminUserGlobalSignOut";
pub const ADMIN_USER_GLOBAL_SIGN_OUT_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminUserGlobalSignOut";

/// AdminUserGlobalSignOut response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminUserGlobalSignOut.html#API_AdminUserGlobalSignOut_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum AdminUserGlobalSignOutError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminUserGlobalSignOutError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminUserGlobalSignOutError::InvalidParameterException
            | AdminUserGlobalSignOutError::NotAuthorizedException
            | AdminUserGlobalSignOutError::ResourceNotFoundException
            | AdminUserGlobalSignOutError::TooManyRequestsException
            | AdminUserGlobalSignOutError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminUserGlobalSignOutRequest {
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminUserGlobalSignOutRequest {
    fn to_action_name() -> &'static str {
        ADMIN_USER_GLOBAL_SIGN_OUT_NAME
    }
}

impl super::ToResponse for AdminUserGlobalSignOutRequest {
    type E = AdminUserGlobalSignOutError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminUserGlobalSignOutRequest) -> bool {
    !common::is_blank(&request.username) && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminUserGlobalSignOutRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminUserGlobalSignOutRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminUserGlobalSignOutError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminUserGlobalSignOutError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
