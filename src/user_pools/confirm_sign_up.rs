use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const CONFIRM_SIGN_UP_NAME: &str = "ConfirmSignUp";
pub const CONFIRM_SIGN_UP_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.ConfirmSignUp";

super::gen_response_err!(
    ConfirmSignUpError,
    AliasExistsException
    | CodeMismatchException
    | ExpiredCodeException
    | InvalidParameterException
    | InvalidLambdaResponseException
    | LimitExceededException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException
    | TooManyFailedAttemptsException
    | UnexpectedLambdaException
    | UserLambdaValidationException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfirmSignUpRequest {
    pub analytics_metadata: Option<super::data_types::AnalyticsMetadataType>,
    pub client_id: Option<String>,
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    pub confirmation_code: Option<String>,
    pub force_alias_creation: Option<bool>,
    pub secret_hash: Option<String>,
    pub user_context_data: Option<super::data_types::UserContextDataType>,
    pub username: Option<String>,
}

impl super::ToActionName for ConfirmSignUpRequest {
    fn to_action_name() -> &'static str {
        CONFIRM_SIGN_UP_NAME
    }
}

impl super::ToResponse for ConfirmSignUpRequest {
    type E = ConfirmSignUpError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &ConfirmSignUpRequest) -> bool {
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
        let request = ConfirmSignUpRequest {
            client_id: Some("client_id".to_string()),
            confirmation_code: Some("confirmation_code".to_string()),
            username: Some("username".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = ConfirmSignUpRequest {
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

        let error = ConfirmSignUpError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = ConfirmSignUpError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
