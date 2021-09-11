use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_ADD_USER_TO_GROUP_NAME: &str = "AdminAddUserToGroup";
pub const ADMIN_ADD_USER_TO_GROUP_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.AdminAddUserToGroup";

/// AdminAddUserToGroup response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminAddUserToGroup.html#API_AdminAddUserToGroup_Errors
#[derive(Display, EnumString)]
pub enum AdminAddUserToGroupError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminAddUserToGroupError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminAddUserToGroupError::InvalidParameterException
            | AdminAddUserToGroupError::NotAuthorizedException
            | AdminAddUserToGroupError::ResourceNotFoundException
            | AdminAddUserToGroupError::TooManyRequestsException
            | AdminAddUserToGroupError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AdminAddUserToGroupRequest {
    pub group_name: Option<String>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminAddUserToGroupRequest {
    fn to_action_name() -> &'static str {
        ADMIN_ADD_USER_TO_GROUP_NAME
    }
}

impl super::ToResponse for AdminAddUserToGroupRequest {
    fn to_response(&self) -> super::Response {
        if let Some(response) =
            super::config_response::<AdminAddUserToGroupRequest, AdminAddUserToGroupError>()
        {
            return response;
        };
        if !valid_request(&self) {
            let error = super::ResponseError::<AdminAddUserToGroupError>::CommonError(
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
fn valid_request(request: &AdminAddUserToGroupRequest) -> bool {
    !common::is_blank(&request.group_name)
        && !common::is_blank(&request.username)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminAddUserToGroupRequest {
            group_name: Some("group_name".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminAddUserToGroupRequest {
            group_name: Some("group_name".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminAddUserToGroupError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminAddUserToGroupError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
