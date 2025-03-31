use crate::common::{PASSWORD_REGEX, TOKEN_REGEX};
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

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

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct ChangePasswordRequest {
    #[validate(required)]
    #[validate(length(min = 1))]
    #[validate(regex(path = *TOKEN_REGEX))]
    access_token: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 256))]
    #[validate(regex(path = *PASSWORD_REGEX))]
    previous_password: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 256))]
    #[validate(regex(path = *PASSWORD_REGEX))]
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
        super::to_empty_response(self)
    }
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
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = ChangePasswordRequest {
            ..Default::default()
        };
        assert!(request.validate().is_err());
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
