use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_RESET_USER_PASSWORD_NAME: &str = "AdminResetUserPassword";
pub const ADMIN_RESET_USER_PASSWORD_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminResetUserPassword";

super::gen_response_err!(
    AdminResetUserPasswordError,
    InvalidEmailRoleAccessPolicyException
    | InvalidParameterException
    | InvalidLambdaResponseException
    | InvalidSmsRoleAccessPolicyException
    | InvalidSmsRoleTrustRelationshipException
    | NotAuthorizedException
    | LimitExceededException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UnexpectedLambdaException
    | UserLambdaValidationException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminResetUserPasswordRequest {
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminResetUserPasswordRequest {
    fn to_action_name() -> &'static str {
        ADMIN_RESET_USER_PASSWORD_NAME
    }
}

impl super::ToResponse for AdminResetUserPasswordRequest {
    type E = AdminResetUserPasswordError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminResetUserPasswordRequest) -> bool {
    !common::is_blank(&request.username) && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminResetUserPasswordRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminResetUserPasswordRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminResetUserPasswordError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminResetUserPasswordError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
