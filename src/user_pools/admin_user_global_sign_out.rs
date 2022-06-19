use crate::common::{NAME_REGEX, USER_POOL_ID_REGEX};
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

pub const ADMIN_USER_GLOBAL_SIGN_OUT_NAME: &str = "AdminUserGlobalSignOut";
pub const ADMIN_USER_GLOBAL_SIGN_OUT_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminUserGlobalSignOut";

super::gen_response_err!(
    AdminUserGlobalSignOutError,
    InvalidParameterException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AdminUserGlobalSignOutRequest {
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    #[validate(regex = "NAME_REGEX")]
    pub username: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex = "USER_POOL_ID_REGEX")]
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
        super::to_empty_response(self)
    }
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
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminUserGlobalSignOutRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
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
