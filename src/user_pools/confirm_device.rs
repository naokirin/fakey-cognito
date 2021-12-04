use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const CONFIRM_DEVICE_NAME: &str = "ConfirmDevice";
pub const CONFIRM_DEVICE_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.ConfirmDevice";

super::gen_response_err!(
    ConfirmDeviceError,
    InvalidParameterException
    | InvalidLambdaResponseException
    | InvalidPasswordException
    | InvalidUserPoolConfigurationException
    | NotAuthorizedException
    | PasswordResetRequiredException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UsernameExistsException
    | UserNotConfirmedException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ConfirmDeviceRequest {
    access_token: Option<String>,
    device_key: Option<String>,
    device_name: Option<String>,
    device_secret_verifier_config: Option<super::DeviceSecretVerifierConfigType>,
}

impl super::ToActionName for ConfirmDeviceRequest {
    fn to_action_name() -> &'static str {
        CONFIRM_DEVICE_NAME
    }
}

impl super::ToResponse for ConfirmDeviceRequest {
    type E = ConfirmDeviceError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, CONFIRM_DEVICE_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &ConfirmDeviceRequest) -> bool {
    !common::is_blank(&request.access_token) && !common::is_blank(&request.device_key)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = ConfirmDeviceRequest {
            access_token: Some("access_token".to_string()),
            device_key: Some("device_key".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = ConfirmDeviceRequest {
            access_token: Some("access_token".to_string()),
            device_key: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = ConfirmDeviceError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = ConfirmDeviceError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
