use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const CHANGE_PASSWORD_NAME: &str = "ChangePassword";
pub const CHANGE_PASSWORD_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.ChangePassword";

super::gen_response_err!(
    ChangePasswordError,
    InvalidParameterException
    | InvalidPasswordException
    | LimitExceededException
    | NotAuthorizedException
    | PasswordResetRequiredException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UserNotConfirmedException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ChangePasswordRequest {
    access_token: Option<String>,
    previous_password: Option<String>,
    proposed_password: Option<String>,
}

impl super::ToActionName for ChangePasswordRequest {
    fn to_action_name() -> &'static str {
        CHANGE_PASSWORD_NAME
    }
}

impl super::ToResponse for ChangePasswordRequest {
    type E = ChangePasswordError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &ChangePasswordRequest) -> bool {
    !common::is_blank(&request.access_token)
        && !common::is_blank(&request.previous_password)
        && !common::is_blank(&request.proposed_password)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = ChangePasswordRequest {
            access_token: Some("access_token".to_string()),
            previous_password: Some("previous_password".to_string()),
            proposed_password: Some("proposed_password".to_string()),
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = ChangePasswordRequest {
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = ChangePasswordError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = ChangePasswordError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
