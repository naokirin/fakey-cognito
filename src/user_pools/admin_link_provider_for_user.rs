use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

pub const ADMIN_LINK_PROVIDER_FOR_USER_NAME: &str = "AdminLinkProviderForUser";
pub const ADMIN_LINK_PROVIDER_FOR_USER_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminLinkProviderForUser";

super::gen_response_err!(
    AdminLinkProviderForUserError,
    AliasExistsException
    | InvalidParameterException
    | NotAuthorizedException
    | LimitExceededException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AdminLinkProviderForUserRequest {
    #[validate(required_nested)]
    pub destination_user: Option<super::data_types::ProviderUserIdentifierType>,
    #[validate(required_nested)]
    pub source_user: Option<super::data_types::ProviderUserIdentifierType>,
    #[validate(required)]
    #[validate(length(min = 1))]
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
        super::to_empty_response(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminLinkProviderForUserRequest {
            destination_user: Some(Default::default()),
            source_user: Some(Default::default()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminLinkProviderForUserRequest {
            destination_user: Some(Default::default()),
            source_user: Some(Default::default()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
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
