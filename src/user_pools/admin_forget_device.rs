use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_FORGET_DEVICE_NAME: &str = "AdminForgetDevice";
pub const ADMIN_FORGET_DEVICE_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminForgetDevice";

/// AdminForgetDevice response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminForgetDevice.html#API_AdminForgetDevice_Errors
#[derive(Display, EnumString)]
pub enum AdminForgetDeviceError {
    InternalErrorException,
    InvalidParameterException,
    InvalidUserPoolConfigurationException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminForgetDeviceError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminForgetDeviceError::InvalidParameterException
            | AdminForgetDeviceError::InvalidUserPoolConfigurationException
            | AdminForgetDeviceError::NotAuthorizedException
            | AdminForgetDeviceError::ResourceNotFoundException
            | AdminForgetDeviceError::TooManyRequestsException
            | AdminForgetDeviceError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminForgetDeviceRequest {
    pub device_key: Option<String>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminForgetDeviceRequest {
    fn to_action_name() -> &'static str {
        ADMIN_FORGET_DEVICE_NAME
    }
}

impl super::ToResponse for AdminForgetDeviceRequest {
    fn to_response(&self) -> super::Response {
        if let Some(response) =
            super::config_response::<AdminForgetDeviceRequest, AdminForgetDeviceError>()
        {
            return response;
        };
        if !valid_request(&self) {
            let error = super::ResponseError::<AdminForgetDeviceError>::CommonError(
                super::CommonError::InvalidParameterValue,
            );
            return super::error_response(error);
        }

        warp::http::Response::builder()
            .status(http::status_code(200))
            .body(super::responses::empty_body())
            .unwrap()
    }
}

/// Validates request.
fn valid_request(request: &AdminForgetDeviceRequest) -> bool {
    !common::is_blank(&request.device_key)
        && !common::is_blank(&request.username)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminForgetDeviceRequest {
            device_key: Some("device_key".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminForgetDeviceRequest {
            device_key: Some("device_key".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminForgetDeviceError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminForgetDeviceError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
