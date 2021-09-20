use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_LIST_DEVICES_NAME: &str = "AdminListDevices";
pub const ADMIN_LIST_DEVICES_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminListDevices";

/// AdminListDevices response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminListDevices.html#API_AdminListDevices_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum AdminListDevicesError {
    InternalErrorException,
    InvalidParameterException,
    InvalidUserPoolConfigurationException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
}

impl super::ToStatusCode for AdminListDevicesError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminListDevicesError::InvalidParameterException
            | AdminListDevicesError::InvalidUserPoolConfigurationException
            | AdminListDevicesError::NotAuthorizedException
            | AdminListDevicesError::ResourceNotFoundException
            | AdminListDevicesError::TooManyRequestsException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminListDevicesRequest {
    limit: Option<u8>,
    pagination_token: Option<String>,
    username: Option<String>,
    user_pool_id: Option<String>,
}

impl super::ToActionName for AdminListDevicesRequest {
    fn to_action_name() -> &'static str {
        ADMIN_LIST_DEVICES_NAME
    }
}

impl super::ToResponse for AdminListDevicesRequest {
    type E = AdminListDevicesError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, ADMIN_LIST_DEVICES_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminListDevicesRequest) -> bool {
    !common::is_blank(&request.username) && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminListDevicesRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminListDevicesRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminListDevicesError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminListDevicesError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
