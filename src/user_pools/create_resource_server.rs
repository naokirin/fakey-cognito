use crate::common::USER_POOL_ID_REGEX;
use crate::http;
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

pub const CREATE_RESOURCE_SERVER_NAME: &str = "CreateResourceServer";
pub const CREATE_RESOURCE_SERVER_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.CreateResourceServer";
static IDENTIFIER_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\x21\x23-\x5B\x5D-\x7E]+").unwrap());
static NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\w\s+=,.@-]+").unwrap());

super::gen_response_err!(
    CreateResourceServerError,
    InvalidParameterException
    | LimitExceededException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct CreateResourceServerRequest {
    #[validate(required)]
    #[validate(length(min = 1, max = 256))]
    #[validate(regex = "IDENTIFIER_REGEX")]
    pub identifier: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 256))]
    #[validate(regex = "NAME_REGEX")]
    pub name: Option<String>,
    #[validate(length(max = 100))]
    pub scopes: Option<Vec<super::data_types::ResourceServerScopeType>>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex = "USER_POOL_ID_REGEX")]
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for CreateResourceServerRequest {
    fn to_action_name() -> &'static str {
        CREATE_RESOURCE_SERVER_NAME
    }
}

impl super::ToResponse for CreateResourceServerRequest {
    type E = CreateResourceServerError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, CREATE_RESOURCE_SERVER_NAME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = CreateResourceServerRequest {
            identifier: Some("identifier".to_string()),
            name: Some("name".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = CreateResourceServerRequest {
            identifier: Some("identifier".to_string()),
            name: Some("name".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = CreateResourceServerError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = CreateResourceServerError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
