use crate::common;
use crate::http;
use crate::templates;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_GET_DEVICE_NAME: &str = "AdminGetDevice";
pub const ADMIN_GET_DEVICE_ACTION_NAME: &str = "AWSCognitoIdentityProviderService.AdminGetDevice";

/// AdminGetDevice response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminGetDevice.html#API_AdminGetDevice_Errors
#[derive(Display, EnumString)]
pub enum AdminGetDeviceError {
    InternalErrorException,
    InvalidParameterException,
    InvalidUserPoolConfigurationException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
}

impl super::ToStatusCode for AdminGetDeviceError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminGetDeviceError::InvalidParameterException
            | AdminGetDeviceError::InvalidUserPoolConfigurationException
            | AdminGetDeviceError::NotAuthorizedException
            | AdminGetDeviceError::ResourceNotFoundException
            | AdminGetDeviceError::TooManyRequestsException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

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
    fn to_response(&self) -> super::Response {
        if let Some(response) =
            super::config_response::<AdminGetDeviceRequest, AdminGetDeviceError>()
        {
            return response;
        };
        if !valid_request(&self) {
            let error = super::ResponseError::<AdminGetDeviceError>::CommonError(
                super::CommonError::InvalidParameterValue,
            );
            return super::error_response(error);
        }

        let opt_json = templates::render_template(ADMIN_GET_DEVICE_NAME, &self);
        match opt_json {
            Some(json) => warp::http::Response::builder()
                .status(http::status_code(200))
                .body(super::responses::json_body(&json))
                .unwrap(),
            _ => super::error_response(super::CommonError::InternalFailure),
        }
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