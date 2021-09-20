use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_UPDATE_DEVICE_STATUS_NAME: &str = "AdminUpdateDeviceStatus";
pub const ADMIN_UPDATE_DEVICE_STATUS_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminUpdateDeviceStatus";

/// AdminUpdateDeviceStatus response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminUpdateDeviceStatus.html#API_AdminUpdateDeviceStatus_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum AdminUpdateDeviceStatusError {
    InternalErrorException,
    InvalidParameterException,
    InvalidUserPoolConfigurationException,
    MFAMethodNotFoundException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminUpdateDeviceStatusError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminUpdateDeviceStatusError::InvalidParameterException
            | AdminUpdateDeviceStatusError::InvalidUserPoolConfigurationException
            | AdminUpdateDeviceStatusError::MFAMethodNotFoundException
            | AdminUpdateDeviceStatusError::NotAuthorizedException
            | AdminUpdateDeviceStatusError::ResourceNotFoundException
            | AdminUpdateDeviceStatusError::TooManyRequestsException
            | AdminUpdateDeviceStatusError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminUpdateDeviceStatusRequest {
    pub device_key: Option<String>,
    pub device_remembered_status: Option<String>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminUpdateDeviceStatusRequest {
    fn to_action_name() -> &'static str {
        ADMIN_UPDATE_DEVICE_STATUS_NAME
    }
}

impl super::ToResponse for AdminUpdateDeviceStatusRequest {
    type E = AdminUpdateDeviceStatusError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminUpdateDeviceStatusRequest) -> bool {
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
        let request = AdminUpdateDeviceStatusRequest {
            device_key: Some("device_key".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminUpdateDeviceStatusRequest {
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

        let error = AdminUpdateDeviceStatusError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminUpdateDeviceStatusError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
