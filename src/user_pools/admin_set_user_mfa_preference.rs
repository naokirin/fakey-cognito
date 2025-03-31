use crate::common::{NAME_REGEX, USER_POOL_ID_REGEX};
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::Validate;

pub const ADMIN_SET_USER_MFA_PREFERENCE_NAME: &str = "AdminSetUserMFAPreference";
pub const ADMIN_SET_USER_MFA_PREFERENCE_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminSetUserMFAPreference";

super::gen_response_err!(
    AdminSetUserMFAPreferenceError,
    InvalidParameterException
    | NotAuthorizedException
    | PasswordResetRequiredException
    | ResourceNotFoundException
    | UserNotConfirmedException
    | UserNotFoundException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AdminSetUserMFAPreferenceRequest {
    #[serde(rename = "SMSMfaSettings")]
    pub sms_mfa_settings: Option<super::data_types::SMSMfaSettingsType>,
    pub software_token_mfa_settings: Option<super::data_types::SoftwareTokenMfaSettingsType>,
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    #[validate(regex(path = *NAME_REGEX))]
    pub username: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 55))]
    #[validate(regex(path = *USER_POOL_ID_REGEX))]
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
        super::to_empty_response(self)
    }
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
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminSetUserMFAPreferenceRequest {
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
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
