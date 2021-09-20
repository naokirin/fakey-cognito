use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_SET_USER_MFA_PREFERENCE_NAME: &str = "AdminSetUserMFAPreference";
pub const ADMIN_SET_USER_MFA_PREFERENCE_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminSetUserMFAPreference";

/// AdminSetUserMFAPreference response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminSetUserMFAPreference.html#API_AdminSetUserMFAPreference_Errors
#[allow(clippy::enum_variant_names)]
#[derive(Display, EnumString)]
pub enum AdminSetUserMFAPreferenceError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    PasswordResetRequiredException,
    ResourceNotFoundException,
    UserNotConfirmedException,
    UserNotFoundException,
}

impl super::ToStatusCode for AdminSetUserMFAPreferenceError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminSetUserMFAPreferenceError::InvalidParameterException
            | AdminSetUserMFAPreferenceError::NotAuthorizedException
            | AdminSetUserMFAPreferenceError::PasswordResetRequiredException
            | AdminSetUserMFAPreferenceError::ResourceNotFoundException
            | AdminSetUserMFAPreferenceError::UserNotConfirmedException
            | AdminSetUserMFAPreferenceError::UserNotFoundException => http::status_code(400),
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminSetUserMFAPreferenceRequest {
    #[serde(rename = "SMSMfaSettings")]
    pub sms_mfa_settings: Option<super::data_types::SMSMfaSettingsType>,

    pub software_token_mfa_settings: Option<super::data_types::SoftwareTokenMfaSettingsType>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminSetUserMFAPreferenceRequest {
    fn to_action_name() -> &'static str {
        ADMIN_SET_USER_MFA_PREFERENCE_NAME
    }
}

impl super::ToResponse for AdminSetUserMFAPreferenceRequest {
    type E = AdminSetUserMFAPreferenceError;
    fn to_response(&self) -> super::Response {
        super::to_empty_response(self, valid_request)
    }
}

/// Validates request.
fn valid_request(request: &AdminSetUserMFAPreferenceRequest) -> bool {
    !common::is_blank(&request.username) && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminSetUserMFAPreferenceRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminSetUserMFAPreferenceRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(!valid_request(&request));
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminSetUserMFAPreferenceError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminSetUserMFAPreferenceError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
