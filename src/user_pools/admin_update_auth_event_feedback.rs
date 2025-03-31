use crate::common::{EVENT_ID_REGEX, NAME_REGEX, USER_POOL_ID_REGEX};
use crate::{http, validator::includes};
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use validator::{Validate, ValidationError};

pub const ADMIN_UPDATE_AUTH_EVENT_FEEDBACK_NAME: &str = "AdminUpdateAuthEventFeedback";
pub const ADMIN_UPDATE_AUTH_EVENT_FEEDBACK_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminUpdateAuthEventFeedback";

fn validate_feedback_value(value: &str) -> Result<(), ValidationError> {
    includes(value, vec!["Valid", "Invalid"])
}

super::gen_response_err!(
    AdminUpdateAuthEventFeedbackError,
    InvalidParameterException
    | NotAuthorizedException
    | ResourceNotFoundException
    | TooManyRequestsException
    | UserNotFoundException
    | UserPoolAddOnNotEnabledException => http::status_code(400),
    InternalErrorException => http::status_code(500)
);

#[derive(Serialize, Deserialize, Debug, Default, Validate)]
#[serde(rename_all = "PascalCase")]
pub struct AdminUpdateAuthEventFeedbackRequest {
    #[validate(required)]
    #[validate(length(min = 1, max = 50))]
    #[validate(regex(path = *EVENT_ID_REGEX))]
    pub event_id: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1))]
    #[validate(custom(function = validate_feedback_value))]
    pub feedback_value: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1, max = 128))]
    #[validate(regex(path = *NAME_REGEX))]
    pub username: Option<String>,
    #[validate(required)]
    #[validate(length(min = 1))]
    #[validate(regex(path = *USER_POOL_ID_REGEX))]
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminUpdateAuthEventFeedbackRequest {
    fn to_action_name() -> &'static str {
        ADMIN_UPDATE_AUTH_EVENT_FEEDBACK_NAME
    }
}

impl super::ToResponse for AdminUpdateAuthEventFeedbackRequest {
    type E = AdminUpdateAuthEventFeedbackError;
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
        let request = AdminUpdateAuthEventFeedbackRequest {
            event_id: Some("event_id".to_string()),
            feedback_value: Some("Valid".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_ok());
    }

    #[test]
    fn failure_to_valid_request() {
        let request = AdminUpdateAuthEventFeedbackRequest {
            event_id: Some("event_id".to_string()),
            feedback_value: Some("feedback_value".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("".to_string()),
            ..Default::default()
        };
        assert!(request.validate().is_err());
    }

    #[test]
    fn error_can_convert_to_status_code() {
        use crate::user_pools::ToStatusCode;

        let error = AdminUpdateAuthEventFeedbackError::InvalidParameterException;
        assert_eq!(http::status_code(400), error.to_status_code());

        let error = AdminUpdateAuthEventFeedbackError::InternalErrorException;
        assert_eq!(http::status_code(500), error.to_status_code());
    }
}
