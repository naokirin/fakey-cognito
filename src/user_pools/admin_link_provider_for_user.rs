use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_LINK_PROVIDER_FOR_USER_NAME: &str = "AdminLinkProviderForUser";
pub const ADMIN_LINK_PROVIDER_FOR_USER_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminLinkProviderForUser";

/// AdminLinkProviderForUser response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminLinkProviderForUser.html#API_AdminLinkProviderForUser_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum AdminLinkProviderForUserError {
    AliasExistsException,
    InternalErrorException,
    InvalidParameterException,
    LimitExceededException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminLinkProviderForUserError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminLinkProviderForUserError::AliasExistsException
            | AdminLinkProviderForUserError::InvalidParameterException
            | AdminLinkProviderForUserError::NotAuthorizedException
            | AdminLinkProviderForUserError::LimitExceededException
            | AdminLinkProviderForUserError::ResourceNotFoundException
            | AdminLinkProviderForUserError::TooManyRequestsException
            | AdminLinkProviderForUserError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminLinkProviderForUserRequest {
    pub desitination_user: Option<super::data_types::ProviderUserIdentifierType>,
    pub source_user: Option<super::data_types::ProviderUserIdentifierType>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminLinkProviderForUserRequest {
    fn to_action_name() -> &'static str {
        ADMIN_LINK_PROVIDER_FOR_USER_NAME
    }
}

impl super::ToResponse for AdminLinkProviderForUserRequest {
    type E = AdminLinkProviderForUserError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminLinkProviderForUserRequest) -> bool {
    request.desitination_user.is_some()
        && request.source_user.is_some()
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminLinkProviderForUserRequest {
            desitination_user: Some(Default::default()),
            source_user: Some(Default::default()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminLinkProviderForUserRequest {
            desitination_user: Some(Default::default()),
            source_user: Some(Default::default()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminLinkProviderForUserError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminLinkProviderForUserError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
