use crate::common::USER_POOL_ID_REGEX;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

pub const ADD_CUSTOM_ATTRIBUTES_NAME: &str = "AddCustomAttributes";
pub const ADD_CUSTOM_ATTRIBUTES_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AddCustomAttributes";

super::gen_response_err!(
    AddCustomAttributesError,
    InvalidParameterException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UserImportInProgressException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AddCustomAttributesRequest {
    #[validate(nested)]
    #[validate(required)]
    #[validate(length(min = 1, max = 25))]
    pub custom_attributes: Option<Vec<super::data_types::SchemaAttributeType>>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex(path = *USER_POOL_ID_REGEX))]
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AddCustomAttributesRequest {
    fn to_action_name() -> &'static str {
        ADD_CUSTOM_ATTRIBUTES_NAME
    }
}

impl super::ToResponse for AddCustomAttributesRequest {
    type E = AddCustomAttributesError;
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
        let request = AddCustomAttributesRequest {
            custom_attributes: Some(vec![Default::default()]),
            user_pool_id: Some("user_pool_id".to_string()),
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AddCustomAttributesRequest {
            custom_attributes: Some(vec![]),
            user_pool_id: Some("".to_string()),
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AddCustomAttributesError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AddCustomAttributesError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
