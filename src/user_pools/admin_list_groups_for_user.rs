use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_LIST_GROUPS_FOR_USER_NAME: &str = "AdminListGroupsForUser";
pub const ADMIN_LIST_GROUPS_FOR_USER_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminListGroupsForUser";

/// AdminListGroupsForUser response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminListGroupsForUser.html#API_AdminListGroupsForUser_Errors
#[derive(Display, EnumString)]
pub enum AdminListGroupsForUserError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminListGroupsForUserError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminListGroupsForUserError::InvalidParameterException
            | AdminListGroupsForUserError::NotAuthorizedException
            | AdminListGroupsForUserError::ResourceNotFoundException
            | AdminListGroupsForUserError::TooManyRequestsException
            | AdminListGroupsForUserError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminListGroupsForUserRequest {
    limit: Option<u8>,
    next_token: Option<String>,
    username: Option<String>,
    user_pool_id: Option<String>,
}

impl super::ToActionName for AdminListGroupsForUserRequest {
    fn to_action_name() -> &'static str {
        ADMIN_LIST_GROUPS_FOR_USER_NAME
    }
}

impl super::ToResponse for AdminListGroupsForUserRequest {
    type E = AdminListGroupsForUserError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, ADMIN_LIST_GROUPS_FOR_USER_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminListGroupsForUserRequest) -> bool {
    !common::is_blank(&request.username) && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminListGroupsForUserRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminListGroupsForUserRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminListGroupsForUserError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminListGroupsForUserError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
