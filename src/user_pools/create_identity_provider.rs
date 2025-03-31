use crate::common::USER_POOL_ID_REGEX;
use crate::{
    http,
    validator::{includes, regex_in_array},
};
use once_cell::sync::Lazy;
use regex::Regex;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::{Validate, ValidationError};

pub const CREATE_IDENTITY_PROVIDER_NAME: &str = "CreateIdentityProvider";
pub const CREATE_IDENTITY_PROVIDER_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.CreateIdentityProvider";

static IDP_IDENTIFIER_REGEX: Lazy<Regex> = Lazy::new(|| Regex::new(r"[\w\s+=.@-]+").unwrap());
static PROVIDER_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"[^_][\p{L}\p{M}\p{S}\p{N}\p{P}][^_]+").unwrap());

fn validate_idp_identifiers(value: &[String]) -> Result<(), ValidationError> {
    regex_in_array(value, &IDP_IDENTIFIER_REGEX)
}
fn validate_provider_type(value: &str) -> Result<(), ValidationError> {
    includes(
        value,
        vec![
            "SAML",
            "Facebook",
            "Google",
            "LoginWithAmazon",
            "SignInWithApple",
            "OIDC",
        ],
    )
}

super::gen_response_err!(
    CreateIdentityProviderError,
    InvalidParameterException
    | InvalidLambdaResponseException
    | InvalidSmsRoleAccessPolicyException
    | InvalidSmsRoleTrustRelationshipException
    | InvalidUserPoolConfigurationException
    | MFAMethodNotFoundException
    | NotAuthorizedException
    | PasswordResetRequiredException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UnexpectedLambdaException
    | UserLambdaValidationException
    | UserNotConfirmedException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct CreateIdentityProviderRequest {
    pub attribute_mapping: Option<std::collections::HashMap<String, String>>,
    #[validate(length(min = 0, max = 50))]
    #[validate(custom(function = validate_idp_identifiers))]
    pub idp_identifiers: Option<Vec<String>>,
    pub provider_details: Option<std::collections::HashMap<String, String>>,
    #[validate(required)]
    #[validate(length(min = 1, max = 32))]
    #[validate(regex(path = *PROVIDER_NAME_REGEX))]
    pub provider_name: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1))]
    #[validate(custom(function = validate_provider_type))]
    pub provider_type: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex(path = *USER_POOL_ID_REGEX))]
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for CreateIdentityProviderRequest {
    fn to_action_name() -> &'static str {
        CREATE_IDENTITY_PROVIDER_NAME
    }
}

impl super::ToResponse for CreateIdentityProviderRequest {
    type E = CreateIdentityProviderError;
    fn to_response(&self) -> super::Response {
        super::to_json_response(self, CREATE_IDENTITY_PROVIDER_NAME)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = CreateIdentityProviderRequest {
            provider_name: Some("provider_name".to_string()),
            provider_type: Some("SAML".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = CreateIdentityProviderRequest {
            provider_name: Some("provider_name".to_string()),
            provider_type: Some("SAML".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = CreateIdentityProviderError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = CreateIdentityProviderError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
