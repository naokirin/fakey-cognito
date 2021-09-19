use crate::common;
use crate::http;
use crate::templates;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_SET_USER_SETTINGS_NAME: &str = "AdminSetUserSettings";
pub const ADMIN_SET_USER_SETTINGS_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminSetUserSettings";

/// AdminSetUserSettings response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminSetUserSettings.html#API_AdminSetUserSettings_Errors
#[derive(Display, EnumString)]
pub enum AdminSetUserSettingsError {
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

impl super::ToStatusCode for AdminSetUserSettingsError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminSetUserSettingsError::InvalidParameterException
            | AdminSetUserSettingsError::InvalidLambdaResponseException
            | AdminSetUserSettingsError::InvalidSmsRoleAccessPolicyException
            | AdminSetUserSettingsError::InvalidSmsRoleTrustRelationshipException
            | AdminSetUserSettingsError::InvalidUserPoolConfigurationException
            | AdminSetUserSettingsError::MFAMethodNotFoundException
            | AdminSetUserSettingsError::NotAuthorizedException
            | AdminSetUserSettingsError::PasswordResetRequiredException
            | AdminSetUserSettingsError::ResourceNotFoundException
            | AdminSetUserSettingsError::TooManyRequestsException
            | AdminSetUserSettingsError::UnexpectedLambdaException
            | AdminSetUserSettingsError::UserLambdaValidationException
            | AdminSetUserSettingsError::UserNotConfirmedException
            | AdminSetUserSettingsError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminSetUserSettingsRequest {
    #[serde(rename = "MFAOptions")]
    pub mfa_options: Option<Vec<super::data_types::MFAOptionType>>,
    username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminSetUserSettingsRequest {
    fn to_action_name() -> &'static str {
        ADMIN_SET_USER_SETTINGS_NAME
    }
}

impl super::ToResponse for AdminSetUserSettingsRequest {
    fn to_response(&self) -> super::Response {
        if let Some(response) =
            super::config_response::<AdminSetUserSettingsRequest, AdminSetUserSettingsError>()
        {
            return response;
        };
        if !valid_request(&self) {
            let error = super::ResponseError::<AdminSetUserSettingsError>::CommonError(
                super::CommonError::InvalidParameterValue,
            );
            return super::error_response(error);
        }

        let opt_json = templates::render_template(ADMIN_SET_USER_SETTINGS_NAME, &self);
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
fn valid_request(request: &AdminSetUserSettingsRequest) -> bool {
    request.mfa_options.is_some()
        && !common::is_blank(&request.username)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminSetUserSettingsRequest {
            mfa_options: Some(Default::default()),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminSetUserSettingsRequest {
            mfa_options: Some(Default::default()),
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminSetUserSettingsError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminSetUserSettingsError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
