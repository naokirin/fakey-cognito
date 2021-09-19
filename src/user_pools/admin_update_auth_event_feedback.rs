use crate::common;
use crate::http;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};

pub const ADMIN_UPDATE_AUTH_EVENT_FEEDBACK_NAME: &str = "AdminUpdateAuthEventFeedback";
pub const ADMIN_UPDATE_AUTH_EVENT_FEEDBACK_ACTION_NAME: &str =
    "AWSCognitoIdentityProviderService.AdminUpdateAuthEventFeedback";

/// AdminUpdateAuthEventFeedback response errors.
/// See https://docs.aws.amazon.com/cognito-user-identity-pools/latest/APIReference/API_AdminUpdateAuthEventFeedback.html#API_AdminUpdateAuthEventFeedback_Errors
#[derive(Display, EnumString)]
pub enum AdminUpdateAuthEventFeedbackError {
    InternalErrorException,
    InvalidParameterException,
    NotAuthorizedException,
    ResourceNotFoundException,
    TooManyRequestsException,
    UserNotFoundException,
    UserPoolAddOnNotEnabledException,
}

impl super::ToStatusCode for AdminUpdateAuthEventFeedbackError {
    fn to_status_code(&self) -> hyper::StatusCode {
        match self {
            AdminUpdateAuthEventFeedbackError::InvalidParameterException
            | AdminUpdateAuthEventFeedbackError::NotAuthorizedException
            | AdminUpdateAuthEventFeedbackError::ResourceNotFoundException
            | AdminUpdateAuthEventFeedbackError::TooManyRequestsException
            | AdminUpdateAuthEventFeedbackError::UserNotFoundException
            | AdminUpdateAuthEventFeedbackError::UserPoolAddOnNotEnabledException => {
                http::status_code(400)
            }
            _ => http::status_code(500),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(rename_all = "PascalCase")]
pub struct AdminUpdateAuthEventFeedbackRequest {
    pub event_id: Option<String>,
    pub feedback_value: Option<String>,
    pub username: Option<String>,
    pub user_pool_id: Option<String>,
}

impl super::ToActionName for AdminUpdateAuthEventFeedbackRequest {
    fn to_action_name() -> &'static str {
        ADMIN_UPDATE_AUTH_EVENT_FEEDBACK_NAME
    }
}

impl super::ToResponse for AdminUpdateAuthEventFeedbackRequest {
    fn to_response(&self) -> super::Response {
        if let Some(response) = super::config_response::<
            AdminUpdateAuthEventFeedbackRequest,
            AdminUpdateAuthEventFeedbackError,
        >() {
            return response;
        };
        if !valid_request(&self) {
            let error = super::ResponseError::<AdminUpdateAuthEventFeedbackError>::CommonError(
                super::CommonError::InvalidParameterValue,
            );
            return super::error_response(error);
        }

        warp::http::Response::builder()
            .status(http::status_code(200))
            .body(super::responses::empty_body())
            .unwrap()
    }
}

/// Validates request.
fn valid_request(request: &AdminUpdateAuthEventFeedbackRequest) -> bool {
    !common::is_blank(&request.event_id)
        && !common::is_blank(&request.feedback_value)
        && !common::is_blank(&request.username)
        && !common::is_blank(&request.user_pool_id)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn success_to_valid_request() {
        let request = AdminUpdateAuthEventFeedbackRequest {
            event_id: Some("event_id".to_string()),
            feedback_value: Some("feedback_value".to_string()),
            username: Some("username".to_string()),
            user_pool_id: Some("user_pool_id".to_string()),
            ..Default::default()
        };
        assert!(valid_request(&request));
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
        assert!(!valid_request(&request));
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
