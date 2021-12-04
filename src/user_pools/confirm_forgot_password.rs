use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

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

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfirmForgotPasswordRequest {
    pub analytics_metadata: Option<super::data_types::AnalyticsMetadataType>,
    pub client_id: Option<String>,
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    pub confirmation_code: Option<String>,
    pub password: Option<String>,
    pub secret_hash: Option<String>,
    pub user_context_data: Option<super::data_types::UserContextDataType>,
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
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &ConfirmForgotPasswordRequest) -> bool {
    !common::is_blank(&request.client_id)
        && !common::is_blank(&request.confirmation_code)
        && !common::is_blank(&request.username)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = ConfirmForgotPasswordRequest {
            client_id: Some("client_id".to_string()),
            confirmation_code: Some("confirmation_code".to_string()),
            username: Some("username".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = ConfirmForgotPasswordRequest {
            client_id: Some("client_id".to_string()),
            confirmation_code: Some("confirmation_code".to_string()),
            username: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
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
