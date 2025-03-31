use crate::common::{NAME_REGEX, USER_POOL_ID_REGEX};
use crate::{http, validator::regex_in_array};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::{Validate, ValidationError};

use super::AdminAddUserToGroupError;

pub const ADMIN_DELETE_USER_ATTRIBUTES_NAME: &str = "AdminDeleteUserAttributes";
pub const ADMIN_DELETE_USER_ATTRIBUTES_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminDeleteUserAttributes";

fn validate_user_attribute_names_regex(value: &[String]) -> Result<(), ValidationError> {
    regex_in_array(value, &NAME_REGEX)
}

super::gen_response_err!(
    AdminDeleteUserAttributesError,
    InvalidParameterException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AdminDeleteUserAttributesRequest {
    #[validate(required)]
    #[validate(length(min = 1, max = 32))]
    #[validate(custom(function = validate_user_attribute_names_regex))]
    pub user_attribute_names: Option<Vec<String>>,
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    #[validate(regex(path = *NAME_REGEX))]
    pub username: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex(path = *USER_POOL_ID_REGEX))]
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminDeleteUserAttributesRequest {
    fn to_action_name() -> &'static str {
        ADMIN_DELETE_USER_ATTRIBUTES_NAME
    }
}

impl super::ToResponse for AdminDeleteUserAttributesRequest {
    type E = AdminAddUserToGroupError;
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
        let request = AdminDeleteUserAttributesRequest {
            user_attribute_names: Some(vec!["foo".to_string()]),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminDeleteUserAttributesRequest {
            user_attribute_names: Some(vec!["foo".to_string()]),
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminDeleteUserAttributesError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminDeleteUserAttributesError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
