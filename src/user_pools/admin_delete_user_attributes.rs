use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

use super::AdminAddUserToGroupError;

pub const ADMIN_DELETE_USER_ATTRIBUTES_NAME: &str = "AdminDeleteUserAttributes";
pub const ADMIN_DELETE_USER_ATTRIBUTES_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminDeleteUserAttributes";

/// AdminDeleteUserAttributes response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminDeleteUserAttributes.html#API_AdminDeleteUserAttributes_Errors
#[derive(Display, EnumString)]
pub enum AdminDeleteUserAttributesError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminDeleteUserAttributesError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminDeleteUserAttributesError::InvalidParameterException
            | AdminDeleteUserAttributesError::NotAuthorizedException
            | AdminDeleteUserAttributesError::ResourceNotFoundException
            | AdminDeleteUserAttributesError::TooManyRequestsException
            | AdminDeleteUserAttributesError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminDeleteUserAttributesRequest {
    pub user_attribute_names: Option<Vec<String>>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminDeleteUserAttributesRequest {
    fn to_action_name() -> &'static str {
        ADMIN_DELETE_USER_ATTRIBUTES_NAME
    }
}

impl super::ToResponse for AdminDeleteUserAttributesRequest {
    type E = AdminAddUserToGroupError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminDeleteUserAttributesRequest) -> bool {
    !common::is_none_or_empty_vec(&request.user_attribute_names)
        && !common::is_blank(&request.username)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminDeleteUserAttributesRequest {
            user_attribute_names: Some(vec!["foo".to_string()]),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminDeleteUserAttributesRequest {
            user_attribute_names: Some(vec!["foo".to_string()]),
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminDeleteUserAttributesError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminDeleteUserAttributesError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
