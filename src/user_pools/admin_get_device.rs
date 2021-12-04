use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_GET_DEVICE_NAME: &str = "AdminGetDevice";
pub const ADMIN_GET_DEVICE_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.AdminGetDevice";

super::gen_response_err!(
    AdminGetDeviceError,
    InvalidParameterException
    | InvalidUserPoolConfigurationException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminGetDeviceRequest {
    pub device_key: Option<String>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminGetDeviceRequest {
    fn to_action_name() -> &'static str {
        ADMIN_GET_DEVICE_NAME
    }
}

impl super::ToResponse for AdminGetDeviceRequest {
    type E = AdminGetDeviceError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, ADMIN_GET_DEVICE_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminGetDeviceRequest) -> bool {
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
        let request = AdminGetDeviceRequest {
            device_key: Some("device_key".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminGetDeviceRequest {
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

        let error = AdminGetDeviceError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminGetDeviceError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
