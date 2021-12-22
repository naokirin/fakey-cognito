use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const CREATE_USER_POOL_CLIENT_NAME: &str = "CreateUserPoolClient";
pub const CREATE_USER_POOL_CLIENT_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.CreateUserPoolClient";

super::gen_response_err!(
    CreateUserPoolClientError,
    InvalidOAuthFlowException
    | InvalidParameterException
    | LimitExceededException
    | NotAuthorizedException
    | ResourceNotFoundException
    | ScopeDoesNotExistException
    | TooManyRequestsException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserPoolClientRequest {
    access_token_validity: Option<i64>,
    #[serde(rename = "AllowedOAuthFlows")]
    allowed_oatuh_flows: Option<Vec<String>>,
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    allowed_oauth_flows_user_pool_client: Option<bool>,
    #[serde(rename = "AllowedOAuthScopes")]
    allowed_oauth_scopes: Option<Vec<String>>,
    analytics_configuration: Option<super::data_types::AnalyticsConfigurationType>,
    #[serde(rename = "CallbackURLs")]
    callback_urls: Option<Vec<String>>,
    client_name: Option<String>,
    #[serde(rename = "DefaultRedirectURI")]
    default_redirect_uri: Option<String>,
    enable_token_revocation: Option<bool>,
    explicit_auth_flows: Option<Vec<String>>,
    generate_secret: Option<bool>,
    id_token_validity: Option<i64>,
    #[serde(rename = "LogoutURLs")]
    logout_urls: Option<Vec<String>>,
    prevent_user_existence_errors: Option<String>,
    read_attributes: Option<Vec<String>>,
    refresh_token_validity: Option<i64>,
    supported_identity_providers: Option<Vec<String>>,
    token_validity_units: Option<super::data_types::TokenValidityUnitsType>,
    user_pool_id: Option<String>,
    write_attributes: Option<Vec<String>>,
}

impl super::ToActionName for CreateUserPoolClientRequest {
    fn to_action_name() -> &'static str {
        CREATE_USER_POOL_CLIENT_NAME
    }
}

impl super::ToResponse for CreateUserPoolClientRequest {
    type E = CreateUserPoolClientError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, CREATE_USER_POOL_CLIENT_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &CreateUserPoolClientRequest) -> bool {
    !common::is_blank(&request.client_name) && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = CreateUserPoolClientRequest {
            client_name: Some("client_name".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = CreateUserPoolClientRequest {
            client_name: Some("client_name".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = CreateUserPoolClientError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = CreateUserPoolClientError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
