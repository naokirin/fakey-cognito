use crate::common::{URL_REGEX, USER_POOL_ID_REGEX};
use crate::{
    http,
    validator::{includes, includes_in_array, regex_in_array},
};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::{Validate, ValidationError};

pub const CREATE_USER_POOL_CLIENT_NAME: &str = "CreateUserPoolClient";
pub const CREATE_USER_POOL_CLIENT_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.CreateUserPoolClient";

static OAUTH_SCOPE_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\x21\x23-\x5B\x5D-\x7E]+").unwrap());
static CLIENT_NAME_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\w\s+=,.@-]+").unwrap());
static PROVIDER_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[\p{L}\p{M}\p{S}\p{N}\p{P}]+").unwrap());

fn validate_allowed_oauth_flows(value: &[String]) -> Result<(), ValidationError> {
    includes_in_array(value, vec!["code", "implicit", "client_credentials"])
}

fn validate_allowed_oauth_scopes(value: &[String]) -> Result<(), ValidationError> {
    regex_in_array(value, &OAUTH_SCOPE_REGEX)
}

fn validate_urls(value: &[String]) -> Result<(), ValidationError> {
    regex_in_array(value, &URL_REGEX)
}

fn validate_explicit_oauth_flows(value: &[String]) -> Result<(), ValidationError> {
    includes_in_array(
        value,
        vec![
            "ADMIN_NO_SRP_AUTH",
            "CUSTOM_AUTH_FLOW_ONLY",
            "USER_PASSWORD_AUTH",
            "ALLOW_ADMIN_USER_PASSWORD_AUTH",
            "ALLOW_CUSTOM_AUTH",
            "ALLOW_USER_PASSWORD_AUTH",
            "ALLOW_USER_SRP_AUTH",
            "ALLOW_REFRESH_TOKEN_AUTH",
        ],
    )
}

fn validate_prevent_user_extence_errors(value: &str) -> Result<(), ValidationError> {
    includes(value, vec!["ENABLED", "LEGACY"])
}

fn validate_supported_identity_providers(value: &[String]) -> Result<(), ValidationError> {
    regex_in_array(value, &PROVIDER_NAME_REGEX)
}

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

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct CreateUserPoolClientRequest {
    #[validate(range(min = 1, max = 86400))]
    access_token_validity: Option<i64>,
    #[serde(rename = "AllowedOAuthFlows")]
    #[validate(length(min = 0, max = 3))]
    #[validate(custom(function = "validate_allowed_oauth_flows"))]
    allowed_oatuh_flows: Option<Vec<String>>,
    #[serde(rename = "AllowedOAuthFlowsUserPoolClient")]
    allowed_oauth_flows_user_pool_client: Option<bool>,
    #[serde(rename = "AllowedOAuthScopes")]
    #[validate(length(max = 50))]
    #[validate(custom(function = "validate_allowed_oauth_scopes"))]
    allowed_oauth_scopes: Option<Vec<String>>,
    analytics_configuration: Option<super::data_types::AnalyticsConfigurationType>,
    #[serde(rename = "CallbackURLs")]
    #[validate(length(min = 0, max = 100))]
    #[validate(custom(function = "validate_urls"))]
    callback_urls: Option<Vec<String>>,
    #[validate(required)]
    #[validate(length(min = 1))]
    #[validate(regex = "CLIENT_NAME_REGEX")]
    client_name: Option<String>,
    #[serde(rename = "DefaultRedirectURI")]
    #[validate(length(min = 1, max = 1024))]
    #[validate(regex = "URL_REGEX")]
    default_redirect_uri: Option<String>,
    enable_token_revocation: Option<bool>,
    #[validate(custom(function = "validate_explicit_oauth_flows"))]
    explicit_auth_flows: Option<Vec<String>>,
    generate_secret: Option<bool>,
    #[validate(range(min = 1, max = 86400))]
    id_token_validity: Option<i64>,
    #[serde(rename = "LogoutURLs")]
    #[validate(length(min = 1, max = 100))]
    #[validate(custom(function = "validate_urls"))]
    logout_urls: Option<Vec<String>>,
    #[validate(custom(function = "validate_prevent_user_extence_errors"))]
    prevent_user_existence_errors: Option<String>,
    #[validate(length(min = 1, max = 2048))]
    read_attributes: Option<Vec<String>>,
    #[validate(range(min = 0, max = 315360000))]
    refresh_token_validity: Option<i64>,
    #[validate(length(min = 1, max = 32))]
    #[validate(custom(function = "validate_supported_identity_providers"))]
    supported_identity_providers: Option<Vec<String>>,
    token_validity_units: Option<super::data_types::TokenValidityUnitsType>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex = "USER_POOL_ID_REGEX")]
    user_pool_id: Option<String>,
    #[validate(length(min = 1, max = 2048))]
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
        super::to_json_response(self, CREATE_USER_POOL_CLIENT_NAME)
    }
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
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = CreateUserPoolClientRequest {
            client_name: Some("client_name".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
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
