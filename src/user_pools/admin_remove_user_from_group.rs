use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_REMOVE_USER_FROM_GROUP_NAME: &str = "AdminRemoveUserFromGroup";
pub const ADMIN_REMOVE_USER_FROM_GROUP_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminRemoveUserFromGroup";

/// AdminRemoveUserFromGroup response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminRemoveUserFromGroup.html#API_AdminRemoveUserFromGroup_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum AdminRemoveUserFromGroupError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminRemoveUserFromGroupError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminRemoveUserFromGroupError::InvalidParameterException
            | AdminRemoveUserFromGroupError::NotAuthorizedException
            | AdminRemoveUserFromGroupError::ResourceNotFoundException
            | AdminRemoveUserFromGroupError::TooManyRequestsException
            | AdminRemoveUserFromGroupError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminRemoveUserFromGroupRequest {
    pub group_name: Option<String>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminRemoveUserFromGroupRequest {
    fn to_action_name() -> &'static str {
        ADMIN_REMOVE_USER_FROM_GROUP_NAME
    }
}

impl super::ToResponse for AdminRemoveUserFromGroupRequest {
    type E = AdminRemoveUserFromGroupError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, ADMIN_REMOVE_USER_FROM_GROUP_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminRemoveUserFromGroupRequest) -> bool {
    !common::is_blank(&request.username)
        && !common::is_blank(&request.group_name)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminRemoveUserFromGroupRequest {
            group_name: Some("group_name".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminRemoveUserFromGroupRequest {
            group_name: Some("group_name".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminRemoveUserFromGroupError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminRemoveUserFromGroupError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
