use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADD_CUSTOM_ATTRIBUTES_NAME: &str = "AddCustomAttributes";
pub const ADD_CUSTOM_ATTRIBUTES_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AddCustomAttributes";

/// AddCustomAttributes response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AddCustomAttributes.html#API_AddCustomAttributes_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum AddCustomAttributesError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserImportInProgressException,
}

impl super::ToStatusCode for AddCustomAttributesError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AddCustomAttributesError::InvalidParameterException
            | AddCustomAttributesError::NotAuthorizedException
            | AddCustomAttributesError::ResourceNotFoundException
            | AddCustomAttributesError::TooManyRequestsException
            | AddCustomAttributesError::UserImportInProgressException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AddCustomAttributesRequest {
    pub custom_attributes: Option<Vec<super::data_types::SchemaAttributeType>>,
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
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AddCustomAttributesRequest) -> bool {
    !common::is_none_or_empty_vec(&request.custom_attributes)
        && !common::is_blank(&request.user_pool_id)
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
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AddCustomAttributesRequest {
            custom_attributes: Some(vec![]),
            user_pool_id: Some("".to_string()),
        };
        assert!(!valid_request(&request));
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
