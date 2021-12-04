use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const CREATE_RESOURCE_SERVER_NAME: &str = "CreateResourceServer";
pub const CREATE_RESOURCE_SERVER_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.CreateResourceServer";

super::gen_response_err!(
    CreateResourceServerError,
    InvalidParameterException
    | LimitExceededException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CreateResourceServerRequest {
    pub identifier: Option<String>,
    pub name: Option<String>,
    pub scopes: Option<Vec<super::data_types::ResourceServerScopeType>>,
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
        super::to_json_response(self, CREATE_RESOURCE_SERVER_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &CreateResourceServerRequest) -> bool {
    !common::is_blank(&request.identifier)
        && !common::is_blank(&request.name)
        && !common::is_blank(&request.user_pool_id)
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
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = CreateResourceServerRequest {
            identifier: Some("identifier".to_string()),
            name: Some("name".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
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
