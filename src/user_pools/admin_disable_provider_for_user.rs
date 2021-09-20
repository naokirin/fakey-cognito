use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_DISABLE_PROVIDER_FOR_USER_NAME: &str = "AdminDisableProviderForUser";
pub const ADMIN_DISABLE_PROVIDER_FOR_USER_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminDisableProviderForUser";

/// AdminDisableProviderForUser response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminDisableProviderForUser.html#API_AdminDisableProviderForUser_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum AdminDisableProviderForUserError {
    AliasExistsException,
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminDisableProviderForUserError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminDisableProviderForUserError::AliasExistsException
            | AdminDisableProviderForUserError::InvalidParameterException
            | AdminDisableProviderForUserError::NotAuthorizedException
            | AdminDisableProviderForUserError::ResourceNotFoundException
            | AdminDisableProviderForUserError::TooManyRequestsException
            | AdminDisableProviderForUserError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminDisableProviderForUserRequest {
    pub user: Option<super::data_types::ProviderUserIdentifierType>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminDisableProviderForUserRequest {
    fn to_action_name() -> &'static str {
        ADMIN_DISABLE_PROVIDER_FOR_USER_NAME
    }
}

impl super::ToResponse for AdminDisableProviderForUserRequest {
    type E = AdminDisableProviderForUserError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminDisableProviderForUserRequest) -> bool {
    request.user.is_some() && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminDisableProviderForUserRequest {
            user: Some(Default::default()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminDisableProviderForUserRequest {
            user: Some(Default::default()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminDisableProviderForUserError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminDisableProviderForUserError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
