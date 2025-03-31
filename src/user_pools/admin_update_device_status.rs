use crate::common::{DEVICE_KEY_REGEX, NAME_REGEX, USER_POOL_ID_REGEX};
use crate::{http, validator::includes};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::{Validate, ValidationError};

pub const ADMIN_UPDATE_DEVICE_STATUS_NAME: &str = "AdminUpdateDeviceStatus";
pub const ADMIN_UPDATE_DEVICE_STATUS_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminUpdateDeviceStatus";

fn validate_device_remembered_status(value: &str) -> Result<(), ValidationError> {
    includes(value, vec!["remembered", "not_remembered"])
}

super::gen_response_err!(
    AdminUpdateDeviceStatusError,
    InvalidParameterException
    | InvalidUserPoolConfigurationException
    | MFAMethodNotFoundException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AdminUpdateDeviceStatusRequest {
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex(path = *DEVICE_KEY_REGEX))]
    pub device_key: Option<String>,
    #[validate(custom(function = validate_device_remembered_status))]
    pub device_remembered_status: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    #[validate(regex(path = *NAME_REGEX))]
    pub username: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex(path = *USER_POOL_ID_REGEX))]
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
        super::to_empty_response(self)
    }
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
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminUpdateDeviceStatusRequest {
            device_key: Some("device_key".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
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
