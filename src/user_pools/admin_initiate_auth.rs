use crate::common;
use crate::http;
use crate::templates;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_INITIATE_AUTH_NAME: &str = "AdminInitiateAuth";
pub const ADMIN_INITIATE_AUTH_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminInitiateAuth";

/// AdminInitiateAuth response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminInitiateAuth.html#API_AdminInitiateAuth_Errors
#[derive(Display, EnumString)]
pub enum AdminInitiateAuthError {
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

impl super::ToStatusCode for AdminInitiateAuthError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminInitiateAuthError::InvalidParameterException
            | AdminInitiateAuthError::InvalidLambdaResponseException
            | AdminInitiateAuthError::InvalidSmsRoleAccessPolicyException
            | AdminInitiateAuthError::InvalidSmsRoleTrustRelationshipException
            | AdminInitiateAuthError::InvalidUserPoolConfigurationException
            | AdminInitiateAuthError::MFAMethodNotFoundException
            | AdminInitiateAuthError::NotAuthorizedException
            | AdminInitiateAuthError::PasswordResetRequiredException
            | AdminInitiateAuthError::ResourceNotFoundException
            | AdminInitiateAuthError::TooManyRequestsException
            | AdminInitiateAuthError::UnexpectedLambdaException
            | AdminInitiateAuthError::UserLambdaValidationException
            | AdminInitiateAuthError::UserNotConfirmedException
            | AdminInitiateAuthError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminInitiateAuthRequest {
    pub analytics_metadata: Option<super::data_types::AnalyticsMetadataType>,
    pub auth_flow: Option<String>,
    pub auth_parameters: Option<std::collections::HashMap<String, String>>,
    pub client_id: Option<String>,
    pub client_metadata: Option<std::collections::HashMap<String, String>>,
    pub context_data: Option<super::data_types::ContextDataType>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminInitiateAuthRequest {
    fn to_action_name() -> &'static str {
        ADMIN_INITIATE_AUTH_NAME
    }
}

impl super::ToResponse for AdminInitiateAuthRequest {
    fn to_response(&self) -> super::Response {
        if let Some(response) =
            super::config_response::<AdminInitiateAuthRequest, AdminInitiateAuthError>()
        {
            return response;
        };
        if !valid_request(&self) {
            let error = super::ResponseError::<AdminInitiateAuthError>::CommonError(
                super::CommonError::InvalidParameterValue,
            );
            return super::error_response(error);
        }

        let opt_json = templates::render_template(ADMIN_INITIATE_AUTH_NAME, &self);
        match opt_json {
            Some(json) => warp::http::Response::builder()
                .status(http::status_code(200))
                .body(super::responses::json_body(&json))
                .unwrap(),
            _ => super::error_response(super::CommonError::InternalFailure),
        }
    }
}

/// Validates request.
fn valid_request(request: &AdminInitiateAuthRequest) -> bool {
    !common::is_blank(&request.auth_flow)
        && !common::is_blank(&request.client_id)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminInitiateAuthRequest {
            auth_flow: Some("auth_flow".to_string()),
            client_id: Some("client_id".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminInitiateAuthRequest {
            auth_flow: Some("auth_flow".to_string()),
            client_id: Some("client_id".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminInitiateAuthError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminInitiateAuthError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
