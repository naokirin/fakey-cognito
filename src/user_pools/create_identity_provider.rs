use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const CREATE_IDENTITY_PROVIDER_NAME: &str = "CreateIdentityProvider";
pub const CREATE_IDENTITY_PROVIDER_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.CreateIdentityProvider";

/// CreateIdentityProvider response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_CreateIdentityProvider.html#API_CreateIdentityProvider_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum CreateIdentityProviderError {
    InternalErrorException,
    InvalidLambdaResponseException,
    InvalidParameterException,
    InvalidSmsRoleAccessPolicyException,
    InvalidSmsRoleTrustRelationshipException,
    InvalidUserPoolConfigurationException,
    MFAMethodNotFoundException,
    NotAuthorizedException,
    PasswordResetRequiredException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UnexpectedLambdaException,
    UserLambdaValidationException,
    UserNotConfirmedException,
    UserNotFoundException,
}

impl super::ToStatusCode for CreateIdentityProviderError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            CreateIdentityProviderError::InvalidParameterException
            | CreateIdentityProviderError::InvalidLambdaResponseException
            | CreateIdentityProviderError::InvalidSmsRoleAccessPolicyException
            | CreateIdentityProviderError::InvalidSmsRoleTrustRelationshipException
            | CreateIdentityProviderError::InvalidUserPoolConfigurationException
            | CreateIdentityProviderError::MFAMethodNotFoundException
            | CreateIdentityProviderError::NotAuthorizedException
            | CreateIdentityProviderError::PasswordResetRequiredException
            | CreateIdentityProviderError::ResourceNotFoundException
            | CreateIdentityProviderError::TooManyRequestsException
            | CreateIdentityProviderError::UnexpectedLambdaException
            | CreateIdentityProviderError::UserLambdaValidationException
            | CreateIdentityProviderError::UserNotConfirmedException
            | CreateIdentityProviderError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct CreateIdentityProviderRequest {
    pub attribute_mapping: Option<std::collections::HashMap<String, String>>,
    pub idp_identifiers: Option<Vec<String>>,
    pub provider_details: Option<std::collections::HashMap<String, String>>,
    pub provider_name: Option<String>,
    pub provider_type: Option<String>,
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
        super::to_json_response(self, CREATE_IDENTITY_PROVIDER_NAME, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &CreateIdentityProviderRequest) -> bool {
    !common::is_blank(&request.provider_name)
        && !common::is_blank(&request.provider_type)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = CreateIdentityProviderRequest {
            provider_name: Some("provider_name".to_string()),
            provider_type: Some("provider_type".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = CreateIdentityProviderRequest {
            provider_name: Some("provider_name".to_string()),
            provider_type: Some("provider_type".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
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
