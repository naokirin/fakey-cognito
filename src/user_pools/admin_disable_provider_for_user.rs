use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

pub const ADMIN_DISABLE_PROVIDER_FOR_USER_NAME: &str = "AdminDisableProviderForUser";
pub const ADMIN_DISABLE_PROVIDER_FOR_USER_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminDisableProviderForUser";

super::gen_response_err!(
    AdminDisableProviderForUserError,
    AliasExistsException
    | InvalidParameterException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AdminDisableProviderForUserRequest {
    #[validate(required)]
    #[validate(nested)]
    pub user: Option<super::data_types::ProviderUserIdentifierType>,
    #[validate(required)]
    #[validate(length(min = 1))]
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
        super::to_empty_response(self)
    }
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
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminDisableProviderForUserRequest {
            user: Some(Default::default()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
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
