use crate::common::{NAME_REGEX, USER_POOL_ID_REGEX};
use crate::http;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

pub const ADMIN_LIST_DEVICES_NAME: &str = "AdminListDevices";
pub const ADMIN_LIST_DEVICES_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminListDevices";

static PAGINATION_TOKEN_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\S]+").unwrap());

super::gen_response_err!(
    AdminListDevicesError,
    InvalidParameterException
    | InvalidUserPoolConfigurationException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AdminListDevicesRequest {
    #[validate(range(min = 0, max = 60))]
    limit: Option<u8>,
    #[validate(length(min = 1))]
    #[validate(regex(path = *PAGINATION_TOKEN_REGEX))]
    pagination_token: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    #[validate(regex(path = *NAME_REGEX))]
    username: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex(path = *USER_POOL_ID_REGEX))]
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
        super::to_json_response(self, ADMIN_LIST_DEVICES_NAME)
    }
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
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminListDevicesRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
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
