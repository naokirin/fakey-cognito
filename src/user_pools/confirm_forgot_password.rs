use crate::common::{CLIENT_ID_REGEX, CODE_REGEX, HASH_REGEX, NAME_REGEX, PASSWORD_REGEX};
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

pub const CONFIRM_FORGOT_PASSWORD_NAME: &str = "ConfirmForgotPassword";
pub const CONFIRM_FORGOT_PASSWORD_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.ConfirmForgotPassword";

super::gen_response_err!(
    ConfirmForgotPasswordError,
    CodeMismatchException
    | ExpiredCodeException
    | InvalidParameterException
    | InvalidLambdaResponseException
    | InvalidPasswordException
    | LimitExceededException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UnexpectedLambdaException
    | UserLambdaValidationException
    | UserNotConfirmedException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct ConfirmForgotPasswordRequest {
    pub analytics_metadata: Option<super::data_types::AnalyticsMetadataType>,
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    #[validate(regex = "CLIENT_ID_REGEX")]
    pub client_id: Option<String>,
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    #[validate(required)]
    #[validate(length(min = 1, max = 2048))]
    #[validate(regex = "CODE_REGEX")]
    pub confirmation_code: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 256))]
    #[validate(regex = "PASSWORD_REGEX")]
    pub password: Option<String>,
    #[validate(length(min = 1, max = 128))]
    #[validate(regex = "HASH_REGEX")]
    pub secret_hash: Option<String>,
    pub user_context_data: Option<super::data_types::UserContextDataType>,
    #[validate(required)]
    #[validate(length(min = 1, 128))]
    #[validate(regex = "NAME_REGEX")]
    pub username: Option<String>,
}

impl super::ToActionName for ConfirmForgotPasswordRequest {
    fn to_action_name() -> &'static str {
        CONFIRM_FORGOT_PASSWORD_NAME
    }
}

impl super::ToResponse for ConfirmForgotPasswordRequest {
    type E = ConfirmForgotPasswordError;
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
        let request = ConfirmForgotPasswordRequest {
            client_id: Some("clientid".to_string()),
            confirmation_code: Some("confirmation_code".to_string()),
            username: Some("username".to_string()),
            password: Some("password".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = ConfirmForgotPasswordRequest {
            client_id: Some("client_id".to_string()),
            confirmation_code: Some("confirmation_code".to_string()),
            username: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = ConfirmForgotPasswordError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = ConfirmForgotPasswordError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
